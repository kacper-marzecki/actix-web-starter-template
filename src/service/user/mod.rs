
mod request;
mod response;

pub mod authentication;

use actix_web::{ web,HttpRequest,  HttpResponse, Responder};
use crate::app::app_state::AppState;
use futures::{Future, FutureExt, TryFutureExt};
use actix::Addr;
use crate::repository::Repository;
use crate::model::user::User;
use crate::common::AppError;
use diesel::PgConnection;
use crate::service::user::request::RegisterUserRequest;
use actix_web::error::BlockingError;
use crate::common::AppResult;
use validator::{Validate, ValidationErrors};
use crate::repository::user::RegisterUser;
use crate::service::user::response::UserResponse;
use std::sync::Arc;

pub async fn register_user(
    user_data: web::Json<RegisterUserRequest>,
    state: web::Data<AppState>
) -> impl Responder {
    user_data.0.validate()
        .map_err(|err: ValidationErrors| AppError::UnprocessableEntity(json!(String::from("Validation error"))))?;
    let repository = state.repository.clone();
    let request = user_data.into_inner();
    repository.send(RegisterUser{
        username: request.username,
        email: request.email,
        password_hash: authentication::hash_password(&request.password)?,
    }).await?
        .map(|user|{
            HttpResponse::Ok().json(UserResponse{
                id: user.id,
                username: user.username,
                email: user.email
            })
        })
    }