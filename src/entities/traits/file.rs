use diesel::result::Error;
use entities::models::File;

pub trait FileStore {
    fn find_by_file_id(&self, file_id: i32) -> Result<File, Error>;

    fn find_by_folder_id(&self, folder_id: i32) -> Result<Vec<File>, Error>;

    fn save(&self, file: &File) -> Result<File, Error>;

    fn update(&self, file: &File) -> Result<File, Error>;

    fn delete(&self, file: &File) -> Result<File, Error>;
}
