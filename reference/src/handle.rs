//! Handle - Human-readable @usernames
//!
//! Handles are human-readable identifiers bound to Human Identities.

use crate::error::{Error, Result};
use std::fmt;

/// Handle format: @[a-z0-9_]{1,20}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Handle(String);

impl Handle {
    /// Create a new handle (validates format)
    pub fn new(name: &str) -> Result<Self> {
        let name = name.trim_start_matches('@').to_lowercase();
        
        if name.is_empty() || name.len() > 20 {
            return Err(Error::InvalidHandle("length must be 1-20".into()));
        }
        
        if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return Err(Error::InvalidHandle("only a-z, 0-9, _ allowed".into()));
        }
        
        Ok(Self(name))
    }

    /// Get the raw name (without @)
    pub fn name(&self) -> &str {
        &self.0
    }

    /// Get display format (with @)
    pub fn display(&self) -> String {
        format!("@{}", self.0)
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handle({})", self.display())
    }
}

impl fmt::Display for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_handles() {
        assert!(Handle::new("alice").is_ok());
        assert!(Handle::new("@bob").is_ok());
        assert!(Handle::new("user_123").is_ok());
        assert!(Handle::new("a").is_ok());
        assert!(Handle::new("12345678901234567890").is_ok());
    }

    #[test]
    fn test_invalid_handles() {
        assert!(Handle::new("").is_err());
        assert!(Handle::new("@").is_err());
        assert!(Handle::new("Alice").is_ok()); // Normalized to lowercase
        assert!(Handle::new("user-name").is_err());
        assert!(Handle::new("123456789012345678901").is_err()); // Too long
    }
}
