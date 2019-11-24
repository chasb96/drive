use bcrypt::hash;
use entities::builders::{Builder, UserBuilder};
use entities::models::User;
use diesel::result::Error;
use env::Env;
use crate::services::UserService;
use crate::services::FolderService;
use entities::traits::user::UserStore;

pub struct Service<T: UserStore, S: FolderService> {
    user_store: T,
    folder_service: S,
}

impl<T: UserStore, S: FolderService> Service<T, S> {
    pub fn new(user_store: T, folder_service: S) -> Self {
        Self { user_store, folder_service }
    }
}

impl<T: UserStore, S: FolderService> UserService for Service<T, S> {
    fn create(
        &self,
        name: String,
        email: String,
        role: String,
        password: String,
    ) -> Result<User, Error> {
        let password_hash = hash(&password, Env::bcrypt_cost()).unwrap();

        let user = UserBuilder::new()
            .with_name(name)
            .with_email(email)
            .with_role(role)
            .with_password(password_hash)
            .build();

        self.user_store.save(&user)?;

        if let Err(e) = self.folder_service.create("/".to_string(), user.id(), None) {
            log!(
                "error",
                "Failed to create root directory for user {}: {}",
                user.id(),
                e
            );
            // return Err(e);
        }

        Ok(user)
    }

    fn update(&self, id: i32, name: String, email: String, role: String) -> Result<User, Error> {
        let mut user = self.user_store.find_by_user_id(id)?;

        user.set_name(name);
        user.set_email(email);
        user.set_role(role);

        self.user_store.update(&user)
    }

    fn delete(&self, id: i32) -> Result<User, Error> {
        let user = self.user_store.find_by_user_id(id)?;

        for folder in self.user_store.folders(&user)? {
            if let Err(e) = self.folder_service.delete(folder.id()) {
                log!("error", "Failed to delete folder {}: {}", folder.id(), e);
                return Err(e);
            }
        }

        self.user_store.delete(&user)
    }

    fn update_password(&self, id: i32, password: String) -> Result<User, Error> {
        let mut user = self.user_store.find_by_user_id(id)?;

        let password_hash = hash(&password, Env::bcrypt_cost()).unwrap();

        user.set_password(password_hash);

        self.user_store.update_password(&user)
    }
}

#[cfg(test)]
mod tests {
    use super::Service;
    use crate::test::mocks::folder::service::FolderServiceMock;
    use crate::test::mocks::user::store::UserStoreMock;
    use crate::services::UserService;
    use crate::entities::builders::{ Builder, UserBuilder };
    use bcrypt::verify;

    #[test]
    fn test_create() {
        let folder_service = FolderServiceMock::new();
        let user_store = UserStoreMock::new();
        let user_service = Service::new(user_store, folder_service);

        let expected = factory!(User);

        let actual = user_service.create(
            expected.name().to_string(),
            expected.email().to_string(),
            expected.role().to_string(),
            expected.password().to_string(),
        )
        .unwrap();

        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.email(), actual.email());
        assert!(verify(expected.password(), actual.password()).unwrap());
    }

    #[test]
    fn test_update() {
        let folder_service = FolderServiceMock::new();
        let user_store = UserStoreMock::new();
        let user_service = Service::new(user_store, folder_service);

        let expected = factory!(User);

        let actual = user_service.update(
            expected.id(),
            expected.name().to_string(),
            expected.email().to_string(),
            expected.role().to_string(),
        )
        .unwrap();

        assert_eq!(expected.id(), actual.id());
        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.email(), actual.email());
    }

    #[test]
    fn test_delete() {
        let folder_service = FolderServiceMock::new();
        let user_store = UserStoreMock::new();
        let user_service = Service::new(user_store, folder_service);

        let expected = factory!(User);

        let actual = user_service.delete(
            expected.id(),
        )
        .unwrap();

        assert_eq!(expected.id(), actual.id());
    }
}
