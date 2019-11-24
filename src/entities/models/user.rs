use super::Model;
use bcrypt::verify;
use chrono::NaiveDateTime;
use schema::*;

#[derive(Clone, Debug, PartialEq, Queryable, Associations, Identifiable)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    root: Option<i32>,
    role: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::default(),
            email: String::default(),
            password: String::default(),
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
            root: None,
            role: "user".to_string(),
        }
    }
}

#[allow(dead_code)]
impl User {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    pub fn root(&self) -> &Option<i32> {
        &self.root
    }

    pub fn set_root(&mut self, root: Option<i32>) {
        self.root = root
    }

    pub fn role(&self) -> &String {
        &self.role
    }

    pub fn set_role(&mut self, role: String) {
        self.role = role
    }

    pub fn password_check(&self, password: &String) -> bool {
        match verify(password, &self.password) {
            Ok(true) => true,
            _ => false,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}

impl Model for User {}
