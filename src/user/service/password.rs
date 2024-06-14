use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
impl super::Service {
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(hash)?;
        argon2.verify_password(password.as_bytes(), &parsed_hash)?;
        Ok(())
    }
}
