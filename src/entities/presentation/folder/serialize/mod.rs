use crate::entities::models::Folder;
use crate::entities::presentation::ToJson;
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for Folder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Folder", 5)?;

        state.serialize_field("folder_id", &self.id())?;
        state.serialize_field("name", &self.name())?;
        state.serialize_field("parent_id", &self.parent_id())?;
        state.serialize_field("user_id", &self.user_id())?;
        state.serialize_field("created_at", &self.created_at())?;
        state.serialize_field("updated_at", &self.updated_at())?;

        state.end()
    }
}

impl ToJson for Folder {}
