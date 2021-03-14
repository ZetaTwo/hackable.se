use argonautica::Hasher;

use super::password::Password;

pub struct PasswordHashingConfig {
    secret_key: String,
}

impl PasswordHashingConfig {
    pub fn new(secret_key: String) -> PasswordHashingConfig {
        PasswordHashingConfig {
            secret_key: secret_key,
        }
    }
}

// TODO: Add display
#[derive(Debug)]
pub enum PasswordHashError {
    Hash(argonautica::Error),
}

// TODO: Implement display to not show hash
#[derive(Debug)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn hash_password(
        password: &Password,
        hash_config: &PasswordHashingConfig,
    ) -> Result<Self, PasswordHashError> {
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(password)
            .with_secret_key(&hash_config.secret_key)
            .hash();

        match hash {
            Ok(hash) => Ok(PasswordHash(hash)),
            // TODO: Granular error handling
            Err(err) => Err(PasswordHashError::Hash(err)),
        }
    }
}
