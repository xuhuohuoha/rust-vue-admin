//!
//! AES 加密解密算法
//!

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

/// Encrypts the given plaintext using AES-256-GCM encryption algorithm.
///
/// # Parameters
///
/// - `key_str`: A `String` representing the encryption key. It must be 32 characters long to be valid for AES-256.
/// - `plaintext`: A `String` containing the data to be encrypted.
///
/// # Returns
///
/// A `String` containing the hexadecimal representation of the encrypted data, which includes both the nonce and the ciphered data.
pub fn encrypt(key_str: String, plaintext: String) -> String {
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let ciphered_data = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .expect("failed to encrypt");
    // combining nonce and encrypted data together
    // for storage purpose
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);
    hex::encode(encrypted_data)
}

/// 解密
pub fn decrypt(key_str: String, encrypted_data: String) -> String {
    let encrypted_data = hex::decode(encrypted_data).expect("failed to decode hex string into vec");
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher
        .decrypt(nonce, ciphered_data)
        .expect("failed to decrypt data");
    String::from_utf8(plaintext).expect("failed to convert vector of bytes to string")
}

// fn main() {
//     let plaintext = "backendengineer.io".to_string();
//     let key_str = "thiskeystrmustbe32charlongtowork".to_string();
//     let encrypted_data = encrypt(key_str.clone(), plaintext);
//     println!("encrypted_data: {:?}", encrypted_data.clone());
//     let original = decrypt(key_str, encrypted_data);
//     println!("original: {:?}", original);
// }
