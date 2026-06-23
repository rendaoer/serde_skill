//! # serde_skill
//!
//! A lightweight frontmatter + content parser for Skill format.
//!
//! Supports parsing and serializing skills in the following format:
//!
//! ```text
//! ---
//! name: skill_name
//! description: skill_description
//! ---
//!
//! skill content here
//! ```
//!
//! # Features
//!
//! - Parse `---` delimited frontmatter
//! - Batch parsing support
//! - Bidirectional conversion
//! - Serde compatible
//! - Zero extra dependencies (only serde)
//!
//! # Quick Start
//!
//! ```
//! use serde_skill::{parse_skill, to_skill_string, Skill};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let input = r#"---
//! name: python_execute
//! description: Execute Python code
//! ---
//! def hello():
//!     print("Hello")
//! "#;
//!
//! let skill = parse_skill(input)?;
//! assert_eq!(skill.name, "python_execute");
//!
//! let output = to_skill_string(&skill);
//! assert!(output.starts_with("---"));
//! # Ok(())
//! # }
//! ```

/// A skill with name, description, and content.
///
/// # Fields
/// - `name`: Skill identifier
/// - `description`: Brief description of the skill
/// - `content`: Implementation code or documentation
///
/// # Serde Support
///
/// `Skill` implements `Serialize` and `Deserialize`, enabling:
/// - JSON serialization/deserialization
/// - YAML, TOML, and other serde-compatible formats
///
/// # Example
/// ```
/// use serde_skill::Skill;
///
/// let skill = Skill::new("python_execute", "Execute Python code", "def run(): pass");
/// assert_eq!(skill.name(), "python_execute");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    /// Skill name (unique identifier)
    pub name: String,

    /// Brief description of the skill
    pub description: String,

    /// Skill content (code, config, or documentation)
    pub content: String,
}

impl Skill {
    /// Creates a new `Skill` instance.
    ///
    /// # Example
    /// ```
    /// # use serde_skill::Skill;
    /// let skill = Skill::new("test", "desc", "content");
    /// assert_eq!(skill.name, "test");
    /// ```
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            content: content.into(),
        }
    }

    /// Returns the skill name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the skill description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the skill content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }
}
