use argon2::Algorithm::Argon2id;
use argon2::password_hash::SaltString;
use argon2::{Argon2, Params, PasswordHasher, Version};
use rand::thread_rng;
// todo: proper error returns
pub async fn hash_password(password: &str) -> Result<String, &'static str> {
    let salt = SaltString::generate(&mut thread_rng());
    let password_hash = Argon2::new(
        Argon2id,
        Version::V0x13,
        // https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
        Params::new(19456, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)
    .unwrap()
    .to_string();

    Ok(password_hash)
}
