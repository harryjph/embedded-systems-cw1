use std::sync::Arc;
use anyhow::Error;
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use crate::db::Database;

pub struct UserManager {
    db: Arc<Database>,
}

impl UserManager {
    pub fn new(db: Arc<Database>) -> Self {
        UserManager {
            db
        }
    }

    pub async fn register(&self, email: String, password: String) -> Result<(), Error> {
        let password_salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(password.as_bytes(), &password_salt)
            .map_err(|e| Error::msg(e.to_string()))?;
        self.db.insert_user(email, password_hash.to_string()).await?;
        Ok(())
    }

    pub async fn login(&self, email: String, password: String) -> Result<(), Error> {
        let user = self.db.get_user(email).await?;
        let parsed_hash = PasswordHash::new(user.password_hash.as_str())
            .map_err(|e| Error::msg(e.to_string()))?;
        if let Err(_) = Pbkdf2.verify_password(password.as_bytes(), &parsed_hash) {
            return Err(Error::msg("Incorrect password"));
        }
        Ok(())
    }
}
