//! In-memory fakes of the session ports, for use case tests.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use sallygate_domain::credentials::{AuthMethod, KeyFilePath};
use sallygate_domain::inventory::{HostAddress, Port, SshEndpoint, Username};
use sallygate_domain::sessions::{SessionEnd, TerminalSize};
use sallygate_domain::trust::{HostKey, HostKeyStatus};

use super::{
    ConnectRequest, LiveSession, LiveSessionError, OpenSessionError, OpenSessionInput,
    SessionOutput, SshConnector,
};
use crate::trust::{KnownHosts, KnownHostsError};

pub fn open_input() -> OpenSessionInput {
    OpenSessionInput {
        endpoint: SshEndpoint::new(
            HostAddress::parse("example.com").unwrap(),
            Port::default(),
            Username::parse("ubuntu").unwrap(),
        ),
        auth: AuthMethod::KeyFile(KeyFilePath::parse("/keys/id_ed25519").unwrap()),
        secret: None,
        size: TerminalSize::new(80, 24).unwrap(),
    }
}

#[derive(Default)]
pub struct FakeKnownHosts;

impl KnownHosts for FakeKnownHosts {
    fn check(&self, _: &SshEndpoint, _: &HostKey) -> Result<HostKeyStatus, KnownHostsError> {
        Ok(HostKeyStatus::Trusted)
    }

    fn trust(&self, _: &SshEndpoint, _: &HostKey) -> Result<(), KnownHostsError> {
        Ok(())
    }
}

#[derive(Default)]
struct LiveSessionRecord {
    written: Vec<u8>,
    sizes: Vec<TerminalSize>,
    is_closed: bool,
}

#[derive(Clone, Default)]
pub struct FakeLiveSession(Arc<Mutex<LiveSessionRecord>>);

impl FakeLiveSession {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn written(&self) -> Vec<u8> {
        self.0.lock().unwrap().written.clone()
    }

    pub fn sizes(&self) -> Vec<TerminalSize> {
        self.0.lock().unwrap().sizes.clone()
    }

    pub fn is_closed(&self) -> bool {
        self.0.lock().unwrap().is_closed
    }
}

#[async_trait]
impl LiveSession for FakeLiveSession {
    async fn write(&self, bytes: &[u8]) -> Result<(), LiveSessionError> {
        self.0.lock().unwrap().written.extend_from_slice(bytes);
        Ok(())
    }

    async fn resize(&self, size: TerminalSize) -> Result<(), LiveSessionError> {
        self.0.lock().unwrap().sizes.push(size);
        Ok(())
    }

    async fn open_files(&self) -> Result<Arc<dyn crate::transfer::RemoteFiles>, LiveSessionError> {
        Err(LiveSessionError("no files in this fake".to_owned()))
    }

    async fn close(&self) -> Result<(), LiveSessionError> {
        self.0.lock().unwrap().is_closed = true;
        Ok(())
    }
}

#[derive(Clone)]
enum ConnectOutcome {
    Succeed,
    EndImmediately,
    Fail(OpenSessionError),
}

#[derive(Clone)]
pub struct FakeConnector {
    outcome: ConnectOutcome,
    last_output: Arc<Mutex<Option<Arc<dyn SessionOutput>>>>,
}

impl FakeConnector {
    fn with(outcome: ConnectOutcome) -> Self {
        Self {
            outcome,
            last_output: Arc::default(),
        }
    }

    pub fn succeeding() -> Self {
        Self::with(ConnectOutcome::Succeed)
    }

    pub fn ending_immediately() -> Self {
        Self::with(ConnectOutcome::EndImmediately)
    }

    pub fn failing(error: OpenSessionError) -> Self {
        Self::with(ConnectOutcome::Fail(error))
    }

    pub fn last_output(&self) -> Arc<dyn SessionOutput> {
        self.last_output.lock().unwrap().clone().unwrap()
    }
}

#[async_trait]
impl SshConnector for FakeConnector {
    async fn connect(
        &self,
        request: ConnectRequest,
    ) -> Result<Arc<dyn LiveSession>, OpenSessionError> {
        *self.last_output.lock().unwrap() = Some(Arc::clone(&request.output));

        match &self.outcome {
            ConnectOutcome::Succeed => Ok(FakeLiveSession::new()),
            ConnectOutcome::EndImmediately => {
                request.output.ended(SessionEnd::Exited(0));
                Ok(FakeLiveSession::new())
            }
            ConnectOutcome::Fail(error) => Err(error.clone()),
        }
    }
}

#[derive(Default)]
struct OutputRecord {
    bytes: Vec<u8>,
    ends: Vec<SessionEnd>,
}

#[derive(Clone, Default)]
pub struct RecordingOutput(Arc<Mutex<OutputRecord>>);

impl RecordingOutput {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.lock().unwrap().bytes.clone()
    }

    pub fn ends(&self) -> Vec<SessionEnd> {
        self.0.lock().unwrap().ends.clone()
    }
}

impl SessionOutput for RecordingOutput {
    fn data(&self, bytes: &[u8]) {
        self.0.lock().unwrap().bytes.extend_from_slice(bytes);
    }

    fn ended(&self, end: SessionEnd) {
        self.0.lock().unwrap().ends.push(end);
    }
}
