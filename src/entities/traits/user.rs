use entities::models::Folder;
use crate::entities::error::DataStoreError;
use entities::models::User;

pub trait UserStore {
    fn all(&self) -> Result<Vec<User>, DataStoreError>;

    fn find_by_user_id(&self, id: i32) -> Result<User, DataStoreError>;

    fn save(&self, user: &User) -> Result<User, DataStoreError>;

    fn update(&self, user: &User) -> Result<User, DataStoreError>;

    fn delete(&self, user: &User) -> Result<User, DataStoreError>;

    fn update_password(&self, user: &User) -> Result<User, DataStoreError>;

    fn folders(&self, user: &User) -> Result<Vec<Folder>, DataStoreError>;
}
