use std::fmt;
use std::str::FromStr;

use uuid::Uuid;

use super::TunnelsError;
use crate::inventory::{HostAddress, HostId, Port};

/// Identifies a forwarding rule for its whole life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TunnelId(Uuid);

impl TunnelId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TunnelId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for TunnelId {
    type Err = TunnelsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| TunnelsError::InvalidTunnelId(value.to_owned()))
    }
}

impl fmt::Display for TunnelId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// What the user calls a rule, like `Postgres on oracle`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TunnelName(String);

impl TunnelName {
    pub const MAX_LENGTH: usize = 64;

    /// # Errors
    ///
    /// Returns an error when the name is empty or too long.
    pub fn parse(name: &str) -> Result<Self, TunnelsError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(TunnelsError::EmptyName);
        }
        if name.chars().count() > Self::MAX_LENGTH {
            return Err(TunnelsError::NameTooLong {
                max: Self::MAX_LENGTH,
            });
        }
        Ok(Self(name.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// How a rule forwards. Both listen on this computer only (`127.0.0.1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TunnelKind {
    /// `ssh -L`: `localhost:local_port` reaches `target` as seen from the server.
    Local {
        local_port: Port,
        target_address: HostAddress,
        target_port: Port,
    },
    /// `ssh -D`: a SOCKS5 proxy on `localhost:local_port` that goes out through the server.
    Dynamic { local_port: Port },
}

impl TunnelKind {
    #[must_use]
    pub fn local_port(&self) -> Port {
        match self {
            Self::Local { local_port, .. } | Self::Dynamic { local_port } => *local_port,
        }
    }
}

/// A saved forwarding rule, through one saved host.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tunnel {
    pub id: TunnelId,
    pub name: TunnelName,
    pub host_id: HostId,
    pub kind: TunnelKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_trimmed_and_required() {
        assert_eq!(
            TunnelName::parse(" Postgres ").unwrap().as_str(),
            "Postgres"
        );
        assert_eq!(TunnelName::parse(""), Err(TunnelsError::EmptyName));
    }

    #[test]
    fn both_kinds_tell_their_local_port() {
        let port = Port::try_from(5432).unwrap();
        let local = TunnelKind::Local {
            local_port: port,
            target_address: HostAddress::parse("db.internal").unwrap(),
            target_port: port,
        };
        assert_eq!(local.local_port(), port);
        assert_eq!(TunnelKind::Dynamic { local_port: port }.local_port(), port);
    }
}
