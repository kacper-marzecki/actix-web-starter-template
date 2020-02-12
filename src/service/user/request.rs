use crate::common::AppError;
use actix_http::Payload;
use actix_identity::{Identity, RequestIdentity};
use actix_web::{FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use std::future::Future;
use validator::{Validate, ValidationError};


#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct RegisterUserRequest {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1), must_match = "password")]
    pub password_repeat: String,
}
