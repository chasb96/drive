use super::field_visitor::FieldVisitor;
use serde::de::{Deserialize, Deserializer};

pub enum Field {
    UserId,
    Email,
    Name,
    Role,
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(FieldVisitor)
    }
}
