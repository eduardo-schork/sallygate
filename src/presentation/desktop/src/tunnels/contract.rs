use sallygate_application::tunnels::{OpenTunnelError, StartTunnelError, TunnelState};
use sallygate_domain::inventory::{HostAddress, HostId, Port};
use sallygate_domain::tunnels::{Tunnel, TunnelId, TunnelKind, TunnelName};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::command_error::{CommandError, CommandErrorKind};
use crate::sessions::from_open_session_error;

/// How a rule forwards; both listen on this computer only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(
    rename = "TTunnelKind",
    export,
    export_to = "../../ui/tunnels/tunnels-contract.ts"
)]
pub enum TunnelKindDto {
    /// `localhost:localPort` reaches `targetAddress:targetPort` as seen from the server.
    #[serde(rename_all = "camelCase")]
    Local {
        local_port: u16,
        target_address: String,
        target_port: u16,
    },
    /// A SOCKS5 proxy on `localhost:localPort`.
    #[serde(rename_all = "camelCase")]
    Dynamic { local_port: u16 },
}

impl From<&TunnelKind> for TunnelKindDto {
    fn from(kind: &TunnelKind) -> Self {
        match kind {
            TunnelKind::Local {
                local_port,
                target_address,
                target_port,
            } => Self::Local {
                local_port: local_port.value(),
                target_address: target_address.to_string(),
                target_port: target_port.value(),
            },
            TunnelKind::Dynamic { local_port } => Self::Dynamic {
                local_port: local_port.value(),
            },
        }
    }
}

impl TunnelKindDto {
    fn into_domain(self) -> Result<TunnelKind, CommandError> {
        let port =
            |value: u16| Port::try_from(value).map_err(|error| CommandError::invalid_input(&error));
        Ok(match self {
            Self::Local {
                local_port,
                target_address,
                target_port,
            } => TunnelKind::Local {
                local_port: port(local_port)?,
                target_address: HostAddress::parse(&target_address)
                    .map_err(|error| CommandError::invalid_input(&error))?,
                target_port: port(target_port)?,
            },
            Self::Dynamic { local_port } => TunnelKind::Dynamic {
                local_port: port(local_port)?,
            },
        })
    }
}

/// A saved rule and whether it is forwarding now.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TTunnel",
    export,
    export_to = "../../ui/tunnels/tunnels-contract.ts"
)]
pub struct TunnelDto {
    pub id: String,
    pub name: String,
    pub host_id: String,
    pub kind: TunnelKindDto,
    pub is_on: bool,
}

impl From<&TunnelState> for TunnelDto {
    fn from(state: &TunnelState) -> Self {
        Self {
            id: state.tunnel.id.to_string(),
            name: state.tunnel.name.as_str().to_owned(),
            host_id: state.tunnel.host_id.to_string(),
            kind: TunnelKindDto::from(&state.tunnel.kind),
            is_on: state.is_on,
        }
    }
}

/// A new rule (no `id`) or the changes to a saved one.
#[derive(Debug, Clone, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(
    rename = "TSaveTunnelRequest",
    export,
    export_to = "../../ui/tunnels/tunnels-contract.ts"
)]
pub struct SaveTunnelRequest {
    #[ts(optional)]
    pub id: Option<String>,
    pub name: String,
    pub host_id: String,
    pub kind: TunnelKindDto,
}

impl SaveTunnelRequest {
    pub fn into_domain(self) -> Result<Tunnel, CommandError> {
        let invalid = |error: &dyn ToString| CommandError::invalid_input(&error.to_string());
        Ok(Tunnel {
            id: match self.id.as_deref() {
                Some(id) => id.parse::<TunnelId>().map_err(|error| invalid(&error))?,
                None => TunnelId::new(),
            },
            name: TunnelName::parse(&self.name).map_err(|error| invalid(&error))?,
            host_id: self
                .host_id
                .parse::<HostId>()
                .map_err(|error| invalid(&error))?,
            kind: self.kind.into_domain()?,
        })
    }
}

pub fn from_start_tunnel_error(error: StartTunnelError) -> CommandError {
    match error {
        StartTunnelError::Open(OpenTunnelError::Connect(error)) => from_open_session_error(error),
        StartTunnelError::Open(error @ OpenTunnelError::PortInUse(_)) => {
            CommandError::new(CommandErrorKind::PortInUse, &error)
        }
        error @ (StartTunnelError::TunnelNotFound | StartTunnelError::HostNotFound) => {
            CommandError::invalid_input(&error)
        }
        StartTunnelError::Store(error) => {
            CommandError::new(CommandErrorKind::TunnelStoreUnavailable, &error)
        }
        StartTunnelError::Hosts(error) => {
            CommandError::new(CommandErrorKind::HostStoreUnavailable, &error)
        }
    }
}
