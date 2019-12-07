use entities::models::{Folder, User};
use crate::entities::traits::folder::FolderStore;
use crate::entities::error::DataStoreError;

impl User {
    pub fn folders(&self) -> Result<Vec<Folder>, DataStoreError> {
        let folder_store = resolve!(FolderStore);

        folder_store.find_by_user_id(self.id())
    }
}
