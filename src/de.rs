use serde::{Deserialize, Deserializer};

use crate::frontmatter::parse_skill;
use crate::value::Skill;

impl<'de> Deserialize<'de> for Skill {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SkillVisitor;

        impl<'de> serde::de::Visitor<'de> for SkillVisitor {
            type Value = Skill;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Skill with name, description, and content")
            }

            fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // 支持从 frontmatter 字符串解析
                parse_skill(v).map_err(serde::de::Error::custom)
            }

            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut name = None;
                let mut description = None;
                let mut content = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "name" => {
                            if name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        "description" => {
                            if description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description = Some(map.next_value()?);
                        }
                        "content" => {
                            if content.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content = Some(map.next_value()?);
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                Ok(Skill {
                    name: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    description: description
                        .ok_or_else(|| serde::de::Error::missing_field("description"))?,
                    content: content.ok_or_else(|| serde::de::Error::missing_field("content"))?,
                })
            }
        }

        // 先尝试解析为 frontmatter 字符串，再尝试为对象
        deserializer.deserialize_any(SkillVisitor)
    }
}
