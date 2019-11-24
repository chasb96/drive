pub mod implementation;

use entities::models::Folder;
use diesel::result::Error;

pub trait FolderService {
    fn all(&self, user_id: i32) -> Result<Vec<Folder>, Error>;

    fn find(&self, folder_id: i32) -> Result<Folder, Error>;

    fn create(&self, name: String, user_id: i32, parent_id: Option<i32>) -> Result<Folder, Error>;

    fn update(
        &self,
        id: i32,
        name: String,
        user_id: i32,
        parent_id: Option<i32>,
    ) -> Result<Folder, Error>;

    fn delete(&self, id: i32) -> Result<Folder, Error>;
}
