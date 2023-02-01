use crate::db::Database;
use anyhow::Error;
use lazy_regex::regex;
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use std::sync::Arc;

const MIN_PASSWORD_LEN: usize = 8;

pub struct UserManager {
    db: Arc<Database>,
}

impl UserManager {
    pub fn new(db: Arc<Database>) -> Self {
        UserManager { db }
    }

    pub async fn register<S: Into<String>>(&self, email: S, password: &str) -> Result<(), Error> {
        let email = email.into();
        let email_regex = regex!("^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$");
        if !email_regex.is_match(&email) {
            return Err(Error::msg("Invalid email"));
        }
        if password.len() < MIN_PASSWORD_LEN {
            return Err(Error::msg("Password too short"));
        }
        if !password.chars().any(|c| matches!(c, 'a'..='z')) {
            return Err(Error::msg("Password must contain a lowercase letter"));
        }
        if !password.chars().any(|c| matches!(c, 'A'..='Z')) {
            return Err(Error::msg("Password must contain an uppercase letter"));
        }
        if !password.chars().any(|c| matches!(c, '0'..='9')) {
            return Err(Error::msg("Password must contain a number"));
        }

        let password_salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2
            .hash_password(password.as_bytes(), &password_salt)
            .map_err(|e| Error::msg(e.to_string()))?;
        self.db
            .insert_user(email, password_hash.to_string())
            .await?;
        Ok(())
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(), Error> {
        let user = self.db.get_user(email).await?;
        let parsed_hash = PasswordHash::new(user.password_hash.as_str())
            .map_err(|e| Error::msg(e.to_string()))?;
        if let Err(_) = Pbkdf2.verify_password(password.as_bytes(), &parsed_hash) {
            return Err(Error::msg("Incorrect password"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::db::Database;
    use crate::user_manager::UserManager;

    #[tokio::test]
    async fn test_register_and_login() {
        let user_manager = create_user_manager().await;
        user_manager.register(TEST_EMAIL, TEST_PASSWORD).await.unwrap();
        user_manager.login(TEST_EMAIL, TEST_PASSWORD).await.unwrap();
    }

    #[tokio::test]
    async fn test_registration_rejections() {
        let user_manager = create_user_manager().await;

        // List of invalid registrations. Tuples contain: (email, password, problem)
        let bad_registrations = [
            ("invalid_email", "", "Invalid email"),
            ("test@example.com", "short", "Short password"),
            ("test@example.com", "looooong", "Weak password (no uppercase or numbers)"),
            ("test@example.com", "Looooong", "Weak password (no numbers)"),
            ("test@example.com", "LOOOOONG", "Weak password (no lowercase or numbers)"),
            ("test@example.com", "L00000NG", "Weak password (no lowercase)"),
            ("test@example.com", "12345678", "Weak password (no lowercase or uppercase)"),
        ];
        for registration in bad_registrations {
            user_manager.register(registration.0, registration.1)
                .await
                .expect_err(format!("{} was OK", registration.2).as_str());
        }
    }

    #[tokio::test]
    async fn test_register_reject_duplicates() {
        let user_manager = create_user_manager().await;
        let email = "test@email.com";
        user_manager.register(email, TEST_PASSWORD).await.unwrap();
        let duplicate_emails = [
            email,
            "TEST@EMAIL.COM",
            "Test@Email.Com",
        ];
        for email in duplicate_emails {
            user_manager.register(email, TEST_PASSWORD)
                .await
                .expect_err(format!("Duplicate email was registered: {email}").as_str());
        }
    }

    #[tokio::test]
    async fn test_login_rejections() {
        let user_manager = create_user_manager().await;
        user_manager.register(TEST_EMAIL, TEST_PASSWORD).await.unwrap();

        // List of invalid logins. Tuples contain: (email, password, problem)
        let bad_logins = [
            ("unregisterd@email.com", TEST_PASSWORD, "Incorrect email"),
            (TEST_EMAIL, "password", "Incorrect password"),
        ];
        for login in bad_logins {
            user_manager.login(login.0, login.1)
                .await
                .expect_err(format!("{} was OK", login.2).as_str());
        }
    }

    async fn create_user_manager() -> UserManager {
        UserManager::new(Arc::new(Database::new_in_memory().await.unwrap()))
    }

    const TEST_EMAIL: &str = "test@example.com";
    const TEST_PASSWORD: &str = "Passw0rd";
}
