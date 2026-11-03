use std::sync::{Arc, PoisonError};
use std::time::Duration;

use async_trait::async_trait;
use russh::client;
use russh::keys::PrivateKeyWithHashAlg;
use sallygate_application::sessions::{
    ConnectRequest, LiveSession, OpenSessionError, SshConnector,
};
use sallygate_application::trust::KnownHosts;
use sallygate_domain::credentials::{AuthMethod, Secret};
use sallygate_domain::inventory::SshEndpoint;

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

/// The client settings every connection uses: keepalives so idle routers keep it open.
pub(crate) fn client_config() -> Arc<client::Config> {
    Arc::new(client::Config {
        keepalive_interval: Some(KEEPALIVE_INTERVAL),
        ..client::Config::default()
    })
}

impl RusshConnector {
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: client_config(),
        }
    }
}

impl Default for RusshConnector {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn protocol_error(error: &russh::Error) -> OpenSessionError {
    OpenSessionError::Protocol(error.to_string())
}

/// Who to log in as, and where.
pub(crate) struct LogIn<'a> {
    pub endpoint: &'a SshEndpoint,
    pub auth: &'a AuthMethod,
    pub secret: Option<&'a Secret>,
    pub known_hosts: Arc<dyn KnownHosts>,
}

/// Connects, checks the server key with `known_hosts` and authenticates. Shared by terminal
/// sessions and tunnels.
pub(crate) async fn log_in(
    config: Arc<client::Config>,
    request: LogIn<'_>,
) -> Result<client::Handle<ServerKeyCheck>, OpenSessionError> {
    let LogIn {
        endpoint,
        auth,
        secret,
        known_hosts,
    } = request;
    let (server_key_check, rejection) = ServerKeyCheck::new(endpoint.clone(), known_hosts);
    let server = (endpoint.address().as_str(), endpoint.port().value());
    let mut connection = match client::connect(config, server, server_key_check).await {
        Ok(connection) => connection,
        Err(error) => {
            // A refused server key also ends the handshake with an error; report the real reason.
            let rejection = rejection
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .take();
            return Err(rejection.unwrap_or_else(|| match error {
                russh::Error::IO(io_error) => OpenSessionError::Unreachable(io_error.to_string()),
                other => protocol_error(&other),
            }));
        }
    };

    // Checked after the server key, so a first connection asks about the key before the secret.
    let private_key = match auth {
        AuthMethod::KeyFile(key_file_path) => Some(load_private_key(key_file_path, secret)?),
        AuthMethod::Password => None,
    };
    let password = match (auth, secret) {
        (AuthMethod::Password, None) => return Err(OpenSessionError::SecretRequired),
        (AuthMethod::Password, Some(secret)) => Some(secret.expose()),
        (AuthMethod::KeyFile(_), _) => None,
    };

    let username = endpoint.username().as_str();
    let authentication = if let Some(private_key) = private_key {
        let hash_algorithm = connection
            .best_supported_rsa_hash()
            .await
            .map_err(|error| protocol_error(&error))?
            .flatten();
        connection
            .authenticate_publickey(
                username,
                PrivateKeyWithHashAlg::new(Arc::new(private_key), hash_algorithm),
            )
            .await
    } else {
        connection
            .authenticate_password(username, password.unwrap_or_default())
            .await
    }
    .map_err(|error| protocol_error(&error))?;
    if !authentication.success() {
        return Err(OpenSessionError::AuthenticationRejected);
    }
    Ok(connection)
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
            secret,
            size,
            known_hosts,
            output,
        } = request;
        let connection = log_in(
            Arc::clone(&self.config),
            LogIn {
                endpoint: &endpoint,
                auth: &auth,
                secret: secret.as_ref(),
                known_hosts,
            },
        )
        .await?;

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
