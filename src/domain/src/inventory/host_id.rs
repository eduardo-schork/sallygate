use std::fmt;
use std::str::FromStr;

use uuid::Uuid;

use super::InventoryError;

/// Identifies a saved host for its whole life, even when its label or address changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HostId(Uuid);

impl HostId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for HostId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for HostId {
    type Err = InventoryError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| InventoryError::InvalidHostId(value.to_owned()))
    }
}

impl fmt::Display for HostId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_its_text_form() {
        let host_id = HostId::new();
        assert_eq!(host_id.to_string().parse::<HostId>(), Ok(host_id));
    }

    #[test]
    fn rejects_text_that_is_not_an_id() {
        assert!(matches!(
            "nope".parse::<HostId>(),
            Err(InventoryError::InvalidHostId(_))
        ));
    }
}
