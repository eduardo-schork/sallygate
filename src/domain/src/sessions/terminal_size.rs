use super::SessionsError;

/// How many character columns and rows the terminal shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalSize {
    columns: u16,
    rows: u16,
}

impl TerminalSize {
    /// # Errors
    ///
    /// Returns an error when either dimension is zero.
    pub fn new(columns: u16, rows: u16) -> Result<Self, SessionsError> {
        if columns == 0 || rows == 0 {
            return Err(SessionsError::EmptyTerminalSize);
        }
        Ok(Self { columns, rows })
    }

    #[must_use]
    pub fn columns(self) -> u16 {
        self.columns
    }

    #[must_use]
    pub fn rows(self) -> u16 {
        self.rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_columns_and_rows() {
        let size = TerminalSize::new(120, 40).unwrap();
        assert_eq!((size.columns(), size.rows()), (120, 40));
    }

    #[test]
    fn rejects_a_zero_dimension() {
        assert_eq!(
            TerminalSize::new(0, 40),
            Err(SessionsError::EmptyTerminalSize)
        );
        assert_eq!(
            TerminalSize::new(80, 0),
            Err(SessionsError::EmptyTerminalSize)
        );
    }
}
