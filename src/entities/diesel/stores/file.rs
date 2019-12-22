use crate::entities::traits::file::FileStore;
use super::super::query::Query;
use crate::entities::models::File;
use crate::entities::error::DataStoreError;
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

impl FileStore for Store {
    fn find_by_file_id(&self, file_id: i32) -> Result<File, DataStoreError> {
        let file = File::all()
            .filter(files::id.eq(file_id))
            .first::<File>(&DbFacade::connection())?;

        Ok(file)
    }

    fn find_by_folder_id(&self, folder_id: i32) -> Result<Vec<File>, DataStoreError> {
        let files = File::all()
            .filter(files::folder_id.eq(folder_id))
            .load::<File>(&DbFacade::connection())?;

        Ok(files)
    }

    fn save(&self, file: &File) -> Result<File, DataStoreError> {
        let file = file.save()?;

        Ok(file)
    }

    fn update(&self, file: &File) -> Result<File, DataStoreError> {
        let file = file.update()?;

        Ok(file)
    }

    fn delete(&self, file: &File) -> Result<File, DataStoreError> {
        let file = file.delete()?;

        Ok(file)
    }
}
