use super::field::Field;
use super::FIELDS;
use serde::de::{self, Visitor};
use std::fmt;

pub struct FieldVisitor;

impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("`folder_id`, `name`, `parent_id`, `user_id`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Field, E>
    where
        E: de::Error,
    {
        match value {
            "folder_id" => Ok(Field::FolderId),
            "name" => Ok(Field::Name),
            "parent_id" => Ok(Field::ParentId),
            "user_id" => Ok(Field::UserId),
            _ => Err(de::Error::unknown_field(value, FIELDS)),
        }
    }
}
