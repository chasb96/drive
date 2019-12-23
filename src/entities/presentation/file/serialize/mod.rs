use crate::entities::models::File;
use crate::entities::presentation::ToJson;
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for File {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("File", 6)?;

        state.serialize_field("file_id", &self.id())?;
        state.serialize_field("name", &self.name())?;
        state.serialize_field("folder_id", &self.folder_id())?;
        state.serialize_field("public", &self.public())?;
        state.serialize_field("extension", &self.extension())?;
        state.serialize_field("created_at", &self.created_at())?;
        state.serialize_field("updated_at", &self.updated_at())?;

        state.end()
    }
}

impl ToJson for File {}
