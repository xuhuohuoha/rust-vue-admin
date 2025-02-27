//!
//! SM3 加密算法
//!

use smcrypto::sm3;

/// 加密
pub fn encrpty(str: String) -> String {
    sm3::sm3_hash(str.as_bytes())
}
