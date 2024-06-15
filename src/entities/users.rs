use crate::dbs::users as db_users;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginBody {
    #[validate(length(min = 8, max = 50))]
    pub email: String,
    #[validate(length(min = 10, max = 20))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
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
impl UserBody {
    // TODO: implement
    pub fn to_user_model(&self) -> db_users::Model {
        db_users::Model {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "password".to_string(),
            is_admin: true,
            created_at: None,
        }
    }
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
