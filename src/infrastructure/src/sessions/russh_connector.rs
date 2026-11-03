use std::sync::{Arc, PoisonError};
use std::time::Duration;

use async_trait::async_trait;
use russh::client;
use russh::keys::PrivateKeyWithHashAlg;
use sallygate_application::sessions::{
    ConnectRequest, JumpHost, LiveSession, OpenSessionError, SshConnector,
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
    pub jump: Option<&'a JumpHost>,
}

/// A logged-in connection, and the jump host connection it runs through, if any. Both must
/// live as long as the session or tunnel does.
pub(crate) struct Connected {
    pub connection: client::Handle<ServerKeyCheck>,
    pub jump: Option<client::Handle<ServerKeyCheck>>,
}

/// Connects (through the jump host, if any), checks the server key with `known_hosts` and
/// authenticates. Shared by terminal sessions and tunnels.
pub(crate) async fn log_in(
    config: Arc<client::Config>,
    request: LogIn<'_>,
) -> Result<Connected, OpenSessionError> {
    let jump = match request.jump {
        None => None,
        Some(jump) => Some(log_in_to_jump_host(Arc::clone(&config), jump, &request).await?),
    };
    let (server_key_check, rejection) =
        ServerKeyCheck::new(request.endpoint.clone(), Arc::clone(&request.known_hosts));
    let address = request.endpoint.address().as_str();
    let port = request.endpoint.port().value();
    let handshake = match &jump {
        None => client::connect(config, (address, port), server_key_check).await,
        Some(jump) => {
            let channel = jump
                .channel_open_direct_tcpip(address, u32::from(port), "127.0.0.1", 0)
                .await
                .map_err(|error| OpenSessionError::JumpHost(error.to_string()))?;
            client::connect_stream(config, channel.into_stream(), server_key_check).await
        }
    };
    let connection = match handshake {
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
    let connection =
        authenticate(connection, request.endpoint, request.auth, request.secret).await?;
    Ok(Connected { connection, jump })
}

/// The jump host must already be trusted and log in with a key that needs no passphrase: one
/// connection asks the user one thing at a time.
async fn log_in_to_jump_host(
    config: Arc<client::Config>,
    jump: &JumpHost,
    request: &LogIn<'_>,
) -> Result<client::Handle<ServerKeyCheck>, OpenSessionError> {
    let shown = jump.endpoint.to_string();
    let connected = Box::pin(log_in(
        config,
        LogIn {
            endpoint: &jump.endpoint,
            auth: &jump.auth,
            secret: None,
            known_hosts: Arc::clone(&request.known_hosts),
            jump: None,
        },
    ))
    .await
    .map_err(|error| {
        OpenSessionError::JumpHost(match error {
            OpenSessionError::UnknownHostKey(_) | OpenSessionError::HostKeyChanged(_) => format!(
                "connect to {shown} on its own first, to check and trust its server key"
            ),
            OpenSessionError::SecretRequired => format!(
                "{shown} needs a password or passphrase; a jump host must use a key without one for now"
            ),
            other => format!("{shown}: {other}"),
        })
    })?;
    Ok(connected.connection)
}

async fn authenticate(
    mut connection: client::Handle<ServerKeyCheck>,
    endpoint: &SshEndpoint,
    auth: &AuthMethod,
    secret: Option<&Secret>,
) -> Result<client::Handle<ServerKeyCheck>, OpenSessionError> {
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
            jump,
            secret,
            size,
            known_hosts,
            output,
        } = request;
        let Connected { connection, jump } = log_in(
            Arc::clone(&self.config),
            LogIn {
                endpoint: &endpoint,
                auth: &auth,
                secret: secret.as_ref(),
                known_hosts,
                jump: jump.as_ref(),
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
            connection, jump, reader, writer, output,
        )))
    }
}
