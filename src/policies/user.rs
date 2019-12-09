use super::{Restrict, Restricted};
use entities::models::User;

impl Restricted for User {}

impl Restrict<User> for User {
    fn indexable_by(user: &User) -> bool {
        user.is_admin()
    }

    fn creatable_by(user: &User) -> bool {
        user.is_admin()
    }

    fn viewable_by(&self, user: &User) -> bool {
        user.is_admin() || self.id() == user.id()
    }

    fn modifiable_by(&self, user: &User) -> bool {
        user.is_admin() || self.id() == user.id()
    }

    fn deletable_by(&self, user: &User) -> bool {
        user.is_admin() || self.id() == user.id()
    }
}

#[cfg(test)]
mod tests {
    use entities::models::User;
    use crate::entities::builders::{ Builder, UserBuilder };
    use policies::Restricted;

    #[test]
    fn test_indexable_by_admin() {
        let mut admin = factory!(User);
        admin.set_role("admin".to_string());

        assert!(admin.can_index::<User>());
    }

    #[test]
    fn test_indexable_by_non_admin() {
        let mut user = factory!(User);
        let mut guest = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());

        assert!(!user.can_index::<User>());
        assert!(!guest.can_index::<User>());
    }

    #[test]
    fn test_createable_by_admin() {
        let mut admin = factory!(User);
        admin.set_role("admin".to_string());

        assert!(admin.can_create::<User>());
    }

    #[test]
    fn test_createable_by_non_admin() {
        let mut user = factory!(User);
        let mut guest = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());

        assert!(!user.can_create::<User>());
        assert!(!guest.can_create::<User>());
    }

    #[test]
    fn test_viewable_by_admin() {
        let mut admin = factory!(User);
        let mut other = factory!(User);
        admin.set_role("admin".to_string());
        admin.set_id(2);
        other.set_id(1);

        assert!(admin.can_view(other));
    }

    #[test]
    fn test_viewable_by_non_admin() {
        let mut user = factory!(User);
        let mut guest = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());
        user.set_id(2);
        guest.set_id(3);
        other.set_id(1);

        assert!(!user.can_view(other.clone()));
        assert!(!guest.can_view(other));
    }

    #[test]
    fn test_viewable_by_self() {
        let mut user = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        user.set_id(1);
        other.set_id(1);

        assert!(user.can_view(other));
    }

    #[test]
    fn test_modifiable_by_admin() {
        let mut admin = factory!(User);
        let mut other = factory!(User);
        admin.set_role("admin".to_string());
        admin.set_id(2);
        other.set_id(1);

        assert!(admin.can_modify(other));
    }

    #[test]
    fn test_modifiable_by_non_admin() {
        let mut user = factory!(User);
        let mut guest = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());
        user.set_id(2);
        guest.set_id(3);
        other.set_id(1);

        assert!(!user.can_modify(other.clone()));
        assert!(!guest.can_modify(other));
    }

    #[test]
    fn test_modifiable_by_self() {
        let mut user = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        user.set_id(1);
        other.set_id(1);

        assert!(user.can_modify(other));
    }

    #[test]
    fn test_deletable_by_admin() {
        let mut admin = factory!(User);
        let mut other = factory!(User);
        admin.set_role("admin".to_string());
        admin.set_id(2);
        other.set_id(1);

        assert!(admin.can_delete(other));
    }

    #[test]
    fn test_deletable_by_non_admin() {
        let mut user = factory!(User);
        let mut guest = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        guest.set_role("guest".to_string());
        user.set_id(2);
        guest.set_id(3);
        other.set_id(1);

        assert!(!user.can_delete(other.clone()));
        assert!(!guest.can_delete(other));
    }

    #[test]
    fn test_deletable_by_self() {
        let mut user = factory!(User);
        let mut other = factory!(User);
        user.set_role("user".to_string());
        user.set_id(1);
        other.set_id(1);

        assert!(user.can_delete(other));
    }
}
