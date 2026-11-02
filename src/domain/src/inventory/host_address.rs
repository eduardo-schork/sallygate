use std::fmt;

use super::InventoryError;

/// A hostname or IP address a server can be reached at.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HostAddress(String);

impl HostAddress {
    /// The longest hostname DNS allows.
    pub const MAX_LENGTH: usize = 253;

    /// Validates a hostname or IP address typed by the user.
    ///
    /// # Errors
    ///
    /// Returns an error when the address is empty, has whitespace inside, or is too long.
    pub fn parse(address: &str) -> Result<Self, InventoryError> {
        let address = address.trim();

        if address.is_empty() {
            return Err(InventoryError::EmptyHostAddress);
        }
        if address.chars().any(char::is_whitespace) {
            return Err(InventoryError::HostAddressWithWhitespace(
                address.to_owned(),
            ));
        }
        if address.len() > Self::MAX_LENGTH {
            return Err(InventoryError::HostAddressTooLong {
                max: Self::MAX_LENGTH,
            });
        }

        Ok(Self(address.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HostAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_hostnames_and_ip_addresses_and_trims_them() {
        assert_eq!(
            HostAddress::parse(" example.com ").unwrap().as_str(),
            "example.com"
        );
        assert_eq!(HostAddress::parse("10.0.0.5").unwrap().as_str(), "10.0.0.5");
        assert_eq!(HostAddress::parse("::1").unwrap().as_str(), "::1");
    }

    #[test]
    fn rejects_an_empty_address() {
        assert_eq!(
            HostAddress::parse("   "),
            Err(InventoryError::EmptyHostAddress)
        );
    }

    #[test]
    fn rejects_whitespace_inside_the_address() {
        assert!(matches!(
            HostAddress::parse("my host"),
            Err(InventoryError::HostAddressWithWhitespace(_))
        ));
    }

    #[test]
    fn rejects_an_address_longer_than_dns_allows() {
        let too_long = "a".repeat(HostAddress::MAX_LENGTH + 1);
        assert_eq!(
            HostAddress::parse(&too_long),
            Err(InventoryError::HostAddressTooLong {
                max: HostAddress::MAX_LENGTH
            })
        );
    }
}
