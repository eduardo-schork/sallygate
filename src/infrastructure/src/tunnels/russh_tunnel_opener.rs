use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use russh::{Disconnect, client};
use sallygate_application::sessions::OpenSessionError;
use sallygate_application::tunnels::{
    LiveTunnel, OpenTunnelError, OpenTunnelRequest, TunnelOpener,
};
use sallygate_domain::tunnels::TunnelKind;
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

use super::socks5;
use crate::sessions::{LogIn, ServerKeyCheck, client_config, log_in, protocol_error};

/// Tunnels listen on this computer only, never on the network.
const LISTEN_ADDRESS: &str = "127.0.0.1";

/// Opens forwarding rules with `russh`: one SSH connection per rule.
///
/// shortcut: a rule whose connection drops (server restart, network change) still shows as on
/// until the user turns it off and on; add a keepalive watcher when that bites.
pub struct RusshTunnelOpener {
    config: Arc<client::Config>,
}

impl RusshTunnelOpener {
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: client_config(),
        }
    }
}

impl Default for RusshTunnelOpener {
    fn default() -> Self {
        Self::new()
    }
}

type Connection = Arc<client::Handle<ServerKeyCheck>>;

struct RusshLiveTunnel {
    connection: Connection,
    accepting: JoinHandle<()>,
}

#[async_trait]
impl LiveTunnel for RusshLiveTunnel {
    async fn close(&self) {
        self.accepting.abort();
        // The server may already be gone; closing is best effort.
        let _ = self
            .connection
            .disconnect(Disconnect::ByApplication, "", "en")
            .await;
    }
}

#[async_trait]
impl TunnelOpener for RusshTunnelOpener {
    async fn open(
        &self,
        request: OpenTunnelRequest,
    ) -> Result<Arc<dyn LiveTunnel>, OpenTunnelError> {
        let port = request.kind.local_port().value();
        // Bound before logging in, so a busy port fails at once and asks for no password.
        let listener =
            TcpListener::bind((LISTEN_ADDRESS, port))
                .await
                .map_err(|error| match error.kind() {
                    std::io::ErrorKind::AddrInUse => OpenTunnelError::PortInUse(port),
                    _ => OpenTunnelError::Connect(OpenSessionError::Protocol(error.to_string())),
                })?;
        let connection = Arc::new(
            log_in(
                Arc::clone(&self.config),
                LogIn {
                    endpoint: &request.endpoint,
                    auth: &request.auth,
                    secret: request.secret.as_ref(),
                    known_hosts: request.known_hosts,
                },
            )
            .await?,
        );
        let accepting = tokio::spawn(accept(listener, Arc::clone(&connection), request.kind));
        Ok(Arc::new(RusshLiveTunnel {
            connection,
            accepting,
        }))
    }
}

async fn accept(listener: TcpListener, connection: Connection, kind: TunnelKind) {
    while let Ok((client, peer)) = listener.accept().await {
        tokio::spawn(forward(client, peer, Arc::clone(&connection), kind.clone()));
    }
}

/// Carries one local connection through the server until either side closes it.
async fn forward(
    mut client: TcpStream,
    peer: SocketAddr,
    connection: Connection,
    kind: TunnelKind,
) {
    let (target, is_socks) = match &kind {
        TunnelKind::Local {
            target_address,
            target_port,
            ..
        } => ((target_address.to_string(), target_port.value()), false),
        TunnelKind::Dynamic { .. } => match socks5::read_request(&mut client).await {
            Ok(target) => (target, true),
            Err(_) => return,
        },
    };
    let channel = connection
        .channel_open_direct_tcpip(
            target.0,
            u32::from(target.1),
            peer.ip().to_string(),
            u32::from(peer.port()),
        )
        .await
        .map_err(|error| protocol_error(&error));
    if is_socks && socks5::reply(&mut client, channel.is_ok()).await.is_err() {
        return;
    }
    let Ok(channel) = channel else {
        return;
    };
    let mut remote = channel.into_stream();
    // Either side closing ends the copy; the error says only who hung up first.
    let _ = tokio::io::copy_bidirectional(&mut client, &mut remote).await;
}
