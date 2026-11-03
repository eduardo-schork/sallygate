//! Integration tests against the local SSH test server.
//! Start it first with `scripts/test-ssh-server.sh up`, then run `bun run test:integration`.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests that use them"
)]

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use sallygate_application::sessions::{
    ConnectRequest, LiveSession, OpenSessionError, SessionOutput, SshConnector,
};
use sallygate_application::trust::KnownHosts;
use sallygate_domain::credentials::{AuthMethod, KeyFilePath, Secret};
use sallygate_domain::inventory::{HostAddress, Port, SshEndpoint, Username};
use sallygate_domain::sessions::{SessionEnd, TerminalSize};
use sallygate_infrastructure::sessions::RusshConnector;
use sallygate_infrastructure::trust::KnownHostsFile;

const TEST_SERVER_PORT: u16 = 2222;
/// Set in tests/ssh-server/Dockerfile.
const TEST_PASSWORD: &str = "sallygate-test-password";

#[derive(Default)]
struct RecordingOutput {
    bytes: Mutex<Vec<u8>>,
    end: Mutex<Option<SessionEnd>>,
}

impl RecordingOutput {
    fn text(&self) -> String {
        String::from_utf8_lossy(&self.bytes.lock().unwrap()).into_owned()
    }

    fn end(&self) -> Option<SessionEnd> {
        self.end.lock().unwrap().clone()
    }
}

impl SessionOutput for RecordingOutput {
    fn data(&self, bytes: &[u8]) {
        self.bytes.lock().unwrap().extend_from_slice(bytes);
    }

    fn ended(&self, end: SessionEnd) {
        *self.end.lock().unwrap() = Some(end);
    }
}

struct TestServer {
    _known_hosts_directory: tempfile::TempDir,
    known_hosts: Arc<KnownHostsFile>,
}

impl TestServer {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let known_hosts = Arc::new(KnownHostsFile::new(directory.path().join("known_hosts")));
        Self {
            _known_hosts_directory: directory,
            known_hosts,
        }
    }

    fn endpoint(username: &str, port: u16) -> SshEndpoint {
        SshEndpoint::new(
            HostAddress::parse("127.0.0.1").unwrap(),
            Port::try_from(port).unwrap(),
            Username::parse(username).unwrap(),
        )
    }

    fn key_file() -> AuthMethod {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/ssh-server/.keys/id_ed25519");
        AuthMethod::KeyFile(KeyFilePath::parse(path.to_str().unwrap()).unwrap())
    }

    async fn connect_as(
        &self,
        username: &str,
        port: u16,
    ) -> (
        Result<Arc<dyn LiveSession>, OpenSessionError>,
        Arc<RecordingOutput>,
    ) {
        self.connect_with(username, port, Self::key_file(), None)
            .await
    }

    async fn connect_with(
        &self,
        username: &str,
        port: u16,
        auth: AuthMethod,
        secret: Option<Secret>,
    ) -> (
        Result<Arc<dyn LiveSession>, OpenSessionError>,
        Arc<RecordingOutput>,
    ) {
        let output = Arc::new(RecordingOutput::default());
        let result = RusshConnector::new()
            .connect(ConnectRequest {
                endpoint: Self::endpoint(username, port),
                auth,
                secret,
                size: TerminalSize::new(80, 24).unwrap(),
                known_hosts: Arc::clone(&self.known_hosts) as Arc<dyn KnownHosts>,
                output: Arc::clone(&output) as Arc<dyn SessionOutput>,
            })
            .await;
        (result, output)
    }

    /// Connects once to learn the server key, trusts it, then connects for real.
    async fn trusted_session(&self) -> (Arc<dyn LiveSession>, Arc<RecordingOutput>) {
        let (first_attempt, _) = self.connect_as("sallygate", TEST_SERVER_PORT).await;
        let Err(OpenSessionError::UnknownHostKey(host_key)) = first_attempt else {
            panic!(
                "expected an unknown host key on the first connection, got {:?}",
                first_attempt.err()
            );
        };
        self.known_hosts
            .trust(&Self::endpoint("sallygate", TEST_SERVER_PORT), &host_key)
            .unwrap();

        let (session, output) = self.connect_as("sallygate", TEST_SERVER_PORT).await;
        (session.unwrap(), output)
    }
}

async fn wait_until(description: &str, condition: impl Fn() -> bool) {
    for _ in 0..100 {
        if condition() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("timed out waiting until {description}");
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn refuses_an_unknown_server_and_reports_its_fingerprint() {
    let server = TestServer::new();

    let (result, _) = server.connect_as("sallygate", TEST_SERVER_PORT).await;

    let Err(OpenSessionError::UnknownHostKey(host_key)) = result else {
        panic!(
            "expected an unknown host key, got {result:?}",
            result = result.err()
        );
    };
    assert!(host_key.fingerprint().starts_with("SHA256:"));
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn runs_commands_in_the_shell_and_streams_their_output() {
    let server = TestServer::new();
    let (session, output) = server.trusted_session().await;

    session.write(b"echo sallygate-$((6 * 7))\n").await.unwrap();

    wait_until("the command output arrives", || {
        output.text().contains("sallygate-42")
    })
    .await;
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn tells_the_shell_the_new_terminal_size() {
    let server = TestServer::new();
    let (session, output) = server.trusted_session().await;

    session
        .resize(TerminalSize::new(100, 30).unwrap())
        .await
        .unwrap();
    session.write(b"stty size\n").await.unwrap();

    wait_until("the shell reports 30 rows and 100 columns", || {
        output.text().contains("30 100")
    })
    .await;
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn reports_the_exit_status_when_the_shell_exits() {
    let server = TestServer::new();
    let (session, output) = server.trusted_session().await;

    session.write(b"exit 3\n").await.unwrap();

    wait_until("the session ends", || output.end().is_some()).await;
    assert_eq!(output.end(), Some(SessionEnd::Exited(3)));
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn does_not_report_an_end_when_the_user_closes_the_session() {
    let server = TestServer::new();
    let (session, output) = server.trusted_session().await;

    session.close().await.unwrap();
    tokio::time::sleep(Duration::from_millis(300)).await;

    assert_eq!(output.end(), None);
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn reports_a_rejected_login() {
    let server = TestServer::new();
    server.trusted_session().await;

    let (result, _) = server.connect_as("someone-else", TEST_SERVER_PORT).await;

    assert_eq!(result.err(), Some(OpenSessionError::AuthenticationRejected));
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn logs_in_with_a_password() {
    let server = TestServer::new();
    server.trusted_session().await;
    let password = Secret::new(TEST_PASSWORD.to_owned());

    let (result, _) = server
        .connect_with(
            "sallygate",
            TEST_SERVER_PORT,
            AuthMethod::Password,
            Some(password),
        )
        .await;

    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn asks_for_a_password_it_was_not_given() {
    let server = TestServer::new();
    server.trusted_session().await;

    let (result, _) = server
        .connect_with("sallygate", TEST_SERVER_PORT, AuthMethod::Password, None)
        .await;

    assert_eq!(result.err(), Some(OpenSessionError::SecretRequired));
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn reports_a_wrong_password_as_a_rejected_login() {
    let server = TestServer::new();
    server.trusted_session().await;
    let password = Secret::new("not-the-password".to_owned());

    let (result, _) = server
        .connect_with(
            "sallygate",
            TEST_SERVER_PORT,
            AuthMethod::Password,
            Some(password),
        )
        .await;

    assert_eq!(result.err(), Some(OpenSessionError::AuthenticationRejected));
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn refuses_a_server_whose_key_changed() {
    let server = TestServer::new();
    let impostor_key = sallygate_domain::trust::HostKey::new(
        "ssh-ed25519".to_owned(),
        "SHA256:impostor".to_owned(),
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIO8s/hhw5ji20pKlZuA5sy2IAShfDR14IXb9OXjRypwr"
            .to_owned(),
    );
    server
        .known_hosts
        .trust(
            &TestServer::endpoint("sallygate", TEST_SERVER_PORT),
            &impostor_key,
        )
        .unwrap();

    let (result, _) = server.connect_as("sallygate", TEST_SERVER_PORT).await;

    assert!(matches!(result, Err(OpenSessionError::HostKeyChanged(_))));
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn reports_an_unreachable_server() {
    let server = TestServer::new();

    let (result, _) = server.connect_as("sallygate", 1).await;

    assert!(matches!(result, Err(OpenSessionError::Unreachable(_))));
}

struct IgnoreProgress;

impl sallygate_application::transfer::TransferProgress for IgnoreProgress {
    fn advanced(&self, _done_bytes: u64, _total_bytes: u64) {}
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn browses_and_copies_files_over_the_session_connection() {
    use sallygate_domain::transfer::EntryKind;

    let server = TestServer::new();
    let (session, _) = server.trusted_session().await;
    let files = session.open_files().await.unwrap();
    let home = files.home().await.unwrap();
    let folder = format!("{home}/sftp-test-{}", std::process::id());
    let local = tempfile::tempdir().unwrap();
    let original = local.path().join("hello.txt");
    std::fs::write(&original, "hello over sftp").unwrap();

    files.make_folder(&folder).await.unwrap();
    let remote_file = format!("{folder}/hello.txt");
    files
        .upload(&original, &remote_file, Arc::new(IgnoreProgress))
        .await
        .unwrap();
    let listed = files.list(&folder).await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].name, "hello.txt");
    assert_eq!(listed[0].size_bytes, 15);

    let downloaded = files
        .download(&remote_file, local.path(), Arc::new(IgnoreProgress))
        .await
        .unwrap();
    assert_eq!(downloaded, local.path().join("hello (1).txt"));
    assert_eq!(
        std::fs::read_to_string(downloaded).unwrap(),
        "hello over sftp"
    );

    let renamed = format!("{folder}/renamed.txt");
    files.rename(&remote_file, &renamed).await.unwrap();
    files.remove(&renamed, EntryKind::File).await.unwrap();
    files.remove(&folder, EntryKind::Folder).await.unwrap();
    assert!(
        !files
            .list(&home)
            .await
            .unwrap()
            .iter()
            .any(|entry| entry.path == folder)
    );
    session.close().await.unwrap();
}
