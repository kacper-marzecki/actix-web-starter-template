use actix::prelude::{Actor, SyncArbiter};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};
use actix::SyncContext;

pub mod schema;


type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub struct Repository(ConnectionPool);

impl Actor for Repository {
    type Context = SyncContext<Self>;
}

impl Repository {
    fn new(database_url: String)-> Self {
        Repository(new_pool(database_url))
    }
}

pub fn new_pool(database_url: String) -> ConnectionPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Cannot create connection pool")
}
