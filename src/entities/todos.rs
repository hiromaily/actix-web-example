use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn validate_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "canceled" | "doing" | "done" | "pending" => Ok(()),
        _ => Err(ValidationError::new("invalid status")),
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TodoBody {
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    #[validate(length(min = 1, max = 200))]
    pub description: Option<String>,
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TodoUpdateBody {
    #[validate(length(min = 1, max = 50))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 200))]
    pub description: Option<String>,
    #[validate(length(min = 1), custom(function = "validate_status"))]
    pub status: String,
}
