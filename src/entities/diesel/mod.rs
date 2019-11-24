mod query;
pub mod relationships;
pub mod stores;
pub mod pool;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PooledConnection;
use self::pool::DbPool;

pub struct DbFacade;

impl DbFacade {
    pub fn connection() -> PooledConnection<ConnectionManager<PgConnection>> {
        DbPool::connection()
    }
}
