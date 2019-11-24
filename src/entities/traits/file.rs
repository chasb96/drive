use crate::entities::error::DataStoreError;
use entities::models::File;

pub trait FileStore {
    fn find_by_file_id(&self, file_id: i32) -> Result<File, DataStoreError>;

    fn find_by_folder_id(&self, folder_id: i32) -> Result<Vec<File>, DataStoreError>;

    fn save(&self, file: &File) -> Result<File, DataStoreError>;

    fn update(&self, file: &File) -> Result<File, DataStoreError>;

    fn delete(&self, file: &File) -> Result<File, DataStoreError>;
}
