use argonautica::{Hasher, Verifier};
use argonautica::config::Variant;
use actix_web::web::Data;
use crate::app::app_state::AppState;
use actix_web::HttpRequest;
use futures::{Future, TryFutureExt, FutureExt};
use crate::common::{AppError, AppResult};
use actix_http::http::header::AUTHORIZATION;
use crate::repository::user::{Authenticate, Authentication};
use actix_http::http::HeaderValue;
use std::error::Error;
use jsonwebtoken::{decode, encode, Header, TokenData, Validation};
use std::env;
use uuid::Uuid;

lazy_static::lazy_static! {
    pub  static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    Hasher::fast_but_insecure()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .configure_variant(Variant::Argon2i)
        .hash()
        .map_err(|err| {
            dbg!(err);
            AppError::InternalServerError
        })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, argonautica::Error> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
}