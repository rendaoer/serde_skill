use serde::{Serialize, Serializer};

use crate::value::Skill;

impl Serialize for Skill {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Skill", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("content", &self.content)?;
        state.end()
    }
}
