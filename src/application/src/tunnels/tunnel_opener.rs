use std::sync::Arc;

use async_trait::async_trait;
use sallygate_domain::credentials::{AuthMethod, Secret};
use sallygate_domain::inventory::SshEndpoint;
use sallygate_domain::tunnels::TunnelKind;
use thiserror::Error;

use crate::sessions::{JumpHost, OpenSessionError};
use crate::trust::KnownHosts;

/// Port: logs in to a host and forwards a local port through it.
#[async_trait]
pub trait TunnelOpener: Send + Sync {
    async fn open(
        &self,
        request: OpenTunnelRequest,
    ) -> Result<Arc<dyn LiveTunnel>, OpenTunnelError>;
}

/// Everything an opener needs for one rule.
pub struct OpenTunnelRequest {
    pub endpoint: SshEndpoint,
    pub auth: AuthMethod,
    pub jump: Option<JumpHost>,
    pub secret: Option<Secret>,
    pub known_hosts: Arc<dyn KnownHosts>,
    pub kind: TunnelKind,
}

/// Port: a rule that is forwarding right now.
#[async_trait]
pub trait LiveTunnel: Send + Sync {
    /// Stops listening and closes the SSH connection.
    async fn close(&self);
}

/// Why a rule could not start. Messages are safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum OpenTunnelError {
    #[error(transparent)]
    Connect(#[from] OpenSessionError),
    #[error("port {0} on this computer is already in use")]
    PortInUse(u16),
}
