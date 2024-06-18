use crate::hash;
use crate::repositories::users as repo_users;
use anyhow;
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;

#[async_trait]
pub trait AuthUsecase: Send + Sync + 'static {
    async fn login(&self, email: &str, password: &str) -> anyhow::Result<bool>;
}

/*******************************************************************************
 Auth for Admin
*******************************************************************************/

#[derive(Debug)]
pub struct AuthAdminAction {
    pub users_repo: Arc<dyn repo_users::UserRepository>,
    pub hash: Arc<dyn hash::Hash>,
}

impl AuthAdminAction {
    pub fn new(users_repo: Arc<dyn repo_users::UserRepository>, hash: Arc<dyn hash::Hash>) -> Self {
        Self { users_repo, hash }
    }
}

#[async_trait]
impl AuthUsecase for AuthAdminAction {
    async fn login(&self, email: &str, password: &str) -> anyhow::Result<bool> {
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
}

/*******************************************************************************
 Auth for App
*******************************************************************************/

#[derive(Debug)]
pub struct AuthAppAction {
    pub users_repo: Arc<dyn repo_users::UserRepository>,
    pub hash: Arc<dyn hash::Hash>,
}

impl AuthAppAction {
    pub fn new(users_repo: Arc<dyn repo_users::UserRepository>, hash: Arc<dyn hash::Hash>) -> Self {
        Self { users_repo, hash }
    }
}

#[async_trait]
impl AuthUsecase for AuthAppAction {
    async fn login(&self, email: &str, password: &str) -> anyhow::Result<bool> {
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
}
