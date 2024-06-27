use argon2::Argon2;
use scrypt::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use scrypt::Scrypt;
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

#[derive(Clone, Debug)]
pub struct HashPbkdf2 {
    byte_length: usize,
}

impl Default for HashPbkdf2 {
    fn default() -> Self {
        Self { byte_length: 32 }
    }
}

impl HashPbkdf2 {
    pub fn new(size: usize) -> Self {
        Self { byte_length: size }
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
 scrypt
 - https://docs.rs/argon2/latest/argon2/
*******************************************************************************/

#[derive(Clone, Debug)]
pub struct HashScrypt {
    salt: SaltString,
    //params: scrypt::Params,
}

impl Default for HashScrypt {
    fn default() -> Self {
        let salt = SaltString::generate(&mut OsRng);
        //let params = scrypt::Params::new(18, 8, 1, 32).unwrap();

        Self { salt }
    }
}

impl HashScrypt {
    pub fn new(salt: SaltString) -> Self {
        Self { salt }
    }
}

// FIXME: extremely slow
impl Hash for HashScrypt {
    fn hash(&self, data: &[u8]) -> anyhow::Result<String> {
        // [too slow]
        // Ok(Scrypt
        //     .hash_password_customized(data, None, None, self.params, &self.salt)?
        //     .to_string())
        Ok(Scrypt.hash_password(data, &self.salt)?.to_string())
    }
}

/******************************************************************************
 Test
******************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbkdf2_hash() {
        let password = "foobar".as_bytes();
        let hash = HashPbkdf2::new(32);
        let hashed = hash.hash(password).expect("fail to hash");

        assert_eq!(
            hashed,
            "1550b10bc2d5591908047861f0a8345b21798855407b5951aee7cf0edd39e318".to_string()
        );
    }

    #[ignore = "ignore because function is too slow"]
    #[test]
    fn test_scrypt_hash() {
        let password = "foobar".as_bytes();

        let salt = SaltString::from_b64("oKUiVnIPlVQtm1T19IctrA").expect("fail to make salt");
        let hash = HashScrypt::new(salt);
        let hashed = hash.hash(password).expect("fail to hash");

        assert_eq!(
            hashed,
            "$scrypt$ln=17,r=8,p=1$oKUiVnIPlVQtm1T19IctrA$VLVIRYqGANK8i6Yc9oXsKvPe1BZhFqcq4H5APR28K7Q".to_string()
        );
    }
}
