use super::Query;
use entities::models::Folder;
use entities::diesel::pool::DbPool;
use diesel;
use diesel::dsl::Eq;
use diesel::expression::Expression;
use diesel::pg::Pg;
use diesel::prelude::Insertable;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use schema::*;

type AllColumns = (
    folders::id,
    folders::name,
    folders::parent_id,
    folders::user_id,
    folders::created_at,
    folders::updated_at,
);

const ALL_COLUMNS: AllColumns = (
    folders::id,
    folders::name,
    folders::parent_id,
    folders::user_id,
    folders::created_at,
    folders::updated_at,
);

type SqlType = <AllColumns as Expression>::SqlType;
type BoxedQuery<'a> = folders::BoxedQuery<'a, Pg, SqlType>;

impl Folder {
    pub fn all() -> BoxedQuery<'static> {
        folders::table.select(ALL_COLUMNS).into_boxed()
    }
}

impl Query for Folder {
    fn save(&self) -> Result<Folder, diesel::result::Error> {
        diesel::insert_into(folders::table)
            .values(self)
            .get_result(&DbPool::connection())
    }

    fn update(&self) -> Result<Folder, diesel::result::Error> {
        diesel::update(folders::table.find(&self.id()))
            .set((
                folders::name.eq(self.name()),
                folders::parent_id.eq(self.parent_id()),
                folders::user_id.eq(self.user_id()),
            ))
            .get_result(&DbPool::connection())
    }

    fn delete(&self) -> Result<Folder, diesel::result::Error> {
        diesel::delete(folders::table.find(&self.id())).get_result(&DbPool::connection())
    }
}

impl<'insert> Insertable<folders::table> for &'insert Folder {
    type Values = <(
        Eq<folders::name, &'insert String>,
        Eq<folders::parent_id, &'insert Option<i32>>,
        Eq<folders::user_id, i32>,
    ) as Insertable<folders::table>>::Values;

    fn values(self) -> Self::Values {
        let name = self.name();
        let parent_id = self.parent_id();
        let user_id = self.user_id();

        Insertable::values((
            folders::name.eq(name),
            folders::parent_id.eq(parent_id),
            folders::user_id.eq(user_id),
        ))
    }
}
