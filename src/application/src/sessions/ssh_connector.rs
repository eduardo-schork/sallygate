use std::sync::Arc;

use async_trait::async_trait;
use sallygate_domain::credentials::{AuthMethod, Secret};
use sallygate_domain::inventory::SshEndpoint;
use sallygate_domain::sessions::TerminalSize;
use sallygate_domain::trust::HostKey;
use thiserror::Error;

use super::{LiveSession, SessionOutput};
use crate::trust::{KnownHosts, KnownHostsError};

/// Port: opens interactive terminal sessions over SSH.
#[async_trait]
pub trait SshConnector: Send + Sync {
    /// Connects, verifies the server key with `known_hosts`, authenticates and
    /// opens a shell. Output starts flowing to `output` right away.
    async fn connect(
        &self,
        request: ConnectRequest,
    ) -> Result<Arc<dyn LiveSession>, OpenSessionError>;
}

/// Everything a connector needs to open one session.
pub struct ConnectRequest {
    pub endpoint: SshEndpoint,
    pub auth: AuthMethod,
    /// The password or key passphrase, when the user typed one.
    pub secret: Option<Secret>,
    pub size: TerminalSize,
    pub known_hosts: Arc<dyn KnownHosts>,
    pub output: Arc<dyn SessionOutput>,
}

/// Why a session could not be opened. Messages are safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OpenSessionError {
    #[error("the server is not trusted yet; check its fingerprint before trusting it")]
    UnknownHostKey(HostKey),
    #[error("the server key changed since you trusted it; this can be an attack")]
    HostKeyChanged(HostKey),
    #[error("a password or key passphrase is needed")]
    SecretRequired,
    #[error("the server rejected the login")]
    AuthenticationRejected,
    #[error("could not read the private key: {0}")]
    KeyFileUnreadable(String),
    #[error("could not reach the server: {0}")]
    Unreachable(String),
    #[error("the SSH connection failed: {0}")]
    Protocol(String),
    #[error(transparent)]
    KnownHosts(#[from] KnownHostsError),
}
