// examples/custom_fields.rs

//! Demonstrates extending Skill with an `enabled` field
//!
//! Run with: `cargo run --example custom_fields`

use serde::{Deserialize, Serialize};
use serde_skill::{Skill, parse_skill, to_skill_string};

/// Extended skill with an `enabled` field
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExtendedSkill {
    #[serde(flatten)]
    pub base: Skill,
    pub enabled: bool,
}

impl ExtendedSkill {
    pub fn from_skill(skill: Skill) -> Self {
        Self {
            base: skill,
            enabled: true, // default value
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Custom Fields Example ===\n");

    // 1. Parse a basic skill
    println!("1. Parsing basic skill:");
    let input = r#"---
name: python_execute
description: Execute Python code
---
def hello():
    print("Hello")
"#;

    let skill = parse_skill(input)?;
    println!("   ✅ Parsed: {}", skill.name);

    // 2. Extend with enabled field
    println!("\n2. Adding 'enabled' field:");
    let mut extended = ExtendedSkill::from_skill(skill);
    println!("   ✅ Enabled: {}", extended.enabled);

    // 3. Disable the skill
    println!("\n3. Disabling the skill:");
    extended.enabled = false;
    println!("   ✅ Enabled: {}", extended.enabled);

    // 4. Serialize to JSON
    println!("\n4. Serializing to JSON:");
    let json = serde_json::to_string_pretty(&extended)?;
    println!("   ✅ JSON:\n{}", json);

    // 5. Deserialize from JSON
    println!("\n5. Deserializing from JSON:");
    let parsed: ExtendedSkill = serde_json::from_str(&json)?;
    println!("   ✅ Parsed: {}", parsed.base.name);
    println!("   ✅ Enabled: {}", parsed.enabled);

    // 6. Serialize back to frontmatter
    println!("\n6. Serializing to frontmatter:");
    let frontmatter = to_skill_string(&extended.base);
    println!("   ✅ Frontmatter:\n{}", frontmatter);

    println!("\n=== Custom fields example completed! ===");
    Ok(())
}
