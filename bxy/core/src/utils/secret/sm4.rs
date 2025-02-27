//!
//! SM4 加密解密算法
//!

use smcrypto::sm4;

/// ECB 加密
pub fn encrypt_ecb(encrypt_data: String, key: String) -> String {
    let sm4_ecb = sm4::CryptSM4ECB::new(key.as_bytes());
    let enc_ecb = sm4_ecb.encrypt_ecb(encrypt_data.as_bytes());
    String::from_utf8(enc_ecb).unwrap()
}

/// ECB 解密
pub fn decrypt_ecb(decrypt_data: String, key: String) -> String {
    let sm4_ecb = sm4::CryptSM4ECB::new(key.as_bytes());
    let dec_ecb = sm4_ecb.decrypt_ecb(decrypt_data.as_bytes());
    String::from_utf8(dec_ecb).unwrap()
}

/// CBC 加密
pub fn encrypt_cbc(encrypt_data: String, key: String, iv: String) -> String {
    let sm4_cbc = sm4::CryptSM4CBC::new(key.as_bytes(), iv.as_bytes());
    let enc_cbc = sm4_cbc.encrypt_cbc(encrypt_data.as_bytes());
    String::from_utf8(enc_cbc).unwrap()
}

/// CBC 解密
pub fn decrypt_cbc(decrypt_data: String, key: String, iv: String) -> String {
    let sm4_cbc = sm4::CryptSM4CBC::new(key.as_bytes(), iv.as_bytes());
    let dec_cbc = sm4_cbc.decrypt_cbc(decrypt_data.as_bytes());
    String::from_utf8(dec_cbc).unwrap()
}
