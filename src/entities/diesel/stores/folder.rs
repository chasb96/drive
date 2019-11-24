use entities::models::File;
use entities::models::Folder;
use entities::traits::folder::FolderStore;
use diesel::result::Error;
use super::super::query::Query;
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

impl FolderStore for Store {
    fn find_by_user_id(&self, user_id: i32) -> Result<Vec<Folder>, Error> {
        Folder::all()
            .filter(folders::user_id.eq(user_id))
            .load::<Folder>(&DbFacade::connection())
    }

    fn find_by_folder_id(&self, folder_id: i32) -> Result<Folder, Error> {
        Folder::all()
            .filter(folders::id.eq(folder_id))
            .first::<Folder>(&DbFacade::connection())
    }

    fn save(&self, folder: &Folder) -> Result<Folder, Error> {
        folder.save()
    }

    fn update(&self, folder: &Folder) -> Result<Folder, Error> {
        folder.update()
    }

    fn delete(&self, folder: &Folder) -> Result<Folder, Error> {
        folder.delete()
    }

    fn files(&self, folder: &Folder) -> Result<Vec<File>, Error> {
        folder.files()
    }
}
