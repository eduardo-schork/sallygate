use std::fmt;

use super::{HostAddress, Port, Username};

/// Where to connect and as whom: `username@address:port`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SshEndpoint {
    address: HostAddress,
    port: Port,
    username: Username,
}

impl SshEndpoint {
    #[must_use]
    pub fn new(address: HostAddress, port: Port, username: Username) -> Self {
        Self {
            address,
            port,
            username,
        }
    }

    #[must_use]
    pub fn address(&self) -> &HostAddress {
        &self.address
    }

    #[must_use]
    pub fn port(&self) -> Port {
        self.port
    }

    #[must_use]
    pub fn username(&self) -> &Username {
        &self.username
    }
}

impl fmt::Display for SshEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}@{}:{}",
            self.username, self.address, self.port
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_as_username_at_address_and_port() {
        let endpoint = SshEndpoint::new(
            HostAddress::parse("example.com").unwrap(),
            Port::try_from(2222).unwrap(),
            Username::parse("ubuntu").unwrap(),
        );

        assert_eq!(endpoint.to_string(), "ubuntu@example.com:2222");
    }
}
