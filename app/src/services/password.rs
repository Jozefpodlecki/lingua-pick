use phc::SaltString;
use argon2::{password_hash::{Salt}, Argon2, PasswordHash, PasswordVerifier};
use anyhow::{anyhow, Result};
use argon2::PasswordHasher;
use rand::rngs::SysRng;

pub struct AppPasswordHasher;

impl AppPasswordHasher {
    pub fn new() -> Self {
        Self
    }

    pub fn hash(&self, password: &str) -> Result<Vec<u8>> {
        let salt_str = SaltString::try_from_rng(&mut SysRng)?;
        let salt = Salt::from_b64(&salt_str).map_err(|err| anyhow!(err))?;
        let argon2 = Argon2::default();
        let result = argon2.hash_password(password.as_bytes(), salt).map_err(|err| anyhow!(err))?;

        Ok(result.to_string().as_bytes().into())
    }

    pub fn verify(&self, password: &[u8], hash: &[u8]) -> Result<bool> {
        let hash_str = str::from_utf8(hash)?;
        let parsed_hash = PasswordHash::new(hash_str).map_err(|err| anyhow!(err))?;
        let argon2 = Argon2::default();
        let matched = parsed_hash.verify_password(&[&argon2], password).is_ok();

        Ok(matched)
    }
}