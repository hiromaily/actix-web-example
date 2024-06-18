use crate::hash;
use crate::jwt::{self, PayLoad};
use crate::repositories::users as repo_users;
use crate::schemas::users as db_users;
use anyhow;
use async_trait::async_trait;
use log::debug;
use std::sync::Arc;

#[async_trait]
pub trait AuthUsecase: Send + Sync + 'static {
    async fn login(&self, email: &str, password: &str) -> anyhow::Result<Option<db_users::Model>>;
    fn generate_token(&self, user_id: i32, email: &str, is_admin: bool) -> anyhow::Result<String>;
    fn validate_token(&self, token: &str) -> anyhow::Result<()>;
}

/*******************************************************************************
 Auth for Admin
*******************************************************************************/

#[derive(Debug)]
pub struct AuthAdminAction {
    pub users_repo: Arc<dyn repo_users::UserRepository>,
    pub hash: Arc<dyn hash::Hash>,
    pub jwt: Arc<dyn jwt::JWT>,
}

impl AuthAdminAction {
    pub fn new(
        users_repo: Arc<dyn repo_users::UserRepository>,
        hash: Arc<dyn hash::Hash>,
        jwt: Arc<dyn jwt::JWT>,
    ) -> Self {
        Self {
            users_repo,
            hash,
            jwt,
        }
    }
}

#[async_trait]
impl AuthUsecase for AuthAdminAction {
    // return user_id if exist, but return 0 if not exist
    async fn login(&self, email: &str, password: &str) -> anyhow::Result<Option<db_users::Model>> {
        const IS_ADMIN: bool = true;

        // hash
        let hash_password = self.hash.hash(password.as_bytes())?;
        debug!("hash_password is {}", hash_password);

        self.users_repo
            .find(email, hash_password.as_str(), IS_ADMIN)
            .await
    }

    fn generate_token(&self, user_id: i32, email: &str, is_admin: bool) -> anyhow::Result<String> {
        let payload = PayLoad::new(user_id as u64, email.to_string(), is_admin);
        let token = self.jwt.issue(payload)?;
        Ok(token)
    }
    fn validate_token(&self, token: &str) -> anyhow::Result<()> {
        self.jwt.validate(token)?;
        Ok(())
    }
}

/*******************************************************************************
 Auth for App
*******************************************************************************/

#[derive(Debug)]
pub struct AuthAppAction {
    pub users_repo: Arc<dyn repo_users::UserRepository>,
    pub hash: Arc<dyn hash::Hash>,
    pub jwt: Arc<dyn jwt::JWT>,
}

impl AuthAppAction {
    pub fn new(
        users_repo: Arc<dyn repo_users::UserRepository>,
        hash: Arc<dyn hash::Hash>,
        jwt: Arc<dyn jwt::JWT>,
    ) -> Self {
        Self {
            users_repo,
            hash,
            jwt,
        }
    }
}

#[async_trait]
impl AuthUsecase for AuthAppAction {
    async fn login(&self, email: &str, password: &str) -> anyhow::Result<Option<db_users::Model>> {
        // TODO: query without is_admin condition
        const IS_ADMIN: bool = false;

        // hash
        let hash_password = self.hash.hash(password.as_bytes())?;
        self.users_repo
            .find(email, hash_password.as_str(), IS_ADMIN)
            .await
    }

    fn generate_token(&self, user_id: i32, email: &str, is_admin: bool) -> anyhow::Result<String> {
        let payload = PayLoad::new(user_id as u64, email.to_string(), is_admin);
        let token = self.jwt.issue(payload)?;
        Ok(token)
    }
    fn validate_token(&self, token: &str) -> anyhow::Result<()> {
        self.jwt.validate(token)?;
        Ok(())
    }
}
