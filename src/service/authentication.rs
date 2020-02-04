use actix_web::web::Data;
use crate::app::app_state::AppState;
use actix_web::HttpRequest;
use futures::Future;
use crate::common::{AppError, AppResult};
use actix_http::http::header::AUTHORIZATION;
use crate::repository::user::{Authenticate, Authentication};
use actix_http::http::HeaderValue;
use std::error::Error;
use jsonwebtoken::{decode, encode, Header, TokenData, Validation};
use std::env;
use uuid::Uuid;

const TOKEN_PREFIX: &str = "Token ";
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: Uuid,
    pub exp: i64,
}

pub fn authenticate(state: &Data<AppState>, req: &HttpRequest) -> impl Future<Output=Result<Authentication, AppError>> {
    let db = state.db.clone();
    let token = extract_token(req);
    result(token)
        .and_then(move |token| db.send(Authenticate {
            claims_id: decode_jwt(token),
            token,
        }).from_err())
        .flatten()
}

fn extract_token(request: &HttpRequest) -> Result<String, AppError> {
    let header = request.headers()
        .get(AUTHORIZATION);
    let token = match header {
        Some(token) => token.to_str().unwrap(),
        None => {
            return Err(AppError::Unauthorized(json!({
                "common": "No authorization was provided",
            })));
        }
    };

    if !token.starts_with(TOKEN_PREFIX) {
        return Err(AppError::Unauthorized(json!({
            "common": "Invalid authorization method",
        })));
    }

    let token = token.replacen(TOKEN_PREFIX, "", 1);

    Ok(token)
}

fn decode_jwt(token: &String) -> AppResult<TokenData<Claims>> {
    match decode::<Claims>(token, get_secret().as_ref(), &Validation::default()) {
        Ok(res) => Ok(res),
        Err(e) => Err(e.into()),
    }
}

fn get_secret() -> String {
env::var("JWT_SECRET").unwrap_or_else( |_ | String::from("secret"))
}
