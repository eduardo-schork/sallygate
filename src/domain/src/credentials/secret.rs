use std::fmt;

use zeroize::Zeroizing;

/// A password or key passphrase. It never prints and is wiped from memory when dropped.
pub struct Secret(Zeroizing<String>);

impl Secret {
    #[must_use]
    pub fn new(value: String) -> Self {
        Self(Zeroizing::new(value))
    }

    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Secret([redacted])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_prints_its_value() {
        let secret = Secret::new("hunter2".to_owned());
        assert!(!format!("{secret:?}").contains("hunter2"));
        assert_eq!(secret.expose(), "hunter2");
    }
}
