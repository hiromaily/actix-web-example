use jwt_simple::prelude::*;
use std::{
    fmt::Debug,
    marker::{Send, Sync},
};

pub trait JWT: Debug + Send + Sync + 'static {
    fn issue(&self, payload: PayLoad) -> anyhow::Result<String>;
    fn validate(&self, token: &str) -> anyhow::Result<PayLoad>;
    //fn validate_with_id(&self, token: &str, user_id: i32) -> anyhow::Result<bool>;
}

/*******************************************************************************
 jwt_simple
 - https://docs.rs/jwt-simple/0.11.9/jwt_simple/index.html
*******************************************************************************/

#[derive(Serialize, Deserialize, Debug)]
pub struct PayLoad {
    pub user_id: u64,
    pub email: String,
    pub is_admin: bool,
}

impl PayLoad {
    pub fn new(user_id: u64, email: String, is_admin: bool) -> Self {
        Self {
            user_id,
            email,
            is_admin,
        }
    }
}

#[derive(Debug)]
pub struct SimpleJWT {
    token_key: HS256Key,
    duration: u64,
}

impl Default for SimpleJWT {
    fn default() -> Self {
        Self {
            token_key: HS256Key::generate(),
            duration: 1,
        }
    }
}

impl SimpleJWT {
    pub fn new(duration: u64) -> Self {
        Self {
            token_key: HS256Key::generate(),
            duration,
        }
    }
}

// refer to: https://www.abc.osaka/actix/jwt-token
impl JWT for SimpleJWT {
    // issue access token
    // issue is called after login succeeded
    fn issue(&self, payload: PayLoad) -> anyhow::Result<String> {
        //let claims = Claims::create(Duration::from_hours(1));
        let claims = Claims::with_custom_claims(payload, Duration::from_hours(self.duration));

        // sign
        let token = self.token_key.authenticate(claims)?;
        Ok(token)
    }

    // TODO: after validating, is_admin value needs to be returned
    fn validate(&self, token: &str) -> anyhow::Result<PayLoad> {
        // let claim = self
        //     .token_key
        //     .verify_token::<NoCustomClaims>(token.as_str(), None)?;
        let claims = self.token_key.verify_token::<PayLoad>(token, None)?;
        Ok(claims.custom)
    }

    // TODO: after validating, is_admin value needs to be returned
    // - need to return is_admin
    // fn validate_with_id(&self, token: &str, user_id: i32) -> anyhow::Result<bool> {
    //     let claims = self.token_key.verify_token::<PayLoad>(token, None)?;
    //     if claims.custom.user_id == user_id as u64 {
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }
    // }
}
