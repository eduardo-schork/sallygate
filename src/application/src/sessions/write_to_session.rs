use std::sync::Arc;

use sallygate_domain::sessions::SessionId;

use super::{SessionCommandError, SessionRegistry};

/// Use case: send what the user typed or pasted to a session.
pub struct WriteToSession {
    registry: Arc<SessionRegistry>,
}

impl WriteToSession {
    #[must_use]
    pub fn new(registry: Arc<SessionRegistry>) -> Self {
        Self { registry }
    }

    /// # Errors
    ///
    /// Returns an error when the session is not open or no longer accepts input.
    pub async fn execute(
        &self,
        session_id: SessionId,
        bytes: &[u8],
    ) -> Result<(), SessionCommandError> {
        let session = self.registry.get(session_id)?;
        session.write(bytes).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sessions::test_doubles::FakeLiveSession;

    #[tokio::test]
    async fn sends_the_bytes_to_the_open_session() {
        let registry = Arc::new(SessionRegistry::new());
        let session = FakeLiveSession::new();
        let session_id = SessionId::new();
        registry.add(session_id, session.clone());

        WriteToSession::new(Arc::clone(&registry))
            .execute(session_id, b"ls\r")
            .await
            .unwrap();

        assert_eq!(session.written(), b"ls\r");
    }

    #[tokio::test]
    async fn rejects_a_session_that_is_not_open() {
        let session_id = SessionId::new();
        let result = WriteToSession::new(Arc::new(SessionRegistry::new()))
            .execute(session_id, b"ls\r")
            .await;

        assert_eq!(result, Err(SessionCommandError::NotFound(session_id)));
    }
}
