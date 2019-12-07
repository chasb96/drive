use super::{Basic, Error};
use entities::models::User;
use entities::diesel::pool::DbPool;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use schema::*;

impl Basic for User {
    fn verify(email: String, password: String) -> Result<Self, Error> {
        let conn = DbPool::connection();

        match Self::all()
            .filter(users::email.eq(email))
            .first::<Self>(&conn)
        {
            Ok(user) => {
                if user.password_check(&password) {
                    Ok(user)
                } else {
                    Err(Error::CredentialsInvalid)
                }
            },
            Err(_) => Err(Error::CredentialsInvalid),
        }
    }
}
