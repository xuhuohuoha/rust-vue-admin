//!
//! SM2 加密解密算法
//!

use smcrypto::sm2;

/// 生成 key
pub fn key() -> (String, String) {
    sm2::gen_keypair()
}

/// 加密
pub fn encrypt(encrypt_data: String, pk: String) -> String {
    let enc_ctx = sm2::Encrypt::new(&pk);
    let enc = enc_ctx.encrypt(encrypt_data.as_bytes());
    String::from_utf8(enc).unwrap()
}

/// 解密
pub fn decrypt(decrypt_data: String, sk: String) -> String {
    let dec_ctx = sm2::Decrypt::new(&sk);
    let dec = dec_ctx.decrypt(decrypt_data.as_bytes());
    String::from_utf8(dec).unwrap()
}
