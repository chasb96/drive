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
