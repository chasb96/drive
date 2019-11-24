use super::field::Field;
use entities::builders::{Builder, UserBuilder};
use entities::models::User;
use serde::de::{self, MapAccess, Visitor};
use std::fmt;

pub struct UserVisitor;

impl<'de> Visitor<'de> for UserVisitor {
    type Value = User;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct User")
    }

    fn visit_map<V>(self, mut map: V) -> Result<User, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut user_id = None;
        let mut email: Option<String> = None;
        let mut name = None;
        let mut role = None;

        while let Some(key) = map.next_key()? {
            match key {
                Field::UserId => {
                    if user_id.is_some() {
                        return Err(de::Error::duplicate_field("user_id"));
                    }

                    user_id = Some(map.next_value()?);
                }
                Field::Email => {
                    if email.is_some() {
                        return Err(de::Error::duplicate_field("email"));
                    }

                    email = Some(map.next_value()?);
                }
                Field::Name => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }

                    name = Some(map.next_value()?);
                }
                Field::Role => {
                    if role.is_some() {
                        return Err(de::Error::duplicate_field("role"));
                    }

                    role = Some(map.next_value()?);
                }
            }
        }

        let user_id = user_id.ok_or_else(|| de::Error::missing_field("user_id"))?;
        let email = email.ok_or_else(|| de::Error::missing_field("email"))?;
        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let role = role.unwrap_or_else(|| String::from("user"));

        let user = UserBuilder::new()
            .with_id(user_id)
            .with_email(email)
            .with_name(name)
            .with_role(role)
            .build();

        Ok(user)
    }
}
