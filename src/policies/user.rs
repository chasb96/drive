use crate::entities::models::User;

pub trait UserAuthorizer {
    fn can_index(&self, user: &User) -> bool;

    fn can_create(&self, user: &User) -> bool;

    fn can_view(&self, user: &User, other: &User) -> bool;

    fn can_modify(&self, user: &User, other: &User) -> bool;

    fn can_delete(&self, user: &User, other: &User) -> bool;
}

pub struct Authorizer;

impl Authorizer {
    pub fn new() -> Self {
        Self
    }
}

impl UserAuthorizer for Authorizer {
    fn can_index(&self, user: &User) -> bool {
        user.is_admin()
    }

    fn can_create(&self, user: &User) -> bool {
        user.is_admin()
    }

    fn can_view(&self, user: &User, other: &User) -> bool {
        user.is_admin() || other.id() == user.id()
    }

    fn can_modify(&self, user: &User, other: &User) -> bool {
        user.is_admin() || other.id() == user.id()
    }

    fn can_delete(&self, user: &User, other: &User) -> bool {
        user.is_admin() || other.id() == user.id()
    }
}

#[cfg(test)]
mod tests {
    use super::Authorizer;
    use super::UserAuthorizer;
    use crate::entities::builders::{ Builder, UserBuilder };

    #[test]
    fn test_indexable_by_admin() {
        let authorizer = Authorizer::new();

        let mut admin = factory!(User);
        admin.set_role("admin".to_string());

        assert!(authorizer.can_index(&admin))
    }

    #[test]
    fn test_indexable_by_non_admin() {
        let authorizer = Authorizer::new();

        let mut user = factory!(User);
        let mut guest = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());

        assert!(!authorizer.can_index(&user));
        assert!(!authorizer.can_index(&user))
    }

    #[test]
    fn test_createable_by_admin() {
        let authorizer = Authorizer::new();

        let mut admin = factory!(User);
        admin.set_role("admin".to_string());

        assert!(authorizer.can_create(&admin))
    }

    #[test]
    fn test_createable_by_non_admin() {
        let authorizer = Authorizer::new();

        let mut user = factory!(User);
        let mut guest = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());

        assert!(!authorizer.can_create(&user));
        assert!(!authorizer.can_create(&guest))
    }

    #[test]
    fn test_viewable_by_admin() {
        let authorizer = Authorizer::new();

        let mut admin = factory!(User);
        let mut other = factory!(User);
        admin.set_role("admin".to_string());
        admin.set_id(2);
        other.set_id(1);

        assert!(authorizer.can_view(&admin, &other));
    }

    #[test]
    fn test_viewable_by_non_admin() {
        let authorizer = Authorizer::new();

        let mut user = factory!(User);
        let mut guest = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());
        user.set_id(2);
        guest.set_id(3);
        other.set_id(1);

        assert!(!authorizer.can_view(&user, &other));
        assert!(!authorizer.can_view(&guest, &other))
    }

    #[test]
    fn test_viewable_by_self() {
        let authorizer = Authorizer::new();

        let mut user = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        user.set_id(1);
        other.set_id(1);

        assert!(authorizer.can_view(&user, &other))
    }

    #[test]
    fn test_modifiable_by_admin() {
        let authorizer = Authorizer::new();

        let mut admin = factory!(User);
        let mut other = factory!(User);
        admin.set_role("admin".to_string());
        admin.set_id(2);
        other.set_id(1);

        assert!(authorizer.can_modify(&admin, &other))
    }

    #[test]
    fn test_modifiable_by_non_admin() {
        let authorizer = Authorizer::new();

        let mut user = factory!(User);
        let mut guest = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());
        user.set_id(2);
        guest.set_id(3);
        other.set_id(1);

        assert!(!authorizer.can_modify(&user, &other));
        assert!(!authorizer.can_modify(&guest, &other))
    }

    #[test]
    fn test_modifiable_by_self() {
        let authorizer = Authorizer::new();

        let mut user = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        user.set_id(1);
        other.set_id(1);

        assert!(authorizer.can_modify(&user, &other))
    }

    #[test]
    fn test_deletable_by_admin() {
        let authorizer = Authorizer::new();

        let mut admin = factory!(User);
        let mut other = factory!(User);
        admin.set_role("admin".to_string());
        admin.set_id(2);
        other.set_id(1);

        assert!(authorizer.can_delete(&admin, &other))
    }

    #[test]
    fn test_deletable_by_non_admin() {
        let authorizer = Authorizer::new();

        let mut user = factory!(User);
        let mut guest = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());
        user.set_id(2);
        guest.set_id(3);
        other.set_id(1);

        assert!(!authorizer.can_delete(&user, &other));
        assert!(!authorizer.can_delete(&guest, &other))
    }

    #[test]
    fn test_deletable_by_self() {
        let authorizer = Authorizer::new();

        let mut user = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        user.set_id(1);
        other.set_id(1);

        assert!(authorizer.can_delete(&user, &other))
    }
}
