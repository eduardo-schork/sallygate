//! The composition root: the only place that knows which adapter backs each port.

use std::sync::Arc;

use sallygate_application::sessions::{
    CloseSession, OpenSession, ResizeSession, SessionRegistry, WriteToSession,
};
use sallygate_application::trust::{KnownHosts, TrustHostKey};
use sallygate_infrastructure::sessions::RusshConnector;
use sallygate_infrastructure::trust::KnownHostsFile;
use thiserror::Error;

/// The use cases the Tauri commands call, shared as Tauri state.
pub struct AppServices {
    pub open_session: OpenSession,
    pub write_to_session: WriteToSession,
    pub resize_session: ResizeSession,
    pub close_session: CloseSession,
    pub trust_host_key: TrustHostKey,
}

#[derive(Debug, Error)]
pub enum CompositionError {
    #[error("the home folder could not be found, so ~/.ssh/known_hosts is unavailable")]
    NoHomeFolder,
}

/// Builds the use cases on top of the real adapters.
///
/// # Errors
///
/// Returns an error when the user's home folder cannot be found.
pub fn compose() -> Result<AppServices, CompositionError> {
    let home = std::env::home_dir().ok_or(CompositionError::NoHomeFolder)?;
    let known_hosts: Arc<dyn KnownHosts> =
        Arc::new(KnownHostsFile::new(home.join(".ssh").join("known_hosts")));
    let registry = Arc::new(SessionRegistry::new());

    Ok(AppServices {
        open_session: OpenSession::new(
            Arc::new(RusshConnector::new()),
            Arc::clone(&known_hosts),
            Arc::clone(&registry),
        ),
        write_to_session: WriteToSession::new(Arc::clone(&registry)),
        resize_session: ResizeSession::new(Arc::clone(&registry)),
        close_session: CloseSession::new(registry),
        trust_host_key: TrustHostKey::new(known_hosts),
    })
}
