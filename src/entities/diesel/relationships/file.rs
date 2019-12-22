use crate::entities::models::{File, Folder};
use crate::entities::traits::folder::FolderStore;
use crate::entities::error::DataStoreError;

impl File {
    pub fn folder(&self) -> Result<Folder, DataStoreError> {
        let folder_service = resolve!(FolderStore);

        folder_service.find_by_folder_id(self.folder_id())
    }
}
