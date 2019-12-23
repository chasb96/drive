use crate::services::folder::FolderService;
use crate::entities::models::{File, User};

#[cfg(test)]
use mockall::*;
#[cfg(test)]
use mockall::predicate::*;

#[cfg_attr(test, automock)]
pub trait FileAuthorizer {
    fn can_create(&self, user: &User) -> bool;

    fn can_view(&self, user: &User, file: &File) -> bool;

    fn can_modify(&self, user: &User, file: &File) -> bool;

    fn can_delete(&self, user: &User, file: &File) -> bool;
}

pub struct Authorizer<T: FolderService> {
    folder_service: T
}

impl<T: FolderService> Authorizer<T> {
    pub fn new(folder_service: T) -> Self {
        Self {
            folder_service
        }
    }
}

impl<T: FolderService> FileAuthorizer for Authorizer<T> {
    fn can_create(&self, _user: &User) -> bool {
        true
    }

    fn can_view(&self, user: &User, file: &File) -> bool {
        let folder = match self.folder_service.find(file.folder_id()) {
            Ok(folder) => folder,
            Err(_) => return false,
        };

        file.public() || folder.user_id() == user.id()
    }

    fn can_modify(&self, user: &User, file: &File) -> bool {
        let folder = match self.folder_service.find(file.folder_id()) {
            Ok(folder) => folder,
            Err(_) => return false,
        };

        folder.user_id() == user.id()
    }

    fn can_delete(&self, user: &User, file: &File) -> bool {
        let folder = match self.folder_service.find(file.folder_id()) {
            Ok(folder) => folder,
            Err(_) => return false,
        };

        folder.user_id() == user.id()
    }
}

#[cfg(test)]
mod tests {
    use super::Authorizer;
    use super::FileAuthorizer;
    use crate::services::folder::MockFolderService;
    use crate::entities::builders::{
        Builder,
        UserBuilder,
        FileBuilder,
        FolderBuilder
    };

    #[test]
    fn test_can_view() {
        let mock_folder_result = factory!(Folder, 1, None);
        let mut file = factory!(File, 1);
        file.set_public(false);
        let mut user = factory!(User);
        user.set_id(1);

        let mut mock_folder_service = MockFolderService::new();
        mock_folder_service
            .expect_find()
            .returning(move |_| Ok(mock_folder_result.clone()));

        let authorizer = Authorizer::new(mock_folder_service);

        assert!(authorizer.can_view(&user, &file))
    }

    #[test]
    fn test_cannot_view() {
        let mock_folder_result = factory!(Folder, 2, None);
        let mut file = factory!(File, 1);
        file.set_public(false);
        let mut user = factory!(User);
        user.set_id(1);

        let mut mock_folder_service = MockFolderService::new();
        mock_folder_service
            .expect_find()
            .returning(move |_| Ok(mock_folder_result.clone()));

        let authorizer = Authorizer::new(mock_folder_service);

        assert!(!authorizer.can_view(&user, &file))
    }

    #[test]
    fn test_can_modify() {
        let mock_folder_result = factory!(Folder, 1, None);
        let mut file = factory!(File, 1);
        file.set_public(false);
        let mut user = factory!(User);
        user.set_id(1);

        let mut mock_folder_service = MockFolderService::new();
        mock_folder_service
            .expect_find()
            .returning(move |_| Ok(mock_folder_result.clone()));

        let authorizer = Authorizer::new(mock_folder_service);

        assert!(authorizer.can_modify(&user, &file))
    }

    #[test]
    fn test_cannot_modify() {
        let mock_folder_result = factory!(Folder, 2, None);
        let mut file = factory!(File, 1);
        file.set_public(false);
        let mut user = factory!(User);
        user.set_id(1);

        let mut mock_folder_service = MockFolderService::new();
        mock_folder_service
            .expect_find()
            .returning(move |_| Ok(mock_folder_result.clone()));

        let authorizer = Authorizer::new(mock_folder_service);

        assert!(!authorizer.can_modify(&user, &file))
    }

    #[test]
    fn test_can_delete() {
        let mock_folder_result = factory!(Folder, 1, None);
        let mut file = factory!(File, 1);
        file.set_public(false);
        let mut user = factory!(User);
        user.set_id(1);

        let mut mock_folder_service = MockFolderService::new();
        mock_folder_service
            .expect_find()
            .returning(move |_| Ok(mock_folder_result.clone()));

        let authorizer = Authorizer::new(mock_folder_service);

        assert!(authorizer.can_delete(&user, &file))
    }

    #[test]
    fn test_cannot_delete() {
        let mock_folder_result = factory!(Folder, 2, None);
        let mut file = factory!(File, 1);
        file.set_public(false);
        let mut user = factory!(User);
        user.set_id(1);

        let mut mock_folder_service = MockFolderService::new();
        mock_folder_service
            .expect_find()
            .returning(move |_| Ok(mock_folder_result.clone()));

        let authorizer = Authorizer::new(mock_folder_service);

        assert!(!authorizer.can_delete(&user, &file))
    }
}
