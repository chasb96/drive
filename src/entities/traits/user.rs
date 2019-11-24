use entities::models::Folder;
use diesel::result::Error;
use entities::models::User;

pub trait UserStore {
    fn find_by_user_id(&self, id: i32) -> Result<User, Error>;

    fn save(&self, user: &User) -> Result<User, Error>;

    fn update(&self, user: &User) -> Result<User, Error>;

    fn delete(&self, user: &User) -> Result<User, Error>;

    fn update_password(&self, user: &User) -> Result<User, Error>;

    fn folders(&self, user: &User) -> Result<Vec<Folder>, Error>;
}
