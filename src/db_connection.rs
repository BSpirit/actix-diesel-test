use diesel::prelude::*;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };


pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
pub type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

fn init_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> SqlitePool {
    init_pool("file:db/tmp.db").expect("Failed to create pool")
}