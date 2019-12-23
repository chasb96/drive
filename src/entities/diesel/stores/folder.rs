use crate::entities::models::File;
use crate::entities::models::Folder;
use crate::entities::traits::folder::FolderStore;
use crate::entities::error::DataStoreError;
use super::super::query::Query;
use crate::entities::diesel::DbFacade;
use crate::schema::*;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

pub struct Store;

impl Store {
    pub fn new() -> Self {
        Self
    }
}

impl FolderStore for Store {
    fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Folder>, DataStoreError> {
        let folders = Folder::all()
            .filter(folders::user_id.eq(user_id))
            .load::<Folder>(&DbFacade::connection())?;

        Ok(folders)
    }

    fn find_by_folder_id(&self, folder_id: i32) -> Result<Folder, DataStoreError> {
        let folder = Folder::all()
            .filter(folders::id.eq(folder_id))
            .first::<Folder>(&DbFacade::connection())?;

        Ok(folder)
    }

    fn save(&self, folder: &Folder) -> Result<Folder, DataStoreError> {
        let folder = folder.save()?;

        Ok(folder)
    }

    fn update(&self, folder: &Folder) -> Result<Folder, DataStoreError> {
        let folder = folder.update()?;

        Ok(folder)
    }

    fn delete(&self, folder: &Folder) -> Result<Folder, DataStoreError> {
        let folder = folder.delete()?;

        Ok(folder)
    }

    fn files(&self, folder: &Folder) -> Result<Vec<File>, DataStoreError> {
        let files = folder.files()?;

        Ok(files)
    }
}
