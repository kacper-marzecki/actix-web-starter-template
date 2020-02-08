use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String
}
