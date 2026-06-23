# serde_skill

[![Crates.io](https://img.shields.io/crates/v/serde_skill.svg)](https://crates.io/crates/serde_skill)
[![Docs.rs](https://docs.rs/serde_skill/badge.svg)](https://docs.rs/serde_skill)

A lightweight, serde-compatible library for parsing and serializing skills in frontmatter format.

## Overview

`serde_skill` parses skills defined with YAML frontmatter and content body:

```text
---
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
```

## Features

- ✅ Parse `---` delimited frontmatter
- ✅ Serialize skills back to frontmatter format
- ✅ Batch parsing for multiple skills
- ✅ Serde compatibility (Serialize/Deserialize)
- ✅ Zero extra dependencies (only serde)
- ✅ No unsafe code

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde_skill = "0.1.0"
```

## Quick Start

```rust
use serde_skill::{parse_skill, to_skill_string, Skill};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"---
name: python_execute
description: Execute Python code
---
def hello():
    print("Hello")
"#;

    // Parse from frontmatter
    let skill = parse_skill(input)?;
    println!("Skill: {}", skill.name);

    // Serialize back to frontmatter
    let output = to_skill_string(&skill);
    println!("{}", output);

    Ok(())
}
```

## Usage

### Parse a single skill

```rust
use serde_skill::parse_skill;

let skill = parse_skill(input)?;
assert_eq!(skill.name, "my_skill");
assert_eq!(skill.description, "My skill description");
assert_eq!(skill.content, "Skill content here");
```

### Parse multiple skills

```rust
use serde_skill::parse_skills;

let input = r#"---
name: skill1
description: First skill
---
content 1

---
name: skill2
description: Second skill
---
content 2
"#;

let skills = parse_skills(input);
assert_eq!(skills.len(), 2);
```

### Serialize a skill

```rust
use serde_skill::{to_skill_string, Skill};

let skill = Skill::new("my_skill", "My description", "My content");
let frontmatter = to_skill_string(&skill);
println!("{}", frontmatter);
```

### Serde integration

`Skill` implements `Serialize` and `Deserialize`, so you can use it with any serde-compatible format:

```rust
use serde_skill::Skill;
use serde_json;

let skill = Skill::new("test", "desc", "content");

// To JSON
let json = serde_json::to_string(&skill)?;

// From JSON
let parsed: Skill = serde_json::from_str(&json)?;
```

### Check if input is frontmatter

```rust
use serde_skill::is_frontmatter;

assert!(is_frontmatter("---\nname: test\n---\ncontent"));
assert!(!is_frontmatter("not frontmatter"));
```

## API Reference

| Function | Description |
|----------|-------------|
| `parse_skill(input: &str) -> Result<Skill>` | Parses a single skill from frontmatter |
| `parse_skills(input: &str) -> Vec<Skill>` | Parses multiple skills from frontmatter |
| `to_skill_string(skill: &Skill) -> String` | Serializes a skill to frontmatter |
| `is_frontmatter(input: &str) -> bool` | Checks if input starts with frontmatter delimiter |

## Error Handling

```rust
use serde_skill::{parse_skill, ParseError};

match parse_skill(input) {
    Ok(skill) => println!("Parsed: {}", skill.name),
    Err(ParseError::MissingName) => eprintln!("Missing 'name' field"),
    Err(ParseError::MissingDescription) => eprintln!("Missing 'description' field"),
    Err(ParseError::InvalidFormat(msg)) => eprintln!("Format error: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Format Specification

Skills are defined with frontmatter delimited by `---`:

```text
---
name: <skill_name>
description: <skill_description>
---
<skill_content>
```

- `name`: Required. Unique skill identifier.
- `description`: Required. Brief description of the skill.
- `content`: Required. The skill's implementation code, documentation, or configuration.

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch
3. Add your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Links

- [Documentation](https://docs.rs/serde_skill)
- [Repository](https://github.com/yourusername/serde_skill)
- [Crates.io](https://crates.io/crates/serde_skill)

---

Made with ❤️ in Rust
