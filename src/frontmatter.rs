// src/frontmatter.rs

//! Frontmatter serialization and deserialization.

use crate::error::Result;
use crate::parser::Parser;
use crate::value::Skill;

/// Serializes a `Skill` to frontmatter format.
///
/// # Example
/// ```
/// use serde_skill::{Skill, to_skill_string};
///
/// let skill = Skill::new("test", "desc", "content");
/// let output = to_skill_string(&skill);
/// assert!(output.starts_with("---"));
/// ```
pub fn to_skill_string(skill: &Skill) -> String {
    format!(
        "---\nname: {}\ndescription: {}\n---\n\n{}",
        skill.name, skill.description, skill.content
    )
}

/// Parses a single `Skill` from frontmatter format.
///
/// # Example
/// ```
/// use serde_skill::parse_skill;
///
/// let input = r#"---
/// name: test
/// description: desc
/// ---
/// content
/// "#;
///
/// let skill = parse_skill(input).unwrap();
/// assert_eq!(skill.name, "test");
/// ```
pub fn parse_skill(input: &str) -> Result<Skill> {
    let mut parser = Parser::new(input);
    parser.parse()
}

/// Parses multiple `Skill`s from frontmatter format.
///
/// # Example
/// ```
/// use serde_skill::parse_skills;
///
/// let input = r#"---
/// name: skill1
/// description: desc1
/// ---
/// content1
///
/// ---
/// name: skill2
/// description: desc2
/// ---
/// content2
/// "#;
///
/// let skills = parse_skills(input);
/// assert_eq!(skills.len(), 2);
/// ```
pub fn parse_skills(input: &str) -> Vec<Skill> {
    let mut skills = Vec::new();
    let mut current = String::new();
    let mut delim_count = 0;

    for line in input.lines() {
        let trimmed = line.trim();

        if trimmed == "---" {
            delim_count += 1;
            if delim_count % 2 == 1 {
                if !current.is_empty() {
                    if let Ok(skill) = parse_skill(&current) {
                        skills.push(skill);
                    }
                    current.clear();
                }
                current.push_str(line);
                current.push('\n');
            } else {
                current.push_str(line);
                current.push('\n');
            }
        } else {
            current.push_str(line);
            current.push('\n');
        }
    }

    if !current.is_empty()
        && let Ok(skill) = parse_skill(&current)
    {
        skills.push(skill);
    }

    skills
}

/// Checks if the input string is in frontmatter format.
///
/// # Example
/// ```
/// use serde_skill::is_frontmatter;
///
/// assert!(is_frontmatter("---\nname: test\n---\ncontent"));
/// assert!(!is_frontmatter("not frontmatter"));
/// ```
pub fn is_frontmatter(input: &str) -> bool {
    input.trim().starts_with("---")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_skill() {
        let input = r#"---
name: python_execute
description: Execute Python code
---
def hello():
    print("Hello")
"#;

        let skill = parse_skill(input).unwrap();
        assert_eq!(skill.name, "python_execute");
        assert_eq!(skill.description, "Execute Python code");
        assert_eq!(skill.content, "def hello():\n    print(\"Hello\")");
    }

    #[test]
    fn test_to_skill_string() {
        let skill = Skill::new("test", "desc", "content");
        let output = to_skill_string(&skill);
        assert!(output.contains("name: test"));
        assert!(output.contains("description: desc"));
        assert!(output.contains("content"));
    }

    #[test]
    fn test_roundtrip() {
        let original = Skill::new("roundtrip", "test", "content\nwith\nnewlines");
        let frontmatter = to_skill_string(&original);
        let parsed = parse_skill(&frontmatter).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_parse_skills() {
        let input = r#"---
name: skill1
description: first
---
content1

---
name: skill2
description: second
---
content2
"#;

        let skills = parse_skills(input);
        assert_eq!(skills.len(), 2);
        assert_eq!(skills[0].name, "skill1");
        assert_eq!(skills[1].name, "skill2");
    }

    #[test]
    fn test_is_frontmatter() {
        assert!(is_frontmatter("---\nname: test\n---\ncontent"));
        assert!(!is_frontmatter("not frontmatter"));
        assert!(!is_frontmatter(""));
    }
}
