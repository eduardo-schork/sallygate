use std::sync::{Arc, Mutex, PoisonError};

use russh::client;
use russh::keys::PublicKeyOrCertificate;
use sallygate_application::sessions::OpenSessionError;
use sallygate_application::trust::KnownHosts;
use sallygate_domain::inventory::SshEndpoint;
use sallygate_domain::trust::HostKeyStatus;

use crate::trust::host_key_from_public_key;

/// The russh event handler for one connection. Its only job is to accept or
/// refuse the server key, remembering why it refused.
pub(super) struct ServerKeyCheck {
    endpoint: SshEndpoint,
    known_hosts: Arc<dyn KnownHosts>,
    rejection: Arc<Mutex<Option<OpenSessionError>>>,
}

impl ServerKeyCheck {
    pub(super) fn new(
        endpoint: SshEndpoint,
        known_hosts: Arc<dyn KnownHosts>,
    ) -> (Self, Arc<Mutex<Option<OpenSessionError>>>) {
        let rejection = Arc::new(Mutex::new(None));
        let check = Self {
            endpoint,
            known_hosts,
            rejection: Arc::clone(&rejection),
        };
        (check, rejection)
    }

    fn reject(&self, reason: OpenSessionError) -> bool {
        *self
            .rejection
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some(reason);
        false
    }
}

impl ServerKeyCheck {
    /// Accepts only a key `known_hosts` already trusts.
    fn accepts(&self, server_key: &PublicKeyOrCertificate) -> bool {
        let PublicKeyOrCertificate::PublicKey { key, .. } = server_key else {
            return self.reject(OpenSessionError::Protocol(
                "servers that present certificates are not supported yet".to_owned(),
            ));
        };
        let host_key = match host_key_from_public_key(key) {
            Ok(host_key) => host_key,
            Err(reason) => return self.reject(OpenSessionError::Protocol(reason)),
        };

        match self.known_hosts.check(&self.endpoint, &host_key) {
            Ok(HostKeyStatus::Trusted) => true,
            Ok(HostKeyStatus::Unknown) => self.reject(OpenSessionError::UnknownHostKey(host_key)),
            Ok(HostKeyStatus::Changed) => self.reject(OpenSessionError::HostKeyChanged(host_key)),
            Err(error) => self.reject(OpenSessionError::KnownHosts(error)),
        }
    }
}

impl client::Handler for ServerKeyCheck {
    type Error = russh::Error;

    // The check reads a local file and never waits, so the future is ready at once.
    fn check_server_key(
        &mut self,
        server_key: &PublicKeyOrCertificate,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        std::future::ready(Ok(self.accepts(server_key)))
    }
}
