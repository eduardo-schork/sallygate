use std::sync::Arc;

use sallygate_domain::sessions::{SessionId, TerminalSize};

use super::{SessionCommandError, SessionRegistry};

/// Use case: the terminal on screen changed size.
pub struct ResizeSession {
    registry: Arc<SessionRegistry>,
}

impl ResizeSession {
    #[must_use]
    pub fn new(registry: Arc<SessionRegistry>) -> Self {
        Self { registry }
    }

    /// # Errors
    ///
    /// Returns an error when the session is not open or cannot be resized.
    pub async fn execute(
        &self,
        session_id: SessionId,
        size: TerminalSize,
    ) -> Result<(), SessionCommandError> {
        let session = self.registry.get(session_id)?;
        session.resize(size).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sessions::test_doubles::FakeLiveSession;

    #[tokio::test]
    async fn resizes_the_open_session() {
        let registry = Arc::new(SessionRegistry::new());
        let session = FakeLiveSession::new();
        let session_id = SessionId::new();
        registry.add(session_id, session.clone());
        let size = TerminalSize::new(120, 40).unwrap();

        ResizeSession::new(Arc::clone(&registry))
            .execute(session_id, size)
            .await
            .unwrap();

        assert_eq!(session.sizes(), vec![size]);
    }
}
