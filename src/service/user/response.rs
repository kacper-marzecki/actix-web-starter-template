use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}
