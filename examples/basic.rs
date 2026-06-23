// examples/basic.rs

//! Basic usage example for serde_skill
//!
//! Run with: `cargo run --example basic`

use serde_skill::{Skill, parse_skill, parse_skills, to_skill_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== serde_skill Basic Example ===\n");

    // 1. Parse a single skill
    println!("1. Parsing a single skill:");
    let input = r#"---
name: python_execute
description: Execute Python code safely
---
def run_python(code):
    try:
        result = subprocess.run(
        ['python3', '-c', code],
        capture_output=True,
        text=True,
        timeout=5
        )
        return result.stdout if result.stdout else result.stderr
    except Exception as e:
        return str(e)
"#;

    let skill = parse_skill(input)?;
    println!("   ✅ Parsed skill:");
    println!("   Name: {}", skill.name);
    println!("   Description: {}", skill.description);
    println!("   Content preview: {}...", &skill.content[..50]);

    // 2. Serialize a skill
    println!("\n2. Serializing a skill:");
    let new_skill = Skill::new(
        "bash_execute",
        "Execute bash commands",
        "#!/bin/bash\necho 'Hello, World!'",
    );
    let frontmatter = to_skill_string(&new_skill);
    println!("   ✅ Serialized to frontmatter:\n{}", frontmatter);

    // 3. Parse multiple skills
    println!("\n3. Parsing multiple skills:");
    let multi_input = r#"---
name: skill1
description: First skill
---
Content of skill 1

---
name: skill2
description: Second skill
---
Content of skill 2

---
name: skill3
description: Third skill
---
Content of skill 3
"#;

    let skills = parse_skills(multi_input);
    println!("   ✅ Parsed {} skills:", skills.len());
    for (i, s) in skills.iter().enumerate() {
        println!("   {}. {} - {}", i + 1, s.name, s.description);
    }

    // 4. Roundtrip test
    println!("\n4. Roundtrip test:");
    let original = Skill::new(
        "roundtrip_test",
        "Testing roundtrip",
        "This content should survive the roundtrip",
    );
    let serialized = to_skill_string(&original);
    let parsed = parse_skill(&serialized)?;
    assert_eq!(original, parsed);
    println!("   ✅ Roundtrip successful! Original == Parsed");

    // 5. Error handling
    println!("\n5. Error handling:");
    let invalid_input = r#"---
name: only_name
---
no description or content
"#;

    match parse_skill(invalid_input) {
        Ok(_) => println!("   ❌ Unexpected success"),
        Err(e) => println!("   ✅ Caught expected error: {}", e),
    }

    println!("\n=== Example completed successfully! ===");
    Ok(())
}
