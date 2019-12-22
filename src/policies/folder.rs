use crate::entities::models::{Folder, User};

pub trait FolderAuthorizer {
    fn can_index(&self, user: &User) -> bool;

    fn can_create(&self, user: &User) -> bool;

    fn can_view(&self, user: &User, folder: &Folder) -> bool;

    fn can_modify(&self, user: &User, folder: &Folder) -> bool;

    fn can_delete(&self, user: &User, folder: &Folder) -> bool;
}

pub struct Authorizer;

impl Authorizer {
    pub fn new() -> Self {
        Self
    }
}

impl FolderAuthorizer for Authorizer {
    fn can_index(&self, _user: &User) -> bool {
        true
    }

    fn can_create(&self, _user: &User) -> bool {
        true
    }

    fn can_view(&self, user: &User, folder: &Folder) -> bool {
        folder.user_id() == user.id()
    }

    fn can_modify(&self, user: &User, folder: &Folder) -> bool {
        folder.user_id() == user.id()
    }

    fn can_delete(&self, user: &User, folder: &Folder) -> bool {
        folder.user_id() == user.id()
    }
}
