//! End-to-end folder copy over SFTP against the local SSH test server.
//! Start it first with `scripts/test-ssh-server.sh up`, then run `bun run test:integration`.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "test helpers fail loudly, like the tests that use them"
)]

use std::path::PathBuf;
use std::sync::Arc;

use sallygate_application::sessions::{
    OpenSession, OpenSessionError, OpenSessionInput, SessionOutput, SessionRegistry,
};
use sallygate_application::transfer::{OpenFileSystems, TransferFiles, TransferProgress};
use sallygate_application::trust::KnownHosts;
use sallygate_domain::credentials::{AuthMethod, KeyFilePath};
use sallygate_domain::inventory::{HostAddress, Port, SshEndpoint, Username};
use sallygate_domain::sessions::{SessionEnd, TerminalSize};
use sallygate_domain::transfer::EntryKind;
use sallygate_infrastructure::sessions::RusshConnector;
use sallygate_infrastructure::transfer::LocalFileSystem;
use sallygate_infrastructure::trust::KnownHostsFile;

struct Quiet;

impl SessionOutput for Quiet {
    fn data(&self, _bytes: &[u8]) {}
    fn ended(&self, _end: SessionEnd) {}
}

struct IgnoreProgress;

impl TransferProgress for IgnoreProgress {
    fn advanced(&self, _done_bytes: u64, _total_bytes: u64) {}
}

fn input() -> OpenSessionInput {
    let key =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/ssh-server/.keys/id_ed25519");
    OpenSessionInput {
        endpoint: SshEndpoint::new(
            HostAddress::parse("127.0.0.1").unwrap(),
            Port::try_from(2222).unwrap(),
            Username::parse("sallygate").unwrap(),
        ),
        auth: AuthMethod::KeyFile(KeyFilePath::parse(key.to_str().unwrap()).unwrap()),
        jump: None,
        secret: None,
        size: TerminalSize::new(80, 24).unwrap(),
    }
}

#[tokio::test]
#[ignore = "needs the SSH test server: scripts/test-ssh-server.sh up"]
async fn copies_a_folder_with_a_subfolder_both_ways() {
    let workspace = tempfile::tempdir().unwrap();
    let known_hosts = Arc::new(KnownHostsFile::new(workspace.path().join("known_hosts")));
    let registry = Arc::new(SessionRegistry::new());
    let open_session = OpenSession::new(
        Arc::new(RusshConnector::new()),
        Arc::clone(&known_hosts) as Arc<dyn KnownHosts>,
        Arc::clone(&registry),
    );
    let first = open_session.execute(input(), Arc::new(Quiet)).await;
    let Err(OpenSessionError::UnknownHostKey(host_key)) = first else {
        panic!("expected an unknown host key first");
    };
    known_hosts.trust(&input().endpoint, &host_key).unwrap();
    let session_id = open_session
        .execute(input(), Arc::new(Quiet))
        .await
        .unwrap();

    let local_root = workspace.path().join("local");
    let site = local_root.join(format!("site-{}", std::process::id()));
    std::fs::create_dir_all(site.join("css")).unwrap();
    std::fs::write(site.join("index.html"), "<h1>hello</h1>").unwrap();
    std::fs::write(site.join("css/main.css"), "h1 { color: teal }").unwrap();
    let transfers = TransferFiles::new(
        Arc::new(OpenFileSystems::new(Arc::clone(&registry))),
        Arc::new(LocalFileSystem::new(local_root.clone())),
    );

    let remote = transfers
        .upload(session_id, &site, "/tmp", Arc::new(IgnoreProgress))
        .await
        .unwrap();
    let back = transfers
        .download(
            session_id,
            &remote,
            EntryKind::Folder,
            &local_root,
            Arc::new(IgnoreProgress),
        )
        .await
        .unwrap();

    assert_ne!(
        back, site,
        "the copy must not overwrite the original folder"
    );
    assert_eq!(
        std::fs::read_to_string(back.join("index.html")).unwrap(),
        "<h1>hello</h1>"
    );
    assert_eq!(
        std::fs::read_to_string(back.join("css/main.css")).unwrap(),
        "h1 { color: teal }"
    );
}
