use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use sallygate_domain::credentials::Secret;
use sallygate_domain::tunnels::{Tunnel, TunnelId};
use thiserror::Error;

use super::{
    LiveTunnel, OpenTunnelError, OpenTunnelRequest, TunnelOpener, TunnelRepository,
    TunnelStoreError,
};
use crate::inventory::{HostRepository, HostStoreError};
use crate::sessions::JumpHost;
use crate::trust::KnownHosts;

/// A saved rule and whether it is forwarding now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TunnelState {
    pub tunnel: Tunnel,
    pub is_on: bool,
}

/// Why a rule could not be turned on. Messages are safe to show the user.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StartTunnelError {
    #[error("the forwarding rule no longer exists")]
    TunnelNotFound,
    #[error("the rule's host was deleted; edit the rule to pick another")]
    HostNotFound,
    #[error(transparent)]
    Store(#[from] TunnelStoreError),
    #[error(transparent)]
    Hosts(#[from] HostStoreError),
    #[error(transparent)]
    Open(#[from] OpenTunnelError),
}

/// Use case: keep forwarding rules and turn them on and off.
pub struct ManageTunnels {
    repository: Arc<dyn TunnelRepository>,
    hosts: Arc<dyn HostRepository>,
    opener: Arc<dyn TunnelOpener>,
    known_hosts: Arc<dyn KnownHosts>,
    running: Mutex<HashMap<TunnelId, Arc<dyn LiveTunnel>>>,
}

impl ManageTunnels {
    #[must_use]
    pub fn new(
        repository: Arc<dyn TunnelRepository>,
        hosts: Arc<dyn HostRepository>,
        opener: Arc<dyn TunnelOpener>,
        known_hosts: Arc<dyn KnownHosts>,
    ) -> Self {
        Self {
            repository,
            hosts,
            opener,
            known_hosts,
            running: Mutex::new(HashMap::new()),
        }
    }

    /// Rules sorted by name, each with whether it is on.
    ///
    /// # Errors
    ///
    /// Returns an error when the rules cannot be read.
    pub fn list(&self) -> Result<Vec<TunnelState>, TunnelStoreError> {
        let running = self.lock();
        let mut states: Vec<_> = self
            .repository
            .all()?
            .into_iter()
            .map(|tunnel| TunnelState {
                is_on: running.contains_key(&tunnel.id),
                tunnel,
            })
            .collect();
        states.sort_by_cached_key(|state| state.tunnel.name.as_str().to_lowercase());
        Ok(states)
    }

    /// Saves a rule. A rule that is on keeps its old settings until it is turned off and on.
    ///
    /// # Errors
    ///
    /// Returns an error when the rule cannot be written.
    pub fn save(&self, tunnel: &Tunnel) -> Result<(), TunnelStoreError> {
        self.repository.save(tunnel)
    }

    /// Turns the rule off, then forgets it.
    ///
    /// # Errors
    ///
    /// Returns an error when the rules cannot be written.
    pub async fn delete(&self, tunnel_id: TunnelId) -> Result<(), TunnelStoreError> {
        self.stop(tunnel_id).await;
        self.repository.delete(tunnel_id)
    }

    /// Logs in to the rule's host and starts forwarding. Turning on a rule that is on does nothing.
    ///
    /// # Errors
    ///
    /// Returns why it could not start, such as an unknown server key or a missing password.
    pub async fn start(
        &self,
        tunnel_id: TunnelId,
        secret: Option<Secret>,
    ) -> Result<(), StartTunnelError> {
        if self.lock().contains_key(&tunnel_id) {
            return Ok(());
        }
        let tunnel = self
            .repository
            .all()?
            .into_iter()
            .find(|tunnel| tunnel.id == tunnel_id)
            .ok_or(StartTunnelError::TunnelNotFound)?;
        let hosts = self.hosts.all()?;
        let host = hosts
            .iter()
            .find(|host| host.id() == tunnel.host_id)
            .ok_or(StartTunnelError::HostNotFound)?;
        let jump = match host.jump_host() {
            None => None,
            Some(jump_id) => {
                let jump = hosts
                    .iter()
                    .find(|saved| saved.id() == jump_id)
                    .ok_or(StartTunnelError::HostNotFound)?;
                Some(JumpHost {
                    endpoint: jump.endpoint().clone(),
                    auth: jump.auth().clone(),
                })
            }
        };
        let live = self
            .opener
            .open(OpenTunnelRequest {
                endpoint: host.endpoint().clone(),
                auth: host.auth().clone(),
                jump,
                secret,
                known_hosts: Arc::clone(&self.known_hosts),
                kind: tunnel.kind,
            })
            .await?;
        self.lock().insert(tunnel_id, live);
        Ok(())
    }

    /// Stops forwarding. Stopping a rule that is off does nothing.
    pub async fn stop(&self, tunnel_id: TunnelId) {
        let live = self.lock().remove(&tunnel_id);
        if let Some(live) = live {
            live.close().await;
        }
    }

    // The map stays consistent even if a holder panicked, so a poisoned lock is still safe to use.
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<TunnelId, Arc<dyn LiveTunnel>>> {
        self.running.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    use async_trait::async_trait;
    use sallygate_domain::inventory::{HostAddress, Port};
    use sallygate_domain::tunnels::{TunnelKind, TunnelName};

    use super::*;
    use crate::inventory::test_doubles::{InMemoryHosts, host};
    use crate::sessions::test_doubles::FakeKnownHosts;

    #[derive(Default)]
    struct InMemoryTunnels(Mutex<Vec<Tunnel>>);

    impl TunnelRepository for InMemoryTunnels {
        fn all(&self) -> Result<Vec<Tunnel>, TunnelStoreError> {
            Ok(self.0.lock().unwrap().clone())
        }

        fn save(&self, tunnel: &Tunnel) -> Result<(), TunnelStoreError> {
            let mut tunnels = self.0.lock().unwrap();
            tunnels.retain(|saved| saved.id != tunnel.id);
            tunnels.push(tunnel.clone());
            Ok(())
        }

        fn delete(&self, tunnel_id: TunnelId) -> Result<(), TunnelStoreError> {
            self.0.lock().unwrap().retain(|saved| saved.id != tunnel_id);
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeLiveTunnel(AtomicBool);

    #[async_trait]
    impl LiveTunnel for FakeLiveTunnel {
        async fn close(&self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    #[derive(Default)]
    struct FakeOpener {
        opens: AtomicUsize,
        last: Mutex<Option<Arc<FakeLiveTunnel>>>,
    }

    #[async_trait]
    impl TunnelOpener for FakeOpener {
        async fn open(
            &self,
            _request: OpenTunnelRequest,
        ) -> Result<Arc<dyn LiveTunnel>, OpenTunnelError> {
            self.opens.fetch_add(1, Ordering::SeqCst);
            let live = Arc::new(FakeLiveTunnel::default());
            *self.last.lock().unwrap() = Some(Arc::clone(&live));
            Ok(live)
        }
    }

    fn rule(host_id: sallygate_domain::inventory::HostId) -> Tunnel {
        let port = Port::try_from(5432).unwrap();
        Tunnel {
            id: TunnelId::new(),
            name: TunnelName::parse("Postgres").unwrap(),
            host_id,
            kind: TunnelKind::Local {
                local_port: port,
                target_address: HostAddress::parse("localhost").unwrap(),
                target_port: port,
            },
        }
    }

    fn manage(hosts: Arc<InMemoryHosts>) -> (ManageTunnels, Arc<FakeOpener>) {
        let opener = Arc::new(FakeOpener::default());
        let tunnels = ManageTunnels::new(
            Arc::new(InMemoryTunnels::default()),
            hosts,
            opener.clone(),
            Arc::new(FakeKnownHosts),
        );
        (tunnels, opener)
    }

    #[tokio::test]
    async fn turns_a_rule_on_once_and_off_again() {
        let saved_host = host("oracle", None);
        let (tunnels, opener) = manage(InMemoryHosts::with(vec![saved_host.clone()]));
        let tunnel = rule(saved_host.id());
        tunnels.save(&tunnel).unwrap();

        tunnels.start(tunnel.id, None).await.unwrap();
        tunnels.start(tunnel.id, None).await.unwrap();
        assert!(tunnels.list().unwrap()[0].is_on);
        assert_eq!(opener.opens.load(Ordering::SeqCst), 1);

        tunnels.stop(tunnel.id).await;
        assert!(!tunnels.list().unwrap()[0].is_on);
        let live = opener.last.lock().unwrap().clone().unwrap();
        assert!(live.0.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn refuses_a_rule_whose_host_is_gone_and_stops_a_deleted_rule() {
        let saved_host = host("oracle", None);
        let (tunnels, _) = manage(InMemoryHosts::with(vec![saved_host.clone()]));
        let orphan = rule(sallygate_domain::inventory::HostId::new());
        tunnels.save(&orphan).unwrap();
        assert_eq!(
            tunnels.start(orphan.id, None).await,
            Err(StartTunnelError::HostNotFound)
        );

        let tunnel = rule(saved_host.id());
        tunnels.save(&tunnel).unwrap();
        tunnels.start(tunnel.id, None).await.unwrap();
        tunnels.delete(tunnel.id).await.unwrap();

        assert_eq!(tunnels.list().unwrap().len(), 1);
    }
}
