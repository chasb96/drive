use super::Restrict;
use entities::models::{Folder, User};

impl Restrict<User> for Folder {
    fn viewable_by(&self, user: &User) -> bool {
        self.user_id() == user.id()
    }

    fn modifiable_by(&self, user: &User) -> bool {
        self.user_id() == user.id()
    }

    fn deletable_by(&self, user: &User) -> bool {
        self.user_id() == user.id()
    }
}
