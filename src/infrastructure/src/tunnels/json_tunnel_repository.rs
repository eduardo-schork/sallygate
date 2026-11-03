use std::path::PathBuf;
use std::sync::{Mutex, PoisonError};

use sallygate_application::tunnels::{TunnelRepository, TunnelStoreError};
use sallygate_domain::inventory::{HostAddress, Port};
use sallygate_domain::tunnels::{Tunnel, TunnelId, TunnelKind, TunnelName};
use serde::{Deserialize, Serialize};

use crate::json_file;

/// The file format version; bump it and migrate when the shape changes.
const FORMAT_VERSION: u32 = 1;

/// Saved forwarding rules in one JSON file (`tunnels.json` in the app data folder).
pub struct JsonTunnelRepository {
    path: PathBuf,
    // Serializes read-modify-write cycles so two saves cannot lose each other's change.
    write_lock: Mutex<()>,
}

impl JsonTunnelRepository {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            write_lock: Mutex::new(()),
        }
    }

    fn read_file(&self) -> Result<Vec<Tunnel>, TunnelStoreError> {
        let Some(file) =
            json_file::read::<TunnelsFile>(&self.path).map_err(TunnelStoreError::Read)?
        else {
            return Ok(Vec::new());
        };
        file.tunnels
            .into_iter()
            .map(TunnelRecord::into_tunnel)
            .collect()
    }

    fn write_file(&self, tunnels: &[Tunnel]) -> Result<(), TunnelStoreError> {
        let file = TunnelsFile {
            version: FORMAT_VERSION,
            tunnels: tunnels.iter().map(TunnelRecord::from_tunnel).collect(),
        };
        json_file::write_atomically(&self.path, &file).map_err(TunnelStoreError::Write)
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, ()> {
        self.write_lock
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

impl TunnelRepository for JsonTunnelRepository {
    fn all(&self) -> Result<Vec<Tunnel>, TunnelStoreError> {
        let _guard = self.lock();
        self.read_file()
    }

    fn save(&self, tunnel: &Tunnel) -> Result<(), TunnelStoreError> {
        let _guard = self.lock();
        let mut tunnels = self.read_file()?;
        match tunnels.iter_mut().find(|saved| saved.id == tunnel.id) {
            Some(saved) => *saved = tunnel.clone(),
            None => tunnels.push(tunnel.clone()),
        }
        self.write_file(&tunnels)
    }

    fn delete(&self, tunnel_id: TunnelId) -> Result<(), TunnelStoreError> {
        let _guard = self.lock();
        let mut tunnels = self.read_file()?;
        tunnels.retain(|tunnel| tunnel.id != tunnel_id);
        self.write_file(&tunnels)
    }
}

#[derive(Serialize, Deserialize)]
struct TunnelsFile {
    version: u32,
    tunnels: Vec<TunnelRecord>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TunnelRecord {
    id: String,
    name: String,
    host_id: String,
    kind: KindRecord,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum KindRecord {
    #[serde(rename_all = "camelCase")]
    Local {
        local_port: u16,
        target_address: String,
        target_port: u16,
    },
    #[serde(rename_all = "camelCase")]
    Dynamic { local_port: u16 },
}

impl TunnelRecord {
    fn from_tunnel(tunnel: &Tunnel) -> Self {
        Self {
            id: tunnel.id.to_string(),
            name: tunnel.name.as_str().to_owned(),
            host_id: tunnel.host_id.to_string(),
            kind: match &tunnel.kind {
                TunnelKind::Local {
                    local_port,
                    target_address,
                    target_port,
                } => KindRecord::Local {
                    local_port: local_port.value(),
                    target_address: target_address.to_string(),
                    target_port: target_port.value(),
                },
                TunnelKind::Dynamic { local_port } => KindRecord::Dynamic {
                    local_port: local_port.value(),
                },
            },
        }
    }

    /// A saved rule goes through the same validation as a new one.
    fn into_tunnel(self) -> Result<Tunnel, TunnelStoreError> {
        let invalid = |error: &dyn ToString| {
            TunnelStoreError::Read(format!("rule \"{}\": {}", self.name, error.to_string()))
        };
        let port = |value: u16| Port::try_from(value).map_err(|error| invalid(&error));
        Ok(Tunnel {
            id: self.id.parse().map_err(|error| invalid(&error))?,
            name: TunnelName::parse(&self.name).map_err(|error| invalid(&error))?,
            host_id: self.host_id.parse().map_err(|error| invalid(&error))?,
            kind: match &self.kind {
                KindRecord::Local {
                    local_port,
                    target_address,
                    target_port,
                } => TunnelKind::Local {
                    local_port: port(*local_port)?,
                    target_address: HostAddress::parse(target_address)
                        .map_err(|error| invalid(&error))?,
                    target_port: port(*target_port)?,
                },
                KindRecord::Dynamic { local_port } => TunnelKind::Dynamic {
                    local_port: port(*local_port)?,
                },
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use sallygate_domain::inventory::HostId;

    use super::*;

    #[test]
    fn keeps_both_kinds_of_rules_across_instances() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("tunnels.json");
        let port = |value| Port::try_from(value).unwrap();
        let local = Tunnel {
            id: TunnelId::new(),
            name: TunnelName::parse("Postgres").unwrap(),
            host_id: HostId::new(),
            kind: TunnelKind::Local {
                local_port: port(5432),
                target_address: HostAddress::parse("db.internal").unwrap(),
                target_port: port(5432),
            },
        };
        let socks = Tunnel {
            id: TunnelId::new(),
            name: TunnelName::parse("Proxy").unwrap(),
            host_id: HostId::new(),
            kind: TunnelKind::Dynamic {
                local_port: port(1080),
            },
        };
        let repository = JsonTunnelRepository::new(path.clone());
        repository.save(&local).unwrap();
        repository.save(&socks).unwrap();

        assert_eq!(
            JsonTunnelRepository::new(path).all().unwrap(),
            vec![local, socks]
        );
    }
}
