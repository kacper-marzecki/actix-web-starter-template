use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String
}
