use std::fmt;

use super::InventoryError;

/// The account name used to log in to a server.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    /// Validates a username typed by the user.
    ///
    /// # Errors
    ///
    /// Returns an error when the username is empty or has whitespace inside.
    pub fn parse(username: &str) -> Result<Self, InventoryError> {
        let username = username.trim();

        if username.is_empty() {
            return Err(InventoryError::EmptyUsername);
        }
        if username.chars().any(char::is_whitespace) {
            return Err(InventoryError::UsernameWithWhitespace(username.to_owned()));
        }

        Ok(Self(username.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Username {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_and_trims_a_username() {
        assert_eq!(Username::parse(" ubuntu ").unwrap().as_str(), "ubuntu");
    }

    #[test]
    fn rejects_an_empty_username() {
        assert_eq!(Username::parse(""), Err(InventoryError::EmptyUsername));
    }

    #[test]
    fn rejects_whitespace_inside_the_username() {
        assert!(matches!(
            Username::parse("john doe"),
            Err(InventoryError::UsernameWithWhitespace(_))
        ));
    }
}
