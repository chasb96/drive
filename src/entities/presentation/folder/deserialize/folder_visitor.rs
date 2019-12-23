use super::field::Field;
use crate::entities::builders::{Builder, FolderBuilder};
use crate::entities::models::Folder;
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

pub struct FolderVisitor;

impl<'de> Visitor<'de> for FolderVisitor {
    type Value = Folder;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Folder")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Folder, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut folder_id = None;
        let mut name = None;
        let mut parent_id = None;
        let mut user_id = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::FolderId => {
                    if folder_id.is_some() {
                        return Err(de::Error::duplicate_field("folder_id"));
                    }

                    folder_id = Some(map.next_value()?);
                }
                Field::Name => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }

                    name = Some(map.next_value()?);
                }
                Field::ParentId => {
                    if parent_id.is_some() {
                        return Err(de::Error::duplicate_field("parent_id"));
                    }

                    parent_id = Some(map.next_value()?);
                }
                Field::UserId => {
                    if user_id.is_some() {
                        return Err(de::Error::duplicate_field("user_id"));
                    }

                    user_id = Some(map.next_value()?);
                }
            }
        }

        let folder_id = folder_id.ok_or_else(|| de::Error::missing_field("folder_id"))?;
        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let parent_id = parent_id.ok_or_else(|| de::Error::missing_field("parent_id"))?;
        let user_id = user_id.ok_or_else(|| de::Error::missing_field("user_id"))?;

        let folder = FolderBuilder::new()
            .with_id(folder_id)
            .with_name(name)
            .with_parent_id(parent_id)
            .with_user_id(user_id)
            .build();

        Ok(folder)
    }
}
