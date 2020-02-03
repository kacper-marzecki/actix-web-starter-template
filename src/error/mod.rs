use crate::error::ApplicationError::DbError;
use std::error::Error;
use diesel::r2d2;
pub enum ApplicationError  {
    DbError{
        message: String,
    },
    Error {
        message: String,
    }
}
impl From<r2d2::PoolError> for ApplicationError {
    fn from(err: r2d2::PoolError) -> Self {
        ApplicationError::DbError {message: err.description().to_string()}
    }
}

