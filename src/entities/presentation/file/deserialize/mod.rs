mod field;
mod field_visitor;
mod file_visitor;

use self::file_visitor::FileVisitor;
use entities::models::File;
use entities::presentation::FromJson;
use serde::de::{Deserialize, Deserializer};

const FIELDS: &[&str] = &["file_id", "name", "folder_id", "public", "extension"];

impl<'de> Deserialize<'de> for File {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("File", FIELDS, FileVisitor)
    }
}

impl<'a> FromJson<'a> for File {}
