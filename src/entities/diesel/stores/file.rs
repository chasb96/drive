use entities::traits::file::FileStore;
use super::super::query::Query;
use entities::models::File;
use diesel::result::Error;
use entities::diesel::DbFacade;
use schema::*;
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
    fn find_by_file_id(&self, file_id: i32) -> Result<File, Error> {
        File::all()
            .filter(files::id.eq(file_id))
            .first::<File>(&DbFacade::connection())
    }

    fn find_by_folder_id(&self, folder_id: i32) -> Result<Vec<File>, Error> {
        File::all()
            .filter(files::folder_id.eq(folder_id))
            .load::<File>(&DbFacade::connection())
    }

    fn save(&self, file: &File) -> Result<File, Error> {
        file.save()
    }

    fn update(&self, file: &File) -> Result<File, Error> {
        file.update()
    }

    fn delete(&self, file: &File) -> Result<File, Error> {
        file.delete()
    }
}
