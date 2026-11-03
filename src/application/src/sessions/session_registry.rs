use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use sallygate_domain::sessions::SessionId;

use super::{LiveSession, SessionCommandError};

/// The sessions that are open right now, shared by the session use cases.
#[derive(Default)]
pub struct SessionRegistry {
    sessions: Mutex<HashMap<SessionId, Arc<dyn LiveSession>>>,
}

impl SessionRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add(&self, session_id: SessionId, session: Arc<dyn LiveSession>) {
        self.lock().insert(session_id, session);
    }

    pub(crate) fn remove(&self, session_id: SessionId) -> Option<Arc<dyn LiveSession>> {
        self.lock().remove(&session_id)
    }

    /// # Errors
    ///
    /// Returns an error when no open session has this id.
    pub fn get(&self, session_id: SessionId) -> Result<Arc<dyn LiveSession>, SessionCommandError> {
        self.lock()
            .get(&session_id)
            .cloned()
            .ok_or(SessionCommandError::NotFound(session_id))
    }

    #[must_use]
    pub fn is_open(&self, session_id: SessionId) -> bool {
        self.lock().contains_key(&session_id)
    }

    // The map stays consistent even if a holder panicked, so a poisoned lock is still safe to use.
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<SessionId, Arc<dyn LiveSession>>> {
        self.sessions.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
