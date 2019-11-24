use super::field::Field;
use super::FIELDS;
use serde::de::{self, Visitor};
use std::fmt;

pub struct FieldVisitor;

impl<'de> Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("`file_id`, `name`, `folder_id`, `public`, `extension`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Field, E>
    where
        E: de::Error,
    {
        match value {
            "file_id" => Ok(Field::FileId),
            "name" => Ok(Field::Name),
            "folder_id" => Ok(Field::FolderId),
            "public" => Ok(Field::Public),
            "extension" => Ok(Field::Extension),
            _ => Err(de::Error::unknown_field(value, FIELDS)),
        }
    }
}
