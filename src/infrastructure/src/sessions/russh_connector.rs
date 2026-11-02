use std::sync::{Arc, PoisonError};
use std::time::Duration;

use async_trait::async_trait;
use russh::client;
use russh::keys::PrivateKeyWithHashAlg;
use sallygate_application::sessions::{
    ConnectRequest, LiveSession, OpenSessionError, SshConnector,
};
use sallygate_domain::credentials::AuthMethod;

use super::key_file::load_private_key;
use super::russh_live_session::RusshLiveSession;
use super::server_key_check::ServerKeyCheck;

/// The terminal type announced to servers; `xterm.js` understands it.
const TERMINAL_TYPE: &str = "xterm-256color";
/// How often to ping an idle server so routers keep the connection open.
const KEEPALIVE_INTERVAL: Duration = Duration::from_secs(30);

/// Opens interactive sessions with the `russh` library.
pub struct RusshConnector {
    config: Arc<client::Config>,
}

impl RusshConnector {
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: Arc::new(client::Config {
                keepalive_interval: Some(KEEPALIVE_INTERVAL),
                ..client::Config::default()
            }),
        }
    }
}

impl Default for RusshConnector {
    fn default() -> Self {
        Self::new()
    }
}

fn protocol_error(error: &russh::Error) -> OpenSessionError {
    OpenSessionError::Protocol(error.to_string())
}

#[async_trait]
impl SshConnector for RusshConnector {
    async fn connect(
        &self,
        request: ConnectRequest,
    ) -> Result<Arc<dyn LiveSession>, OpenSessionError> {
        let ConnectRequest {
            endpoint,
            auth,
            size,
            known_hosts,
            output,
        } = request;
        let AuthMethod::KeyFile(key_file_path) = auth;
        let private_key = load_private_key(&key_file_path)?;

        let (server_key_check, rejection) = ServerKeyCheck::new(endpoint.clone(), known_hosts);
        let server = (endpoint.address().as_str(), endpoint.port().value());
        let mut connection =
            match client::connect(Arc::clone(&self.config), server, server_key_check).await {
                Ok(connection) => connection,
                Err(error) => {
                    // A refused server key also ends the handshake with an error; report the real reason.
                    let rejection = rejection
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .take();
                    return Err(rejection.unwrap_or_else(|| match error {
                        russh::Error::IO(io_error) => {
                            OpenSessionError::Unreachable(io_error.to_string())
                        }
                        other => protocol_error(&other),
                    }));
                }
            };

        let hash_algorithm = connection
            .best_supported_rsa_hash()
            .await
            .map_err(|error| protocol_error(&error))?
            .flatten();
        let authentication = connection
            .authenticate_publickey(
                endpoint.username().as_str(),
                PrivateKeyWithHashAlg::new(Arc::new(private_key), hash_algorithm),
            )
            .await
            .map_err(|error| protocol_error(&error))?;
        if !authentication.success() {
            return Err(OpenSessionError::AuthenticationRejected);
        }

        let channel = connection
            .channel_open_session()
            .await
            .map_err(|error| protocol_error(&error))?;
        channel
            .request_pty(
                false,
                TERMINAL_TYPE,
                u32::from(size.columns()),
                u32::from(size.rows()),
                0,
                0,
                &[],
            )
            .await
            .map_err(|error| protocol_error(&error))?;
        channel
            .request_shell(false)
            .await
            .map_err(|error| protocol_error(&error))?;

        let (reader, writer) = channel.split();
        Ok(Arc::new(RusshLiveSession::start(
            connection, reader, writer, output,
        )))
    }
}
