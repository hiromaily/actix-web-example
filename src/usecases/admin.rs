use crate::entities::users;
use crate::repositories::{todos as repo_todos, users as repo_users};
use crate::schemas::users as db_users;
use anyhow;
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;

#[async_trait]
pub trait AdminUsecase: Send + Sync + 'static {
    async fn admin_login(&self, email: &String, password: &String) -> anyhow::Result<()>;
    async fn get_user_list(&self) -> anyhow::Result<Vec<db_users::Model>>;
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
    pub todos_repo: Arc<dyn repo_todos::TodoRepository>,
    pub users_repo: Arc<dyn repo_users::UserRepository>,
}

impl AdminAction {
    pub fn new(
        todos_repo: Arc<dyn repo_todos::TodoRepository>,
        users_repo: Arc<dyn repo_users::UserRepository>,
    ) -> Self {
        AdminAction {
            todos_repo,
            users_repo,
        }
    }
}

#[async_trait]
impl AdminUsecase for AdminAction {
    async fn admin_login(&self, email: &String, password: &String) -> anyhow::Result<()> {
        const IS_ADMIN: bool = true;

        let ret = self.users_repo.find(email, password, IS_ADMIN).await?;
        match ret {
            Some(user) => {
                // Handle the case where a user is found
                debug!("User found: {:?}", user);
                Ok(())
            }
            None => {
                // Handle the case where no user is found
                debug!("No user found");
                Ok(())
            }
        }
    }

    async fn get_user_list(&self) -> anyhow::Result<Vec<db_users::Model>> {
        let ret = self.users_repo.find_all().await?;
        Ok(ret)
        // vec![db_users::Model {
        //     id: 1,
        //     first_name: "John".to_string(),
        //     last_name: "Doe".to_string(),
        //     email: "john.doe@example.com".to_string(),
        //     password: "password".to_string(),
        //     is_admin: true,
        //     created_at: None,
        // }]
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
