use entities::models::{File, Folder};
use entities::diesel::DbFacade;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::*;

impl File {
    pub fn folder(&self) -> Result<Folder, Error> {
        Folder::all()
            .filter(folders::id.eq(self.folder_id()))
            .first::<Folder>(&DbFacade::connection())
    }
}
