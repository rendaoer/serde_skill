// src/error.rs

//! Error types for the serde_skill library.

use std::fmt;

/// Errors that can occur during parsing or serialization.
#[derive(Debug)]
pub enum ParseError {
    /// Missing required field `name`
    MissingName,
    /// Missing required field `description`
    MissingDescription,
    /// Missing content after frontmatter delimiter
    MissingContent,
    /// Invalid format with additional context
    InvalidFormat(String),
    /// Serde serialization/deserialization error
    SerdeError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingName => write!(f, "Missing 'name' field"),
            ParseError::MissingDescription => write!(f, "Missing 'description' field"),
            ParseError::MissingContent => write!(f, "Missing content after frontmatter"),
            ParseError::InvalidFormat(e) => write!(f, "Invalid format: {}", e),
            ParseError::SerdeError(e) => write!(f, "Serde error: {}", e),
        }
    }
}

impl std::error::Error for ParseError {}

impl serde::ser::Error for ParseError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ParseError::SerdeError(msg.to_string())
    }
}

impl serde::de::Error for ParseError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ParseError::SerdeError(msg.to_string())
    }
}

/// Result type alias for parse operations.
pub type Result<T> = std::result::Result<T, ParseError>;
