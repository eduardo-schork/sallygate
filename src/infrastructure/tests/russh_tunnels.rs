//! Integration tests for port forwarding against the local SSH test server.
//! Start it first with `scripts/test-ssh-server.sh up`, then run `bun run test:integration`.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests that use them"
)]

use std::path::PathBuf;
use std::sync::Arc;

use sallygate_application::sessions::OpenSessionError;
use sallygate_application::trust::KnownHosts;
use sallygate_application::tunnels::{OpenTunnelError, OpenTunnelRequest, TunnelOpener};
use sallygate_domain::credentials::{AuthMethod, KeyFilePath};
use sallygate_domain::inventory::{HostAddress, Port, SshEndpoint, Username};
use sallygate_domain::tunnels::TunnelKind;
use sallygate_infrastructure::trust::KnownHostsFile;
use sallygate_infrastructure::tunnels::RusshTunnelOpener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

fn endpoint() -> SshEndpoint {
    SshEndpoint::new(
        HostAddress::parse("127.0.0.1").unwrap(),
        Port::try_from(2222).unwrap(),
        Username::parse("sallygate").unwrap(),
    )
}

fn key_file() -> AuthMethod {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/ssh-server/.keys/id_ed25519");
    AuthMethod::KeyFile(KeyFilePath::parse(path.to_str().unwrap()).unwrap())
}

fn request(known_hosts: Arc<KnownHostsFile>, kind: TunnelKind) -> OpenTunnelRequest {
    OpenTunnelRequest {
        endpoint: endpoint(),
        auth: key_file(),
        jump: None,
        secret: None,
        known_hosts,
        kind,
    }
}

/// A known hosts file that already trusts the test server, learned on a first refused try.
async fn trusting_known_hosts(folder: &tempfile::TempDir, spare_port: u16) -> Arc<KnownHostsFile> {
    let known_hosts = Arc::new(KnownHostsFile::new(folder.path().join("known_hosts")));
    let first = RusshTunnelOpener::new()
        .open(request(
            Arc::clone(&known_hosts),
            TunnelKind::Dynamic {
                local_port: Port::try_from(spare_port).unwrap(),
            },
        ))
        .await;
    let Err(OpenTunnelError::Connect(OpenSessionError::UnknownHostKey(host_key))) = first else {
        panic!("expected an unknown host key first");
    };
    known_hosts.trust(&endpoint(), &host_key).unwrap();
    known_hosts
}

async fn ssh_banner(stream: &mut TcpStream) -> String {
    let mut bytes = [0; 8];
    stream.read_exact(&mut bytes).await.unwrap();
    String::from_utf8_lossy(&bytes).into_owned()
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn forwards_a_local_port_to_a_service_on_the_server() {
    let folder = tempfile::tempdir().unwrap();
    let known_hosts = trusting_known_hosts(&folder, 18_032).await;
    let local_port = Port::try_from(18_022).unwrap();
    let tunnel = RusshTunnelOpener::new()
        .open(request(
            known_hosts,
            TunnelKind::Local {
                local_port,
                target_address: HostAddress::parse("127.0.0.1").unwrap(),
                target_port: Port::try_from(22).unwrap(),
            },
        ))
        .await
        .unwrap();

    let mut stream = TcpStream::connect(("127.0.0.1", 18_022)).await.unwrap();

    assert_eq!(ssh_banner(&mut stream).await, "SSH-2.0-");
    tunnel.close().await;
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn proxies_socks_connections_through_the_server() {
    let folder = tempfile::tempdir().unwrap();
    let known_hosts = trusting_known_hosts(&folder, 18_033).await;
    let tunnel = RusshTunnelOpener::new()
        .open(request(
            known_hosts,
            TunnelKind::Dynamic {
                local_port: Port::try_from(18_023).unwrap(),
            },
        ))
        .await
        .unwrap();

    let mut stream = TcpStream::connect(("127.0.0.1", 18_023)).await.unwrap();
    stream.write_all(&[5, 1, 0]).await.unwrap();
    let mut choice = [0; 2];
    stream.read_exact(&mut choice).await.unwrap();
    stream
        .write_all(&[
            5, 1, 0, 3, 9, b'l', b'o', b'c', b'a', b'l', b'h', b'o', b's', b't', 0, 22,
        ])
        .await
        .unwrap();
    let mut reply = [0; 10];
    stream.read_exact(&mut reply).await.unwrap();

    assert_eq!(choice, [5, 0]);
    assert_eq!(reply[1], 0, "the SOCKS connect should succeed");
    assert_eq!(ssh_banner(&mut stream).await, "SSH-2.0-");
    tunnel.close().await;
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn reports_a_local_port_that_is_taken() {
    let folder = tempfile::tempdir().unwrap();
    let known_hosts = Arc::new(KnownHostsFile::new(folder.path().join("known_hosts")));
    let _taken = tokio::net::TcpListener::bind(("127.0.0.1", 18_024))
        .await
        .unwrap();

    let result = RusshTunnelOpener::new()
        .open(request(
            known_hosts,
            TunnelKind::Dynamic {
                local_port: Port::try_from(18_024).unwrap(),
            },
        ))
        .await;

    assert!(matches!(result, Err(OpenTunnelError::PortInUse(18_024))));
}
