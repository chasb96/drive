use super::field::Field;
use entities::builders::{Builder, FileBuilder};
use entities::models::File;
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

pub struct FileVisitor;

impl<'de> Visitor<'de> for FileVisitor {
    type Value = File;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct File")
    }

    fn visit_map<V>(self, mut map: V) -> Result<File, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut file_id = None;
        let mut name = None;
        let mut folder_id = None;
        let mut public = None;
        let mut extension = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::FileId => {
                    if file_id.is_some() {
                        return Err(de::Error::duplicate_field("file_id"));
                    }

                    file_id = Some(map.next_value()?);
                }
                Field::Name => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }

                    name = Some(map.next_value()?);
                }
                Field::FolderId => {
                    if folder_id.is_some() {
                        return Err(de::Error::duplicate_field("folder_id"));
                    }

                    folder_id = Some(map.next_value()?);
                }
                Field::Public => {
                    if public.is_some() {
                        return Err(de::Error::duplicate_field("public"));
                    }

                    public = Some(map.next_value()?);
                }
                Field::Extension => {
                    if extension.is_some() {
                        return Err(de::Error::duplicate_field("extension"));
                    }

                    extension = Some(map.next_value()?);
                }
            }
        }

        let file_id = file_id.ok_or_else(|| de::Error::missing_field("file_id"))?;
        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let folder_id = folder_id.ok_or_else(|| de::Error::missing_field("folder_id"))?;
        let public = public.or(Some(false)).unwrap();
        let extension = extension.ok_or_else(|| de::Error::missing_field("extension"))?;

        let user = FileBuilder::new()
            .with_id(file_id)
            .with_name(name)
            .with_folder_id(folder_id)
            .with_public(public)
            .with_extension(extension)
            .build();

        Ok(user)
    }
}
