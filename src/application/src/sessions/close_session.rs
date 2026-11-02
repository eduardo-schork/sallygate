use std::sync::Arc;

use sallygate_domain::sessions::SessionId;

use super::{SessionCommandError, SessionRegistry};

/// Use case: the user closed a session's tab.
pub struct CloseSession {
    registry: Arc<SessionRegistry>,
}

impl CloseSession {
    #[must_use]
    pub fn new(registry: Arc<SessionRegistry>) -> Self {
        Self { registry }
    }

    /// Closing a session that is already gone is not an error: the outcome is the same.
    ///
    /// # Errors
    ///
    /// Returns an error when the connection could not be closed cleanly.
    pub async fn execute(&self, session_id: SessionId) -> Result<(), SessionCommandError> {
        if let Some(session) = self.registry.remove(session_id) {
            session.close().await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sessions::test_doubles::FakeLiveSession;

    #[tokio::test]
    async fn closes_and_forgets_the_session() {
        let registry = Arc::new(SessionRegistry::new());
        let session = FakeLiveSession::new();
        let session_id = SessionId::new();
        registry.add(session_id, session.clone());

        CloseSession::new(Arc::clone(&registry))
            .execute(session_id)
            .await
            .unwrap();

        assert!(session.is_closed());
        assert!(!registry.is_open(session_id));
    }

    #[tokio::test]
    async fn treats_an_already_closed_session_as_closed() {
        let result = CloseSession::new(Arc::new(SessionRegistry::new()))
            .execute(SessionId::new())
            .await;

        assert_eq!(result, Ok(()));
    }
}
