use actix::prelude::{Actor, SyncArbiter};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection,},
};
use actix::SyncContext;

pub mod schema;
pub use crate::error::ApplicationError;
use std::error::Error;

type ConnectionMgr = ConnectionManager<PgConnection>;
type ConnectionPool = Pool<ConnectionMgr>;

pub struct Repository(ConnectionPool);

impl Actor for Repository {
    type Context = SyncContext<Self>;
}

impl Repository {
    fn new(database_url: String)-> Self {
        Repository(new_pool(database_url))
    }
    fn get_conn(self) -> Result<PooledConnection<ConnectionMgr>, ApplicationError> {
        self.0.get().map_err( ApplicationError::from)
    }
}

pub fn new_pool(database_url: String) -> ConnectionPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Cannot create connection pool")
}
