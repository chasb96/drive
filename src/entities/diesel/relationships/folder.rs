use crate::entities::models::{File, Folder, User};
use crate::entities::error::DataStoreError;
use crate::entities::traits::file::FileStore;
use crate::entities::traits::user::UserStore;

impl Folder {
    pub fn files(&self) -> Result<Vec<File>, DataStoreError> {
        let file_store = resolve!(FileStore);

        file_store.find_by_folder_id(self.id())
    }

    pub fn user(&self) -> Result<User, DataStoreError> {
        let user_store = resolve!(UserStore);

        user_store.find_by_user_id(self.id())
    }
}
