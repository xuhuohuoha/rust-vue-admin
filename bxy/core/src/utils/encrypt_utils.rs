//!
//! 密码工具
//!
use std::fmt::Write;

/// 密码加密
pub fn encrypt_password(password: &str, salt: &str) -> String {
    let s = password.to_owned() + salt;
    let digest = md5::compute(s).to_vec();
    let mut result = String::new();
    for a in digest.iter() {
        write!(result, "{:02x}", a).unwrap();
    }
    result
}
