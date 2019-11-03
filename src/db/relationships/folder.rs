use db::models::{File, Folder, User};
use db::DbFacade;
use diesel::result::Error;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use schema::*;

impl Folder {
    pub fn files(&self) -> Result<Vec<File>, Error> {
        File::all()
            .filter(files::folder_id.eq(self.id()))
            .load::<File>(&DbFacade::connection())
    }

    pub fn user(&self) -> Result<User, Error> {
        User::all()
            .filter(users::id.eq(self.user_id()))
            .first::<User>(&DbFacade::connection())
    }
}
