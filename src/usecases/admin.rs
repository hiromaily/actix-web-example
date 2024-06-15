use crate::dbs::users as db_users;
use crate::repositories::{todos, users as repo_users};
use std::sync::Arc;

pub trait AdminUsecase: Send + Sync + 'static {
    fn admin_login(&self, email: &String, password: &String) -> bool;
    fn get_user_list(&self) -> Vec<db_users::Model>;
}

#[derive(Debug)]
pub struct AdminAction {
    pub todos_repo: Arc<dyn todos::TodoRepository>,
    pub users_repo: Arc<dyn repo_users::UserRepository>,
}

impl AdminAction {
    pub fn new(
        todos_repo: Arc<dyn todos::TodoRepository>,
        users_repo: Arc<dyn repo_users::UserRepository>,
    ) -> Self {
        AdminAction {
            todos_repo,
            users_repo,
        }
    }
}

impl AdminUsecase for AdminAction {
    // TODO: implementation
    fn admin_login(&self, email: &String, password: &String) -> bool {
        true
    }

    // TODO: implementation
    fn get_user_list(&self) -> Vec<db_users::Model> {
        vec![db_users::Model {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "password".to_string(),
            is_admin: true,
            created_at: None,
        }]
    }
}
