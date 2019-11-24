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
