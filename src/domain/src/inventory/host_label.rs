use std::fmt;

use super::InventoryError;

/// The name the user gives a host, like `oracle-vps`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HostLabel(String);

impl HostLabel {
    /// Long enough for a descriptive name, short enough to fit a card.
    pub const MAX_LENGTH: usize = 64;

    /// # Errors
    ///
    /// Returns an error when the label is empty or too long.
    pub fn parse(label: &str) -> Result<Self, InventoryError> {
        let label = label.trim();

        if label.is_empty() {
            return Err(InventoryError::EmptyHostLabel);
        }
        if label.chars().count() > Self::MAX_LENGTH {
            return Err(InventoryError::HostLabelTooLong {
                max: Self::MAX_LENGTH,
            });
        }

        Ok(Self(label.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HostLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_and_trims_a_label_with_spaces_inside() {
        assert_eq!(
            HostLabel::parse(" Home server ").unwrap().as_str(),
            "Home server"
        );
    }

    #[test]
    fn rejects_an_empty_label() {
        assert_eq!(HostLabel::parse(" "), Err(InventoryError::EmptyHostLabel));
    }

    #[test]
    fn rejects_a_label_that_is_too_long() {
        let too_long = "a".repeat(HostLabel::MAX_LENGTH + 1);
        assert_eq!(
            HostLabel::parse(&too_long),
            Err(InventoryError::HostLabelTooLong {
                max: HostLabel::MAX_LENGTH
            })
        );
    }
}
