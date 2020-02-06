use validator::{Validate, ValidationError};


#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1))]
    pub password_repeat: String
}

