use crate::repositories::{todos, users};
use std::sync::Arc;

pub trait AppUsecase: Send + Sync + 'static {
    fn app_login(&self, email: &String, password: &String) -> bool;
}

#[derive(Debug)]
pub struct AppAction {
    pub todos_repo: Arc<dyn todos::TodoRepository>,
    pub users_repo: Arc<dyn users::UserRepository>,
}

impl AppAction {
    pub fn new(
        todos_repo: Arc<dyn todos::TodoRepository>,
        users_repo: Arc<dyn users::UserRepository>,
    ) -> Self {
        AppAction {
            todos_repo,
            users_repo,
        }
    }
}

impl AppUsecase for AppAction {
    fn app_login(&self, email: &String, password: &String) -> bool {
        true
    }
}
