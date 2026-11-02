use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use sallygate_domain::credentials::{AuthMethod, Secret};
use sallygate_domain::inventory::SshEndpoint;
use sallygate_domain::sessions::{SessionEnd, SessionId, TerminalSize};

use super::{ConnectRequest, OpenSessionError, SessionOutput, SessionRegistry, SshConnector};
use crate::trust::KnownHosts;

/// What the user asked to open.
pub struct OpenSessionInput {
    pub endpoint: SshEndpoint,
    pub auth: AuthMethod,
    pub secret: Option<Secret>,
    pub size: TerminalSize,
}

/// Use case: open an interactive terminal on a server.
pub struct OpenSession {
    connector: Arc<dyn SshConnector>,
    known_hosts: Arc<dyn KnownHosts>,
    registry: Arc<SessionRegistry>,
}

impl OpenSession {
    #[must_use]
    pub fn new(
        connector: Arc<dyn SshConnector>,
        known_hosts: Arc<dyn KnownHosts>,
        registry: Arc<SessionRegistry>,
    ) -> Self {
        Self {
            connector,
            known_hosts,
            registry,
        }
    }

    /// Opens the session and registers it so the other session use cases can reach it.
    ///
    /// # Errors
    ///
    /// Returns why the session could not be opened, including an unknown or
    /// changed server key that the user must decide on first.
    pub async fn execute(
        &self,
        input: OpenSessionInput,
        output: Arc<dyn SessionOutput>,
    ) -> Result<SessionId, OpenSessionError> {
        let session_id = SessionId::new();
        let output = Arc::new(ForgetSessionWhenItEnds {
            session_id,
            registry: Arc::clone(&self.registry),
            has_ended: AtomicBool::new(false),
            output,
        });

        let session = self
            .connector
            .connect(ConnectRequest {
                endpoint: input.endpoint,
                auth: input.auth,
                secret: input.secret,
                size: input.size,
                known_hosts: Arc::clone(&self.known_hosts),
                output: Arc::clone(&output) as Arc<dyn SessionOutput>,
            })
            .await?;

        self.registry.add(session_id, session);
        // The shell may have exited before the session was registered.
        if output.has_ended.load(Ordering::SeqCst) {
            self.registry.remove(session_id);
        }

        Ok(session_id)
    }
}

/// Removes a session from the registry when it ends on its own, then passes events on.
struct ForgetSessionWhenItEnds {
    session_id: SessionId,
    registry: Arc<SessionRegistry>,
    has_ended: AtomicBool,
    output: Arc<dyn SessionOutput>,
}

impl SessionOutput for ForgetSessionWhenItEnds {
    fn data(&self, bytes: &[u8]) {
        self.output.data(bytes);
    }

    fn ended(&self, end: SessionEnd) {
        self.has_ended.store(true, Ordering::SeqCst);
        self.registry.remove(self.session_id);
        self.output.ended(end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sessions::test_doubles::{
        FakeConnector, FakeKnownHosts, RecordingOutput, open_input,
    };

    fn open_session(connector: FakeConnector, registry: &Arc<SessionRegistry>) -> OpenSession {
        OpenSession::new(
            Arc::new(connector),
            Arc::new(FakeKnownHosts),
            Arc::clone(registry),
        )
    }

    #[tokio::test]
    async fn registers_the_session_it_opens() {
        let registry = Arc::new(SessionRegistry::new());
        let use_case = open_session(FakeConnector::succeeding(), &registry);

        let session_id = use_case
            .execute(open_input(), RecordingOutput::new())
            .await
            .unwrap();

        assert!(registry.is_open(session_id));
    }

    #[tokio::test]
    async fn passes_on_why_the_connection_failed() {
        let registry = Arc::new(SessionRegistry::new());
        let use_case = open_session(
            FakeConnector::failing(OpenSessionError::AuthenticationRejected),
            &registry,
        );

        let result = use_case.execute(open_input(), RecordingOutput::new()).await;

        assert_eq!(result, Err(OpenSessionError::AuthenticationRejected));
    }

    #[tokio::test]
    async fn forgets_a_session_that_ends_on_its_own_and_tells_the_output() {
        let registry = Arc::new(SessionRegistry::new());
        let connector = FakeConnector::succeeding();
        let use_case = open_session(connector.clone(), &registry);
        let output = RecordingOutput::new();

        let session_id = use_case
            .execute(open_input(), output.clone())
            .await
            .unwrap();
        connector.last_output().ended(SessionEnd::Exited(0));

        assert!(!registry.is_open(session_id));
        assert_eq!(output.ends(), vec![SessionEnd::Exited(0)]);
    }

    #[tokio::test]
    async fn forgets_a_session_that_ended_before_it_was_registered() {
        let registry = Arc::new(SessionRegistry::new());
        let use_case = open_session(FakeConnector::ending_immediately(), &registry);

        let session_id = use_case
            .execute(open_input(), RecordingOutput::new())
            .await
            .unwrap();

        assert!(!registry.is_open(session_id));
    }

    #[tokio::test]
    async fn forwards_output_bytes_unchanged() {
        let registry = Arc::new(SessionRegistry::new());
        let connector = FakeConnector::succeeding();
        let use_case = open_session(connector.clone(), &registry);
        let output = RecordingOutput::new();

        use_case
            .execute(open_input(), output.clone())
            .await
            .unwrap();
        connector.last_output().data(b"\x1b[32mhello");

        assert_eq!(output.bytes(), b"\x1b[32mhello");
    }
}
