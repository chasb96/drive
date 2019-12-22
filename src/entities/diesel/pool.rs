use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
use crate::env::Env;

lazy_static! {
    static ref DB_POOL: Pool<ConnectionManager<PgConnection>> = { create_pool() };
}

fn create_pool() -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::new(Env::database_url());

    Pool::new(manager).unwrap()
}

pub struct DbPool;

impl DbPool {
    pub fn connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        DB_POOL.get().unwrap()
    }
}
