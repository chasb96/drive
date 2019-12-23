use crate::entities::builders::Builder;
use crate::entities::models::Folder;

pub struct FolderBuilder {
    id: i32,
    name: String,
    parent_id: Option<i32>,
    user_id: i32,
}

impl Default for FolderBuilder {
    fn default() -> Self {
        Self {
            id: i32::default(),
            name: String::default(),
            parent_id: None,
            user_id: 0,
        }
    }
}

#[allow(dead_code)]
impl FolderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;

        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub fn with_parent_id(mut self, parent_id: Option<i32>) -> Self {
        self.parent_id = parent_id;

        self
    }

    pub fn with_user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;

        self
    }
}

impl Builder for FolderBuilder {
    type Out = Folder;

    fn build(self) -> Self::Out {
        let mut folder = Folder::new();

        folder.set_id(self.id);
        folder.set_name(self.name);
        folder.set_parent_id(self.parent_id);
        folder.set_user_id(self.user_id);

        folder
    }
}
