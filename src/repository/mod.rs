use actix::prelude::{Actor, SyncArbiter};
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection,},
};
use actix::SyncContext;
use std::error::Error;
pub use crate::common::AppError;

pub mod schema;
pub mod authentication;
pub mod user;

type ConnectionMgr = ConnectionManager<PgConnection>;
type ConnectionPool = Pool<ConnectionMgr>;

pub struct Repository(ConnectionPool);

impl Actor for Repository {
    type Context = SyncContext<Self>;
}

impl Repository {
    pub fn new(database_url: String)-> Self {
        Repository(new_pool(database_url.clone()))
    }
    pub fn get_conn(&self) -> Result<PooledConnection<ConnectionMgr>, AppError> {
        self.0.get().map_err(AppError::from)
    }
}

pub fn new_pool(database_url: String) -> ConnectionPool {
    println!("database url: {}", database_url.clone());
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
    .max_size(5)
    .build(manager)
    .expect("Cannot create connection pool")
}
