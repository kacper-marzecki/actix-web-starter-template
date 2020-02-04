
mod request;
pub mod authentication;

use actix_web::{web, HttpResponse};
use crate::app::app_state::AppState;
use futures::{Future, FutureExt};
use actix::Addr;
use crate::repository::Repository;
use crate::model::user::User;
use crate::common::AppError;
use diesel::PgConnection;
use crate::service::user::request::RegisterUserRequest;
use actix_web::error::BlockingError;

pub fn register(
    user_data: web::Json<RegisterUserRequest>,
    state: web::Data<AppState>,
) -> Result< HttpResponse, AppError> {
    let repository = state.repository.clone();
    web::block(move || {
//        repository.send()
        Err(AppError::InternalServerError)
    })
        .then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(&user)),
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(AppError::InternalServerError),
            },
        })
}