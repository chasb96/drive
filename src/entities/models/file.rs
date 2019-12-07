use chrono::NaiveDateTime;
use entities::models::Model;
use schema::*;

#[derive(Clone, Debug, PartialEq, Queryable, Associations, Identifiable)]
pub struct File {
    id: i32,
    name: String,
    file_name: String,
    folder_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    extension: String,
    public: bool,
}

impl Default for File {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::default(),
            file_name: String::default(),
            folder_id: 0,
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
            extension: String::default(),
            public: false,
        }
    }
}

#[allow(dead_code)]
impl File {
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

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name
    }

    pub fn folder_id(&self) -> i32 {
        self.folder_id
    }

    pub fn set_folder_id(&mut self, folder_id: i32) {
        self.folder_id = folder_id
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    pub fn extension(&self) -> &String {
        &self.extension
    }

    pub fn set_extension(&mut self, extension: String) {
        self.extension = extension
    }

    pub fn public(&self) -> bool {
        self.public
    }

    pub fn set_public(&mut self, public: bool) {
        self.public = public
    }
}

impl Model for File {}
