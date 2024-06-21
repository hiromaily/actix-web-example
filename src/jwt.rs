use crate::rand::generate_secret;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    marker::{Send, Sync},
    time::{Duration as TimeDuration, SystemTime, UNIX_EPOCH},
};

/*
 Payload
*/
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PayLoadExp {
    pub user_id: u64,
    pub email: String,
    pub is_admin: bool,
    pub exp: usize, // for only JsonWebToken
}

impl PayLoadExp {
    pub fn to_payload(payload: &PayLoad, exp: usize) -> Self {
        Self {
            user_id: payload.user_id,
            email: payload.email.clone(),
            is_admin: payload.is_admin,
            exp,
        }
    }
}

pub trait JWT: Debug + Send + Sync + 'static {
    fn issue(&self, payload: PayLoad) -> anyhow::Result<String>;
    fn validate(&self, token: &str) -> anyhow::Result<PayLoad>;
    //fn validate_with_id(&self, token: &str, user_id: i32) -> anyhow::Result<bool>;
}

/*******************************************************************************
 jwt_simple
 - https://docs.rs/jwt-simple/0.11.9/jwt_simple/index.html
*******************************************************************************/

#[derive(Debug)]
pub struct SimpleJWT {
    token_key: HS256Key,
    duration_min: u64,
}

impl Default for SimpleJWT {
    fn default() -> Self {
        Self {
            token_key: HS256Key::generate(),
            duration_min: 30,
        }
    }
}

impl SimpleJWT {
    pub fn new(duration_min: u64) -> Self {
        Self {
            token_key: HS256Key::generate(),
            duration_min,
        }
    }
}

// refer to: https://www.abc.osaka/actix/jwt-token
impl JWT for SimpleJWT {
    // issue access token
    // issue is called after login succeeded
    fn issue(&self, payload: PayLoad) -> anyhow::Result<String> {
        //let claims = Claims::create(Duration::from_hours(1));
        let claims = Claims::with_custom_claims(payload, Duration::from_mins(self.duration_min));

        // sign
        let token = self.token_key.authenticate(claims)?;
        Ok(token)
    }

    fn validate(&self, token: &str) -> anyhow::Result<PayLoad> {
        let claims = self.token_key.verify_token::<PayLoad>(token, None)?;
        Ok(claims.custom)
    }
}

/*******************************************************************************
 jsonwebtoken
 - https://crates.io/crates/jsonwebtoken
*******************************************************************************/

#[derive(Debug)]
pub struct JsonWebToken {
    token_key: String,
    duration_sec: u64,
}

impl Default for JsonWebToken {
    fn default() -> Self {
        Self {
            token_key: "secret".to_string(),
            duration_sec: 3600,
        }
    }
}

impl JsonWebToken {
    pub fn new(duration_sec: u64) -> Self {
        // generate
        let secret = generate_secret(32);

        Self {
            token_key: secret,
            duration_sec,
        }
    }
}

impl JWT for JsonWebToken {
    fn issue(&self, payload: PayLoad) -> anyhow::Result<String> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .checked_add(TimeDuration::from_secs(self.duration_sec)) // Token valid for 60 seconds
            .unwrap()
            .as_secs() as usize;

        let claims = PayLoadExp::to_payload(&payload, expiration);
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.token_key.as_ref()),
        )?;
        Ok(token)
    }

    fn validate(&self, token: &str) -> anyhow::Result<PayLoad> {
        let token_data = decode::<PayLoad>(
            token,
            &DecodingKey::from_secret(self.token_key.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(token_data.claims)
    }
}
