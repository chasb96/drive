use super::Query;
use crate::entities::models::User;
use crate::entities::diesel::pool::DbPool;
use diesel;
use diesel::dsl::Eq;
use diesel::expression::Expression;
use diesel::pg::Pg;
use diesel::prelude::Insertable;
use diesel::ExpressionMethods;
use diesel::{QueryDsl, RunQueryDsl};
use crate::schema::*;

type AllColumns = (
    users::id,
    users::name,
    users::email,
    users::password,
    users::created_at,
    users::updated_at,
    users::root,
    users::role,
);

const ALL_COLUMNS: AllColumns = (
    users::id,
    users::name,
    users::email,
    users::password,
    users::created_at,
    users::updated_at,
    users::root,
    users::role,
);

type SqlType = <AllColumns as Expression>::SqlType;
type BoxedQuery<'a> = users::BoxedQuery<'a, Pg, SqlType>;

impl User {
    pub fn all() -> BoxedQuery<'static> {
        users::table.select(ALL_COLUMNS).into_boxed()
    }

    pub fn update_password(&self) -> Result<User, diesel::result::Error> {
        diesel::update(users::table.find(&self.id()))
            .set(users::password.eq(self.password()))
            .get_result(&DbPool::connection())
    }
}

impl Query for User {
    fn save(&self) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(self)
            .get_result(&DbPool::connection())
    }

    fn update(&self) -> Result<User, diesel::result::Error> {
        diesel::update(users::table.find(&self.id()))
            .set((
                users::name.eq(self.name()),
                users::email.eq(self.email()),
                users::root.eq(self.root()),
                users::role.eq(self.role()),
            ))
            .get_result(&DbPool::connection())
    }

    fn delete(&self) -> Result<User, diesel::result::Error> {
        diesel::delete(users::table.find(&self.id())).get_result(&DbPool::connection())
    }
}

// We don't want any inserts into the other fields.
// We could simply just derive insertable, but there also could be lots
//  of stuff doing important conversation through here.
// We know somehow there'd be form injection somehow,
//  so we're just going to purely block off the inserts.
impl<'insert> Insertable<users::table> for &'insert User {
    // Instead of implementing Insertable on the entire table,
    //  we're going to implement Insertable for the two columns we want,
    //  as the table's insert statement
    #[allow(clippy::type_complexity)]
    type Values = <(
        Eq<users::name, &'insert String>,
        Eq<users::email, &'insert String>,
        Eq<users::password, &'insert String>,
        Eq<users::root, &'insert Option<i32>>,
        Eq<users::role, &'insert String>,
    ) as Insertable<users::table>>::Values;

    fn values(self) -> Self::Values {
        let name = self.name();
        let email = self.email();
        let password = self.password();
        let root = self.root();
        let role = self.role();

        Insertable::values((
            users::name.eq(name),
            users::email.eq(email),
            users::password.eq(password),
            users::root.eq(root),
            users::role.eq(role),
        ))
    }
}
