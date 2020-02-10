use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::repository::schema::users;

#[derive(Debug, Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNewQuery {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserUpdateQuery {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}
