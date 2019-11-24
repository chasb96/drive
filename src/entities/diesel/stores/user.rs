use super::super::query::Query;
use entities::traits::user::UserStore;
use entities::models::{Folder, User};
use crate::entities::error::DataStoreError;
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
    fn find_by_user_id(&self, id: i32) -> Result<User, DataStoreError> {
        let user = User::all()
            .filter(users::id.eq(id))
            .first::<User>(&DbFacade::connection())?;

        Ok(user)
    }

    fn save(&self, user: &User) -> Result<User, DataStoreError> {
        let user = user.save()?;

        Ok(user)
    }

    fn update(&self, user: &User) -> Result<User, DataStoreError> {
        let user = user.update()?;

        Ok(user)
    }

    fn delete(&self, user: &User) -> Result<User, DataStoreError> {
        let user = user.delete()?;

        Ok(user)
    }

    fn update_password(&self, user: &User) -> Result<User, DataStoreError> {
        let user = user.update_password()?;

        Ok(user)
    }

    fn folders(&self, user: &User) -> Result<Vec<Folder>, DataStoreError> {
        let folders = user.folders()?;

        Ok(folders)
    }
}
