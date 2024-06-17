use crate::entities::users;
use crate::hash;
use crate::repositories::{todos as repo_todos, users as repo_users};
use crate::schemas::users as db_users;
use anyhow;
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;

#[async_trait]
pub trait AdminUsecase: Send + Sync + 'static {
    async fn admin_login(&self, email: &str, password: &str) -> anyhow::Result<bool>;
    async fn get_user_list(&self) -> anyhow::Result<Vec<db_users::Model>>;
    async fn add_user(&self, user_body: users::UserBody) -> anyhow::Result<db_users::Model>;
    async fn get_user(&self, user_id: i32) -> anyhow::Result<Option<db_users::Model>>;
    async fn update_user(
        &self,
        user_id: i32,
        user_body: users::UserUpdateBody,
    ) -> anyhow::Result<Option<db_users::Model>>;
    async fn delete_user(&self, user_id: i32) -> anyhow::Result<u64>;
}

#[derive(Debug)]
pub struct AdminAction {
    pub todos_repo: Arc<dyn repo_todos::TodoRepository>,
    pub users_repo: Arc<dyn repo_users::UserRepository>,
    pub hash: Arc<dyn hash::Hash>,
}

impl AdminAction {
    pub fn new(
        todos_repo: Arc<dyn repo_todos::TodoRepository>,
        users_repo: Arc<dyn repo_users::UserRepository>,
        hash: Arc<dyn hash::Hash>,
    ) -> Self {
        AdminAction {
            todos_repo,
            users_repo,
            hash,
        }
    }
}

#[async_trait]
impl AdminUsecase for AdminAction {
    async fn admin_login(&self, email: &str, password: &str) -> anyhow::Result<bool> {
        const IS_ADMIN: bool = true;

        // hash
        let hash_password = self.hash.hash(password.as_bytes())?;
        debug!("hash_password is {}", hash_password);

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

    async fn add_user(&self, user_body: users::UserBody) -> anyhow::Result<db_users::Model> {
        // hash
        let hashed_password = self.hash.hash(user_body.password.as_bytes())?;

        // new object
        let updated_user_body = users::UserBody {
            password: hashed_password,
            ..user_body // Copy other fields from the original user_body
        };

        let ret = self.users_repo.create(updated_user_body).await?;
        Ok(ret)
        // Ok(db_users::Model {
        //     id: 1,
        //     first_name: "John".to_string(),
        //     last_name: "Doe".to_string(),
        //     email: "john.doe@example.com".to_string(),
        //     password: "password".to_string(),
        //     is_admin: true,
        //     created_at: None,
        // })
    }

    async fn get_user(&self, user_id: i32) -> anyhow::Result<Option<db_users::Model>> {
        let ret = self.users_repo.find_by_id(user_id).await?;
        Ok(ret)
        // match user_id {
        //     1 => Some(db_users::Model {
        //         id: 1,
        //         first_name: "John".to_string(),
        //         last_name: "Doe".to_string(),
        //         email: "john.doe@example.com".to_string(),
        //         password: "password".to_string(),
        //         is_admin: true,
        //         created_at: None,
        //     }),
        //     _ => None, // User with user_id not found
        // }
    }

    async fn update_user(
        &self,
        user_id: i32,
        user_body: users::UserUpdateBody,
    ) -> anyhow::Result<Option<db_users::Model>> {
        // if user_body contains password, it must be hashed
        let hashed_password = if let Some(_password) = &user_body.password {
            // Hash the password
            Some(self.hash.hash(user_body.password.unwrap().as_bytes())?)
        } else {
            None
        };

        // create new object
        let updated_user_body = users::UserUpdateBody {
            password: hashed_password,
            ..user_body // Copy other fields from the original user_body
        };

        let ret = self.users_repo.update(user_id, updated_user_body).await?;
        Ok(ret)
        // Ok(db_users::Model {
        //     id: 1,
        //     first_name: "John".to_string(),
        //     last_name: "Doe".to_string(),
        //     email: "john.doe@example.com".to_string(),
        //     password: "password".to_string(),
        //     is_admin: true,
        //     created_at: None,
        // })
    }

    async fn delete_user(&self, user_id: i32) -> anyhow::Result<u64> {
        let ret = self.users_repo.delete(user_id).await?;
        Ok(ret)
    }
}
