use crate::entities::models::{File, User};

pub trait FileAuthorizer {
    fn can_create(&self, user: &User) -> bool;

    fn can_view(&self, user: &User, file: &File) -> bool;

    fn can_modify(&self, user: &User, file: &File) -> bool;

    fn can_delete(&self, user: &User, file: &File) -> bool;
}

pub struct Authorizer;

impl Authorizer {
    pub fn new() -> Self {
        Self
    }
}

impl FileAuthorizer for Authorizer {
    fn can_create(&self, _user: &User) -> bool {
        true
    }

    fn can_view(&self, user: &User, file: &File) -> bool {
        let folder = match file.folder() {
            Ok(folder) => folder,
            Err(_) => return false,
        };

        file.public() || folder.user_id() == user.id()
    }

    fn can_modify(&self, user: &User, file: &File) -> bool {
        let folder = match file.folder() {
            Ok(folder) => folder,
            Err(_) => return false,
        };

        folder.user_id() == user.id()
    }

    fn can_delete(&self, user: &User, file: &File) -> bool {
        let folder = match file.folder() {
            Ok(folder) => folder,
            Err(_) => return false,
        };

        folder.user_id() == user.id()
    }
}
