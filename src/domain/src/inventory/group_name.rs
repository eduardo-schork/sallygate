use std::fmt;

use super::InventoryError;

/// The group a host is filed under, like `Cloud` or `Home`. Hosts with the same group name
/// are shown together; a group exists as long as one host uses it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroupName(String);

impl GroupName {
    pub const MAX_LENGTH: usize = 40;

    /// # Errors
    ///
    /// Returns an error when the name is empty or too long.
    pub fn parse(name: &str) -> Result<Self, InventoryError> {
        let name = name.trim();

        if name.is_empty() {
            return Err(InventoryError::EmptyGroupName);
        }
        if name.chars().count() > Self::MAX_LENGTH {
            return Err(InventoryError::GroupNameTooLong {
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

impl fmt::Display for GroupName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_and_trims_a_name() {
        assert_eq!(GroupName::parse(" Cloud ").unwrap().as_str(), "Cloud");
    }

    #[test]
    fn rejects_an_empty_name() {
        assert_eq!(GroupName::parse(""), Err(InventoryError::EmptyGroupName));
    }

    #[test]
    fn rejects_a_name_that_is_too_long() {
        let too_long = "a".repeat(GroupName::MAX_LENGTH + 1);
        assert_eq!(
            GroupName::parse(&too_long),
            Err(InventoryError::GroupNameTooLong {
                max: GroupName::MAX_LENGTH
            })
        );
    }
}
