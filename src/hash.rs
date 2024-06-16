use argon2::Argon2;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::{
    fmt::Debug,
    marker::{Send, Sync},
};

pub trait Hash: Debug + Send + Sync + 'static {
    fn hash(&self, data: &[u8]) -> anyhow::Result<String>;
}

/*******************************************************************************
 pbkdf2
 - https://docs.rs/argon2/latest/argon2/
*******************************************************************************/

#[derive(Debug, Default)]
pub struct HashPbkdf2 {
    byte_length: usize,
}

impl HashPbkdf2 {
    pub fn new() -> Self {
        Self { byte_length: 32 }
    }
}

fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut sha256 = Sha256::default();
    sha256.update(data);
    sha256.finalize().to_vec()
}

impl Hash for HashPbkdf2 {
    fn hash(&self, data: &[u8]) -> anyhow::Result<String> {
        let salt = sha256_hash(data);
        let mut key: Vec<u8> = vec![0u8; self.byte_length];

        Argon2::default()
            .hash_password_into(data, salt.as_ref(), &mut key)
            .map_err(|err| anyhow::Error::msg(format!("Failed to hash, error: {}", err)))?;

        // convert to String
        // FIXME: better code as performance wise
        let mut hashed_str = String::with_capacity(key.len() * 2); // Each byte is 2 hex characters
        for byte in &key {
            write!(hashed_str, "{:02x}", byte).expect("Unable to write");
        }

        Ok(hashed_str)
    }
}

/*******************************************************************************
 secret
*******************************************************************************/

// #[derive(Clone)]

// pub struct Secret(Vec<u8>);

// impl Secret {
//     pub fn new(data: Vec<u8>) -> Self {
//         Self(data)
//     }

//     pub fn as_str(&self) -> Result<&str> {
//         Ok(std::str::from_utf8(&self.0)?)
//     }
// }

// impl AsRef<[u8]> for Secret {
//     fn as_ref(&self) -> &[u8] {
//         &self.0
//     }
// }

// impl From<Vec<u8>> for Secret {
//     fn from(vec: Vec<u8>) -> Self {
//         Secret(vec)
//     }
// }

// impl From<&[u8]> for Secret {
//     fn from(slice: &[u8]) -> Self {
//         Secret(slice.to_vec())
//     }
// }

// impl From<&str> for Secret {
//     fn from(s: &str) -> Self {
//         Secret(s.trim().as_bytes().to_vec())
//     }
// }

// impl std::fmt::Debug for Secret {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Secret")
//             .field("secret", &"********")
//             .finish()
//     }
// }
