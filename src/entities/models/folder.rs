use super::Model;
use chrono::NaiveDateTime;
use schema::*;

#[derive(Clone, Debug, PartialEq, Queryable, Associations, Identifiable)]
pub struct Folder {
    id: i32,
    name: String,
    parent_id: Option<i32>,
    user_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::default(),
            parent_id: None,
            user_id: 0,
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
        }
    }
}

#[allow(dead_code)]
impl Folder {
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

    pub fn parent_id(&self) -> &Option<i32> {
        &self.parent_id
    }

    pub fn set_parent_id(&mut self, parent_id: Option<i32>) {
        self.parent_id = parent_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = user_id
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}

impl Model for Folder {}
