use crate::entities::models::File;
use crate::entities::diesel::pool::DbPool;
use crate::entities::diesel::query::Query;
use diesel;
use diesel::dsl::Eq;
use diesel::expression::Expression;
use diesel::pg::Pg;
use diesel::prelude::Insertable;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use crate::schema::*;

type AllColumns = (
    files::id,
    files::name,
    files::file_name,
    files::folder_id,
    files::created_at,
    files::updated_at,
    files::extension,
    files::public,
);

const ALL_COLUMNS: AllColumns = (
    files::id,
    files::name,
    files::file_name,
    files::folder_id,
    files::created_at,
    files::updated_at,
    files::extension,
    files::public,
);

type SqlType = <AllColumns as Expression>::SqlType;
type BoxedQuery<'a> = files::BoxedQuery<'a, Pg, SqlType>;

impl File {
    pub fn all() -> BoxedQuery<'static> {
        files::table.select(ALL_COLUMNS).into_boxed()
    }
}

impl Query for File {
    fn save(&self) -> Result<File, diesel::result::Error> {
        diesel::insert_into(files::table)
            .values(self)
            .get_result(&DbPool::connection())
    }

    fn update(&self) -> Result<File, diesel::result::Error> {
        diesel::update(files::table.find(&self.id()))
            .set((
                files::name.eq(self.name()),
                files::file_name.eq(self.file_name()),
                files::folder_id.eq(self.folder_id()),
                files::extension.eq(self.extension()),
                files::public.eq(self.public()),
            ))
            .get_result(&DbPool::connection())
    }

    fn delete(&self) -> Result<File, diesel::result::Error> {
        diesel::delete(files::table.find(&self.id())).get_result(&DbPool::connection())
    }
}

impl<'insert> Insertable<files::table> for &'insert File {
    #[allow(clippy::type_complexity)]
    type Values = <(
        Eq<files::name, &'insert String>,
        Eq<files::file_name, &'insert String>,
        Eq<files::folder_id, i32>,
        Eq<files::extension, &'insert String>,
        Eq<files::public, bool>,
    ) as Insertable<files::table>>::Values;

    fn values(self) -> Self::Values {
        let name = self.name();
        let file_name = self.file_name();
        let folder_id = self.folder_id();
        let extension = self.extension();
        let public = self.public();

        Insertable::values((
            files::name.eq(name),
            files::file_name.eq(file_name),
            files::folder_id.eq(folder_id),
            files::extension.eq(extension),
            files::public.eq(public),
        ))
    }
}
