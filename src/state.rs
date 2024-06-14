use crate::repositories::{todos, users};
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct AppState {
    pub app_name: String,
    pub todos_repo: Arc<dyn todos::TodoRepository>,
    pub users_repo: Arc<dyn users::UserRepository>,
}
