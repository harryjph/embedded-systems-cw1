use std::sync::Arc;
use anyhow::Error;
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHasher, SaltString};
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

    pub async fn add_user(&self, username: String, email: String, password: String) -> Result<(), Error> {
        let password_salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(password.as_bytes(), &password_salt)
            .map_err(|e| Error::msg(e.to_string()))?;
        self.db.insert_user(
            username,
            email,
            password_hash.to_string(),
            password_salt.to_string()
        ).await?;
        Ok(())
    }
}
