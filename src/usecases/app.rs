use crate::entities::todos;
use crate::hash;
use crate::repositories::{todos as repo_todos, users as repo_users};
use crate::schemas::todos as db_todos;
use anyhow;
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;

#[async_trait]
pub trait AppUsecase: Send + Sync + 'static {
    async fn app_login(&self, email: &str, password: &str) -> anyhow::Result<bool>;
    async fn get_user_todo_list(&self, user_id: i32) -> anyhow::Result<Vec<db_todos::Model>>;
    async fn add_user_todo(
        &self,
        user_id: i32,
        user_body: todos::TodoBody,
    ) -> anyhow::Result<db_todos::Model>;
    async fn get_user_todo(
        &self,
        user_id: i32,
        todo_id: i32,
    ) -> anyhow::Result<Option<db_todos::Model>>;
    async fn update_user_todo(
        &self,
        user_id: i32,
        todo_id: i32,
        todo_body: todos::TodoUpdateBody,
    ) -> anyhow::Result<Option<db_todos::Model>>;
    async fn delete_user_todo(&self, user_id: i32, todo_id: i32) -> anyhow::Result<u64>;
}

#[derive(Debug)]
pub struct AppAction {
    pub todos_repo: Arc<dyn repo_todos::TodoRepository>,
    pub users_repo: Arc<dyn repo_users::UserRepository>,
    pub hash: Arc<dyn hash::Hash>,
}

impl AppAction {
    pub fn new(
        todos_repo: Arc<dyn repo_todos::TodoRepository>,
        users_repo: Arc<dyn repo_users::UserRepository>,
        hash: Arc<dyn hash::Hash>,
    ) -> Self {
        AppAction {
            todos_repo,
            users_repo,
            hash,
        }
    }
}

#[async_trait]
impl AppUsecase for AppAction {
    async fn app_login(&self, email: &str, password: &str) -> anyhow::Result<bool> {
        // TODO: query without is_admin condition
        const IS_ADMIN: bool = false;

        // hash
        let hash_password = self.hash.hash(password.as_bytes())?;

        let ret = self
            .users_repo
            .find(email, hash_password.as_str(), IS_ADMIN)
            .await?;
        match ret {
            Some(user) => {
                // Handle the case where a user is found
                debug!("User found: {:?}", user);
                Ok(true)
            }
            None => {
                // Handle the case where no user is found
                debug!("No user found");
                Ok(false)
            }
        }
    }

    async fn get_user_todo_list(&self, user_id: i32) -> anyhow::Result<Vec<db_todos::Model>> {
        let ret = self.todos_repo.find_all(user_id).await?;
        Ok(ret)
        // vec![db_todos::Model {
        //     id: 1,
        //     user_id: 1,
        //     title: "Do something".to_string(),
        //     description: Some("foo bar foo bar".to_string()),
        //     status: sea_orm_active_enums::TodoStatus::Pending,
        //     created_at: None,
        //     updated_at: None,
        // }]
    }

    async fn add_user_todo(
        &self,
        user_id: i32,
        todo_body: todos::TodoBody,
    ) -> anyhow::Result<db_todos::Model> {
        let ret = self.todos_repo.create(user_id, todo_body).await?;
        Ok(ret)
        // Ok(db_todos::Model {
        //     id: 1,
        //     user_id: 1,
        //     title: "Do something".to_string(),
        //     description: Some("foo bar foo bar".to_string()),
        //     status: sea_orm_active_enums::TodoStatus::Pending,
        //     created_at: None,
        //     updated_at: None,
        // })
    }

    async fn get_user_todo(
        &self,
        _user_id: i32,
        todo_id: i32,
    ) -> anyhow::Result<Option<db_todos::Model>> {
        let ret = self.todos_repo.find_by_id(todo_id).await?;
        Ok(ret)
        // match user_id {
        //     1 => Some(db_todos::Model {
        //         id: 1,
        //         user_id: 1,
        //         title: "Do something".to_string(),
        //         description: Some("foo bar foo bar".to_string()),
        //         status: sea_orm_active_enums::TodoStatus::Pending,
        //         created_at: None,
        //         updated_at: None,
        //     }),
        //     _ => None, // User with user_id not found
        // }
    }

    async fn update_user_todo(
        &self,
        _user_id: i32,
        todo_id: i32,
        todo_body: todos::TodoUpdateBody,
    ) -> anyhow::Result<Option<db_todos::Model>> {
        let ret = self.todos_repo.update(todo_id, todo_body).await?;
        Ok(ret)
        // Ok(db_todos::Model {
        //     id: 1,
        //     user_id: 1,
        //     title: "Do something".to_string(),
        //     description: Some("foo bar foo bar".to_string()),
        //     status: sea_orm_active_enums::TodoStatus::Pending,
        //     created_at: None,
        //     updated_at: None,
        // })
    }

    async fn delete_user_todo(&self, _user_id: i32, todo_id: i32) -> anyhow::Result<u64> {
        let ret = self.todos_repo.delete(todo_id).await?;
        Ok(ret)
    }
}
