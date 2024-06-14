use crate::repositories::{todos, users};
use std::sync::Arc;

pub trait AdminUsecase: Send + Sync + 'static {
    fn admin_login(&self, email: &String, password: &String) -> bool;
}

#[derive(Debug)]
pub struct AdminAction {
    pub todos_repo: Arc<dyn todos::TodoRepository>,
    pub users_repo: Arc<dyn users::UserRepository>,
}

impl AdminAction {
    pub fn new(
        todos_repo: Arc<dyn todos::TodoRepository>,
        users_repo: Arc<dyn users::UserRepository>,
    ) -> Self {
        AdminAction {
            todos_repo,
            users_repo,
        }
    }
}

impl AdminUsecase for AdminAction {
    fn admin_login(&self, email: &String, password: &String) -> bool {
        true
    }
}
