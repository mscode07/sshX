use core::panic;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use base64::{Engine, engine::general_purpose};
use rand_core::OsRng;
use rand_core :: RngCore;

pub fn  encrypt_private_key(
    private_key: &str,
    password: &str,
)-> String{
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
    .expect("Failed to hash Password");

    let hash = password_hash.hash.expect("No hash generated");

    let key_bytes = hash.as_bytes();

    let cipher = Aes256Gcm::new_from_slice(key_bytes).expect("Failed to create cipher");

    let mut nonce_bytes = [0u8;12];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, private_key.as_bytes(),).expect("Encryption Failed");

    format!(
    "{}:{}:{}",
    salt.as_str(),
    general_purpose::STANDARD.encode(nonce_bytes),
    general_purpose::STANDARD.encode(ciphertext)
)
}

pub fn decrypt_private_key(
    encrypted_data:&str,
    password: &str
)-> String{
    let parts: Vec<&str> = encrypted_data.split(":").collect();

      if parts.len() != 3 {
        panic!("Invalid encrypted format");
    }

    let salt = parts[0];
    let nonce_b64 = parts[1];
    let ciphertext_b64 = parts[2];

    let nonce_bytes = general_purpose::STANDARD.decode(nonce_b64).expect("Failed to decode nonce");

    let ciphertext = general_purpose::STANDARD.decode(ciphertext_b64).expect("Faild to decode ciphertext");

    let argon2 = Argon2::default();

    let salt_string =
    SaltString::from_b64(salt)
        .expect("Invalid salt");

let password_hash = argon2
    .hash_password(
        password.as_bytes(),
        &salt_string,
    )
    .expect("Failed to hash password");

    let hash = password_hash.hash.expect("No Hash");

    let key_bytes = hash.as_bytes();

    let cipher = Aes256Gcm::new_from_slice(key_bytes).expect("Failed to create cipher");

    let nonce = Nonce::from_slice(&nonce_bytes);

    let decrypted = cipher.decrypt(nonce, ciphertext.as_ref(),).expect("Decryption failed");

    String::from_utf8(decrypted).expect("Invalid UTF8")


}