use entities::models::{Folder, User};
use entities::diesel::DbFacade;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::*;

impl User {
    pub fn folders(&self) -> Result<Vec<Folder>, Error> {
        Folder::all()
            .filter(folders::user_id.eq(self.id()))
            .load::<Folder>(&DbFacade::connection())
    }
}
