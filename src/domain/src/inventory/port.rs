use std::fmt;

use super::InventoryError;

/// The TCP port an SSH server listens on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Port(u16);

impl Port {
    /// The standard SSH port.
    pub const SSH_DEFAULT: Self = Self(22);

    #[must_use]
    pub fn value(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for Port {
    type Error = InventoryError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            return Err(InventoryError::ZeroPort);
        }
        Ok(Self(value))
    }
}

impl Default for Port {
    fn default() -> Self {
        Self::SSH_DEFAULT
    }
}

impl fmt::Display for Port {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_the_standard_ssh_port() {
        assert_eq!(Port::default().value(), 22);
    }

    #[test]
    fn accepts_any_non_zero_port() {
        assert_eq!(Port::try_from(2222).unwrap().value(), 2222);
    }

    #[test]
    fn rejects_port_zero() {
        assert_eq!(Port::try_from(0), Err(InventoryError::ZeroPort));
    }
}
