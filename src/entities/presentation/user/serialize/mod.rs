use crate::entities::models::User;
use crate::entities::presentation::ToJson;
use serde::ser::{Serialize, SerializeStruct, Serializer};

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 7)?;

        state.serialize_field("user_id", &self.id())?;
        state.serialize_field("email", &self.email())?;
        state.serialize_field("name", &self.name())?;
        state.serialize_field("created_at", &self.created_at())?;
        state.serialize_field("updated_at", &self.updated_at())?;
        state.serialize_field("role", &self.role())?;
        state.serialize_field("is_admin", &self.is_admin())?;

        state.end()
    }
}

impl ToJson for User {}
