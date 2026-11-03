use std::fmt;
use std::str::FromStr;

use uuid::Uuid;

use super::SnippetsError;

/// Identifies a saved snippet for its whole life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SnippetId(Uuid);

impl SnippetId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SnippetId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for SnippetId {
    type Err = SnippetsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| SnippetsError::InvalidSnippetId(value.to_owned()))
    }
}

impl fmt::Display for SnippetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// What the user calls a snippet, like `Restart nginx`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetName(String);

impl SnippetName {
    pub const MAX_LENGTH: usize = 64;

    /// # Errors
    ///
    /// Returns an error when the name is empty or too long.
    pub fn parse(name: &str) -> Result<Self, SnippetsError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(SnippetsError::EmptyName);
        }
        if name.chars().count() > Self::MAX_LENGTH {
            return Err(SnippetsError::NameTooLong {
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

/// The text sent to the shell. It may span lines; surrounding blank space is dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetCommand(String);

impl SnippetCommand {
    pub const MAX_LENGTH: usize = 4096;

    /// # Errors
    ///
    /// Returns an error when the command is empty or too long.
    pub fn parse(command: &str) -> Result<Self, SnippetsError> {
        let command = command.trim();
        if command.is_empty() {
            return Err(SnippetsError::EmptyCommand);
        }
        if command.chars().count() > Self::MAX_LENGTH {
            return Err(SnippetsError::CommandTooLong {
                max: Self::MAX_LENGTH,
            });
        }
        Ok(Self(command.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A saved command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snippet {
    pub id: SnippetId,
    pub name: SnippetName,
    pub command: SnippetCommand,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_names_and_commands_but_keeps_lines_inside() {
        assert_eq!(SnippetName::parse(" Logs ").unwrap().as_str(), "Logs");
        assert_eq!(
            SnippetCommand::parse("\ncd /srv\ndocker compose up -d\n")
                .unwrap()
                .as_str(),
            "cd /srv\ndocker compose up -d"
        );
    }

    #[test]
    fn rejects_empty_or_oversized_values() {
        assert_eq!(SnippetName::parse(" "), Err(SnippetsError::EmptyName));
        assert_eq!(SnippetCommand::parse(""), Err(SnippetsError::EmptyCommand));
        assert_eq!(
            SnippetCommand::parse(&"x".repeat(SnippetCommand::MAX_LENGTH + 1)),
            Err(SnippetsError::CommandTooLong {
                max: SnippetCommand::MAX_LENGTH
            })
        );
    }

    #[test]
    fn ids_round_trip_through_text() {
        let id = SnippetId::new();
        assert_eq!(id.to_string().parse::<SnippetId>(), Ok(id));
        assert!("nope".parse::<SnippetId>().is_err());
    }
}
