use bcrypt::hash;
use crate::entities::builders::{Builder, UserBuilder};
use crate::entities::models::User;
use crate::services::error::ServiceError;
use crate::env::Env;
use crate::services::UserService;
use crate::services::FolderService;
use crate::entities::traits::user::UserStore;
use crate::services::user::CreateRequest;
use crate::services::user::UpdateRequest;
use crate::services::folder::CreateRequest as FolderCreateRequest;

pub struct Service<T: UserStore, S: FolderService> {
    user_store: T,
    folder_service: S,
}

impl<T: UserStore, S: FolderService> Service<T, S> {
    pub fn new(user_store: T, folder_service: S) -> Self {
        Self {
            user_store,
            folder_service
        }
    }
}

impl<T: UserStore, S: FolderService> UserService for Service<T, S> {
    fn all(&self) -> Result<Vec<User>, ServiceError> {
        match self.user_store.all() {
            Ok(user) => Ok(user),
            Err(e) => Err(ServiceError::from(e))
        }
    }

    fn find_by_user_id(&self, user_id: i32) -> Result<User, ServiceError> {
        match self.user_store.find_by_user_id(user_id) {
            Ok(user) => Ok(user),
            Err(e) => Err(ServiceError::from(e))
        }
    }

    fn create(&self, request: CreateRequest) -> Result<User, ServiceError> {
        // Passwords are hashed - we don't want passwords to be
        //  visible if the database was ever cracked
        // When creating a User, hash the password first
        let password_hash = hash(&request.password, Env::bcrypt_cost()).unwrap();

        // Create user, with their name, email, role, and password
        let user = UserBuilder::new()
            .with_name(request.name)
            .with_email(request.email)
            .with_role(request.role)
            .with_password(password_hash)
            .build();

        // Store the User
        // If the store fails, return back the error
        self.user_store.save(&user)?;

        let folder_create_request = FolderCreateRequest {
            name: "/".to_string(),
            user_id: user.id(),
            parent_id: None
        };

        // If a user is successfully created,
        //  create their root directory
        // If creating the root directory fails, log it
        self.folder_service.create(folder_create_request)?;

        Ok(user)
    }

    fn update(&self, request: UpdateRequest) -> Result<User, ServiceError> {
        // Find the User by their id,
        // If the operation fails (like NotFound), throw it back
        let mut user = self.user_store.find_by_user_id(request.id)?;

        // Update the name, email, and role of the User
        // We don't update their password, as it is sensitive
        //  info; we want to separate the sensitive parts
        user.set_name(request.name);
        user.set_email(request.email);
        user.set_role(request.role);

        // Update the user in the DataStore,
        //  if there's an error, throw it back
        let user = self.user_store.update(&user)?;

        Ok(user)
    }

    fn delete(&self, id: i32) -> Result<User, ServiceError> {
        // Locate the User by their Id,
        // If finding the user fails, throw back the error
        let user = self.user_store.find_by_user_id(id)?;

        // Folders have a dependency on Users
        // To be able to delete the User, their folders
        //  must be deleted first
        //
        // Iterate through every Folder of the User
        //  and delete it
        //
        // TODO: This is an N+1, should bulk delete
        for folder in self.user_store.folders(&user)? {
            self.folder_service.delete(folder.id())?;
        }

        // Delete the User from the DataStore,
        //  if there's an error, throw it back
        let user = self.user_store.delete(&user)?;

        Ok(user)
    }

    fn update_password(&self, id: i32, password: String) -> Result<User, ServiceError> {
        // Locate the User by their Id,
        //  if there's an error, throw it back
        let mut user = self.user_store.find_by_user_id(id)?;

        // Hash the User's new password,
        //  as User's passwords are hashed for security
        let password_hash = hash(&password, Env::bcrypt_cost()).unwrap();

        // Update the User entity with the hashed password
        user.set_password(password_hash);

        // Request that the DataStore side update the
        //  user record
        let user = self.user_store.update_password(&user)?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::Service;
    use super::CreateRequest;
    use super::UpdateRequest;
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

        let request = CreateRequest {
            name: expected.name().to_string(),
            email: expected.email().to_string(),
            role: expected.role().to_string(),
            password: expected.password().to_string(),
        };

        let actual = user_service.create(request).unwrap();

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

        let request = UpdateRequest {
            id: expected.id(),
            name: expected.name().to_string(),
            email: expected.email().to_string(),
            role: expected.role().to_string(),
        };

        let actual = user_service.update(request).unwrap();

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
