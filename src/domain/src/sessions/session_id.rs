use std::fmt;
use std::str::FromStr;

use uuid::Uuid;

use super::SessionsError;

/// Identifies one live session for as long as it is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(Uuid);

impl SessionId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for SessionId {
    type Err = SessionsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| SessionsError::InvalidSessionId(value.to_owned()))
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_its_text_form() {
        let session_id = SessionId::new();
        assert_eq!(session_id.to_string().parse::<SessionId>(), Ok(session_id));
    }

    #[test]
    fn new_ids_are_unique() {
        assert_ne!(SessionId::new(), SessionId::new());
    }

    #[test]
    fn rejects_text_that_is_not_an_id() {
        assert!(matches!(
            "nope".parse::<SessionId>(),
            Err(SessionsError::InvalidSessionId(_))
        ));
    }
}
