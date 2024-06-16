use serde::{Deserialize, Serialize};
use validator::Validate;

// (...Clone, PartialEq, Eq) if needed
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginBody {
    #[validate(length(min = 8, max = 50))]
    pub email: String,
    #[validate(length(min = 10, max = 20))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone, PartialEq, Eq)]
pub struct UserBody {
    #[validate(length(min = 1, max = 50))]
    pub first_name: String,
    #[validate(length(min = 1, max = 50))]
    pub last_name: String,
    #[validate(length(min = 8, max = 50))]
    pub email: String,
    #[validate(length(min = 10, max = 20))]
    pub password: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserUpdateBody {
    #[validate(length(min = 1, max = 50))]
    pub first_name: Option<String>,
    #[validate(length(min = 1, max = 50))]
    pub last_name: Option<String>,
    #[validate(length(min = 8, max = 50))]
    pub email: Option<String>,
    #[validate(length(min = 10, max = 20))]
    pub password: Option<String>,
    pub is_admin: Option<bool>,
}
