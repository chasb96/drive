use entities::models::Folder;
use diesel::result::Error;
use entities::models::File;

pub trait FolderStore {
    fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Folder>, Error>;

    fn find_by_folder_id(&self, id: i32) -> Result<Folder, Error>;

    fn save(&self, folder: &Folder) -> Result<Folder, Error>;

    fn update(&self, folder: &Folder) -> Result<Folder, Error>;

    fn delete(&self, folder: &Folder) -> Result<Folder, Error>;

    fn files(&self, folder: &Folder) -> Result<Vec<File>, Error>;
}
