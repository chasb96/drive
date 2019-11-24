use super::super::query::Query;
use entities::traits::user::UserStore;
use entities::models::{Folder, User};
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

impl UserStore for Store {
    fn find_by_user_id(&self, id: i32) -> Result<User, Error> {
        User::all()
            .filter(users::id.eq(id))
            .first::<User>(&DbFacade::connection())
    }

    fn save(&self, user: &User) -> Result<User, Error> {
        user.save()
    }

    fn update(&self, user: &User) -> Result<User, Error> {
        user.update()
    }

    fn delete(&self, user: &User) -> Result<User, Error> {
        user.delete()
    }

    fn update_password(&self, user: &User) -> Result<User, Error> {
        user.update_password()
    }

    fn folders(&self, user: &User) -> Result<Vec<Folder>, Error> {
        user.folders()
    }
}
