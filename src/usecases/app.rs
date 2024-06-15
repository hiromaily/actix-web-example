use crate::dbs::{sea_orm_active_enums, todos as db_todos};
use crate::entities::todos;
use crate::repositories::{todos as repo_todos, users as repo_users};
use anyhow;
use std::sync::Arc;

pub trait AppUsecase: Send + Sync + 'static {
    fn app_login(&self, email: &String, password: &String) -> anyhow::Result<()>;
    fn get_user_todo_list(&self, user_id: i32) -> Vec<db_todos::Model>;
    fn add_user_todo(&self, user_body: todos::TodoBody) -> anyhow::Result<db_todos::Model>;
    fn get_user_todo(&self, user_id: i32, todo_id: i32) -> Option<db_todos::Model>;
    fn update_user_todo(
        &self,
        user_id: i32,
        todo_id: i32,
        todo_body: todos::TodoUpdateBody,
    ) -> anyhow::Result<db_todos::Model>;
    fn delete_user_todo(&self, user_id: i32, todo_id: i32) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct AppAction {
    pub todos_repo: Arc<dyn repo_todos::TodoRepository>,
    pub users_repo: Arc<dyn repo_users::UserRepository>,
}

impl AppAction {
    pub fn new(
        todos_repo: Arc<dyn repo_todos::TodoRepository>,
        users_repo: Arc<dyn repo_users::UserRepository>,
    ) -> Self {
        AppAction {
            todos_repo,
            users_repo,
        }
    }
}

impl AppUsecase for AppAction {
    // TODO: implementation
    fn app_login(&self, email: &String, password: &String) -> anyhow::Result<()> {
        Ok(())
    }

    // TODO: implementation
    fn get_user_todo_list(&self, user_id: i32) -> Vec<db_todos::Model> {
        vec![db_todos::Model {
            id: 1,
            user_id: 1,
            title: "Do something".to_string(),
            description: Some("foo bar foo bar".to_string()),
            status: sea_orm_active_enums::TodoStatus::Pending,
            created_at: None,
            updated_at: None,
        }]
    }

    // TODO: implementation
    fn add_user_todo(&self, todo_body: todos::TodoBody) -> anyhow::Result<db_todos::Model> {
        Ok(db_todos::Model {
            id: 1,
            user_id: 1,
            title: "Do something".to_string(),
            description: Some("foo bar foo bar".to_string()),
            status: sea_orm_active_enums::TodoStatus::Pending,
            created_at: None,
            updated_at: None,
        })
    }

    // TODO: implementation
    fn get_user_todo(&self, user_id: i32, todo_id: i32) -> Option<db_todos::Model> {
        match user_id {
            1 => Some(db_todos::Model {
                id: 1,
                user_id: 1,
                title: "Do something".to_string(),
                description: Some("foo bar foo bar".to_string()),
                status: sea_orm_active_enums::TodoStatus::Pending,
                created_at: None,
                updated_at: None,
            }),
            _ => None, // User with user_id not found
        }
    }

    // TODO: implementation
    fn update_user_todo(
        &self,
        user_id: i32,
        todo_id: i32,
        todo_body: todos::TodoUpdateBody,
    ) -> anyhow::Result<db_todos::Model> {
        Ok(db_todos::Model {
            id: 1,
            user_id: 1,
            title: "Do something".to_string(),
            description: Some("foo bar foo bar".to_string()),
            status: sea_orm_active_enums::TodoStatus::Pending,
            created_at: None,
            updated_at: None,
        })
    }

    // TODO: implementation
    fn delete_user_todo(&self, user_id: i32, todo_id: i32) -> anyhow::Result<()> {
        Ok(())
    }
}
