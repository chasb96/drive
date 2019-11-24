use crate::entities::traits::user::UserStore;
use crate::entities::models::{Folder, User};
use diesel::result::Error;
use crate::entities::builders::{Builder, FolderBuilder, UserBuilder};

pub struct UserStoreMock;

impl UserStoreMock {
    pub fn new() -> Self {
        Self
    }
}

impl UserStore for UserStoreMock {
    fn find_by_user_id(&self, user_id: i32) -> Result<User, Error> {
        let mut user = factory!(User);

        user.set_id(user_id);

        Ok(user)
    }

    fn save(&self, user: &User) -> Result<User, Error> {
        Ok(user.clone())
    }

    fn update(&self, user: &User) -> Result<User, Error> {
        Ok(user.clone())
    }

    fn delete(&self, user: &User) -> Result<User, Error> {
        Ok(user.clone())
    }

    fn update_password(&self, user: &User) -> Result<User, Error> {
        Ok(user.clone())
    }

    fn folders(&self, user: &User) -> Result<Vec<Folder>, Error> {
        let folders = vec![
            factory!(Folder, user.id(), None),
            factory!(Folder, user.id(), None),
            factory!(Folder, user.id(), None),
            factory!(Folder, user.id(), None),
            factory!(Folder, user.id(), None),
        ];

        Ok(folders)
    }
}
