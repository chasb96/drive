use crate::entities::models::{Folder, User};

#[cfg(test)]
use mockall::*;
#[cfg(test)]
use mockall::predicate::*;

#[cfg_attr(test, automock)]
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

#[cfg(test)]
mod tests {
    use super::Authorizer;
    use super::FolderAuthorizer;
    use crate::entities::builders::{
        Builder,
        UserBuilder,
        FolderBuilder
    };

    #[test]
    fn test_can_view() {
        let user = factory!(User);
        let folder = factory!(Folder, user.id(), None);

        let authorizer = Authorizer::new();

        assert!(authorizer.can_view(&user, &folder))
    }

    #[test]
    fn test_cannot_view() {
        let user = factory!(User);
        let folder = factory!(Folder, user.id() + 1, None);

        let authorizer = Authorizer::new();

        assert!(!authorizer.can_view(&user, &folder))
    }

    #[test]
    fn test_can_modify() {
        let user = factory!(User);
        let folder = factory!(Folder, user.id(), None);

        let authorizer = Authorizer::new();

        assert!(authorizer.can_modify(&user, &folder))
    }

    #[test]
    fn test_cannot_modify() {
        let user = factory!(User);
        let folder = factory!(Folder, user.id() + 1, None);

        let authorizer = Authorizer::new();

        assert!(!authorizer.can_modify(&user, &folder))
    }

    #[test]
    fn test_can_delete() {
        let user = factory!(User);
        let folder = factory!(Folder, user.id(), None);

        let authorizer = Authorizer::new();

        assert!(authorizer.can_delete(&user, &folder))
    }

    #[test]
    fn test_cannot_delete() {
        let user = factory!(User);
        let folder = factory!(Folder, user.id() + 1, None);

        let authorizer = Authorizer::new();

        assert!(!authorizer.can_delete(&user, &folder))
    }
}
