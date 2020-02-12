use crate::app::app_state::AppState;
use crate::common::{AppError, AppResult};
use crate::repository::user::{Authenticate, Authentication};
use actix_http::http::{header::AUTHORIZATION, HeaderValue};
use actix_http::Payload;
use actix_identity::{Identity, RequestIdentity};
use actix_web::{web::Data, FromRequest, HttpRequest};
use argonautica::{config::Variant, Hasher, Verifier};
use futures::future::{err, ok, Ready};
use futures::{Future, FutureExt, TryFutureExt};
use jsonwebtoken::{decode, encode, Header, TokenData, Validation};
use std::env;
use std::error::Error;
use uuid::Uuid;
use validator::{Validate, ValidationError};

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

pub struct Authorization {
    identity: String,
}

impl FromRequest for Authorization {
    type Error = AppError;
    type Future = Ready<Result<Self, AppError>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(identity) = req.get_identity() {
            ok(Authorization { identity })
        } else {
            err(AppError::Forbidden(json!("Unauthorized")))
        }

        // match maybeIdentity {
        //     Some(identi)
        // }
        // if  {
        //     ok(Authorized { req })
        // } else {
        //     Err(ErrorUnauthorized("not authorized"))?
        // }
    }
}
