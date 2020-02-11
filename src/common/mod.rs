use std::error::Error;
use diesel::r2d2;

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use actix::MailboxError;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};
use libreauth::pass::ErrorCode as PassErrorCode;
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::convert::From;
use validator::ValidationErrors;


pub type AppResult<T, E = AppError> = std::result::Result<T, E>;

#[derive(Fail, Debug)]
pub enum AppError {
    // 401
    #[fail(display = "Unauthorized: {}", _0)]
    Unauthorized(JsonValue),

    // 403
    #[fail(display = "Forbidden: {}", _0)]
    Forbidden(JsonValue),

    // 404
    #[fail(display = "Not Found: {}", _0)]
    NotFound(JsonValue),

    // 422
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),
    

    // 500
    #[fail(display = "Internal Server Error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            AppError::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            AppError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            AppError::UnprocessableEntity(ref message) => {
                HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message)
            }
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }
}

impl From<MailboxError> for AppError {
    fn from(_error: MailboxError) -> Self {
        AppError::InternalServerError
    }
}

impl From<JwtError> for AppError {
    fn from(error: JwtError) -> Self {
        match error.kind() {
            JwtErrorKind::InvalidToken => AppError::Unauthorized(json!({
                "error": "Token is invalid",
            })),
            JwtErrorKind::InvalidIssuer => AppError::Unauthorized(json!({
                "error": "Issuer is invalid",
            })),
            _ => AppError::Unauthorized(json!({
                "error": "An issue was found with the token provided",
            })),
        }
    }
}

impl From<DieselError> for AppError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return AppError::UnprocessableEntity(json!({ "error": message }));
                }
                AppError::InternalServerError
            }
            DieselError::NotFound => {
                AppError::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => AppError::InternalServerError,
        }
    }
}

impl From<PoolError> for AppError {
    fn from(_error: PoolError) -> Self {
        AppError::InternalServerError
    }
}

impl From<PassErrorCode> for AppError {
    fn from(_error: PassErrorCode) -> Self {
        AppError::InternalServerError
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        // transforms errors into objects that err_map can take
        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors
                .iter()
                .map(|error| {
                    // dbg!(error) // <- Uncomment this if you want to see what error looks like
                    json!(error.message)
                })
                .collect();
            err_map.insert(field.to_string(), json!(errors));
        }

        AppError::UnprocessableEntity(json!({
            "errors": err_map,
        }))
    }
}

