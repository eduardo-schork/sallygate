use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use sallygate_domain::sessions::SessionId;

use super::{RemoteFiles, TransferError};
use crate::sessions::SessionRegistry;

/// The SFTP connection of each session whose files were opened, so each session opens one.
pub struct OpenFileSystems {
    sessions: Arc<SessionRegistry>,
    opened: Mutex<HashMap<SessionId, Arc<dyn RemoteFiles>>>,
}

impl OpenFileSystems {
    #[must_use]
    pub fn new(sessions: Arc<SessionRegistry>) -> Self {
        Self {
            sessions,
            opened: Mutex::new(HashMap::new()),
        }
    }

    /// The files of an open session, opening them on first use.
    pub(super) async fn of(
        &self,
        session_id: SessionId,
    ) -> Result<Arc<dyn RemoteFiles>, TransferError> {
        let session = match self.sessions.get(session_id) {
            Ok(session) => session,
            Err(error) => {
                // The session closed, so its file connection is gone too.
                self.lock().remove(&session_id);
                return Err(error.into());
            }
        };
        if let Some(files) = self.lock().get(&session_id) {
            return Ok(Arc::clone(files));
        }
        let files = session.open_files().await?;
        self.lock().insert(session_id, Arc::clone(&files));
        Ok(files)
    }

    // Holders never leave the map half-written, so a poisoned lock is still safe to use.
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<SessionId, Arc<dyn RemoteFiles>>> {
        self.opened.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
