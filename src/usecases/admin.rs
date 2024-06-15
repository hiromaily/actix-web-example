use crate::dbs::users as db_users;
use crate::entities::users;
use crate::repositories::{todos, users as repo_users};
use anyhow;
use std::sync::Arc;

pub trait AdminUsecase: Send + Sync + 'static {
    fn admin_login(&self, email: &String, password: &String) -> anyhow::Result<()>;
    fn get_user_list(&self) -> Vec<db_users::Model>;
    fn add_user(&self, user_body: users::UserBody) -> anyhow::Result<db_users::Model>;
    fn get_user(&self, user_id: i32) -> Option<db_users::Model>;
    fn update_user(
        &self,
        user_id: i32,
        user_body: users::UserUpdateBody,
    ) -> anyhow::Result<db_users::Model>;
    fn delete_user(&self, user_id: i32) -> anyhow::Result<()>;
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
    fn admin_login(&self, email: &String, password: &String) -> anyhow::Result<()> {
        Ok(())
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

    // TODO: implementation
    fn add_user(&self, user_body: users::UserBody) -> anyhow::Result<db_users::Model> {
        Ok(db_users::Model {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "password".to_string(),
            is_admin: true,
            created_at: None,
        })
    }

    // TODO: implementation
    fn get_user(&self, user_id: i32) -> Option<db_users::Model> {
        match user_id {
            1 => Some(db_users::Model {
                id: 1,
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "password".to_string(),
                is_admin: true,
                created_at: None,
            }),
            _ => None, // User with user_id not found
        }
    }

    // TODO: implementation
    fn update_user(
        &self,
        user_id: i32,
        user_body: users::UserUpdateBody,
    ) -> anyhow::Result<db_users::Model> {
        Ok(db_users::Model {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "password".to_string(),
            is_admin: true,
            created_at: None,
        })
    }

    // TODO: implementation
    fn delete_user(&self, user_id: i32) -> anyhow::Result<()> {
        Ok(())
    }
}
