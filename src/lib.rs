mod de;
mod error;
mod frontmatter;
mod parser;
mod ser;
mod value;

pub use error::{ParseError, Result};
pub use frontmatter::{is_frontmatter, parse_skill, parse_skills, to_skill_string};
pub use value::Skill;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const NAME: &str = env!("CARGO_PKG_NAME");
