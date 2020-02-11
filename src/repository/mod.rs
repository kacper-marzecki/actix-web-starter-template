pub use crate::common::AppError;
use actix::prelude::{Actor, SyncArbiter};
use actix::Context;
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager, Pool, PooledConnection},
};
use std::error::Error;

pub mod authentication;
pub mod schema;
pub mod user;

pub type ConnectionMgr = ConnectionManager<PgConnection>;
pub type ConnectionPool = Pool<ConnectionMgr>;

pub struct Repository(ConnectionPool);

impl Actor for Repository {
    type Context = Context<Self>;
}

impl Repository {
    pub fn new(database_url: String) -> Self {
        Repository(new_pool(database_url.clone()))
    }
    pub fn get_conn(&mut self) -> Result<PooledConnection<ConnectionMgr>, AppError> {
        self.0.get().map_err(AppError::from)
    }
}

pub fn new_pool(database_url: String) -> ConnectionPool {
    println!("database url: {}", database_url.clone());
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .map(|pool| {
            println!("Connection pool created.");
            pool
        })
        .expect("Cannot create connection pool")
}
