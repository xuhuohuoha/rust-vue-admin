use std::path::Path;
use once_cell::sync::Lazy;

use crate::config::CONFIG;

pub static CERT_KEY: Lazy<CertKey> = Lazy::new(get_cert_key);

pub struct CertKey {
    pub cert: Vec<u8>,
    pub key: Vec<u8>,
}

impl CertKey {
    pub fn new(cert: Vec<u8>, key: Vec<u8>) -> Self {
        Self { cert, key }
    }
}
fn get_cert_key() -> CertKey {
    let cert = get_string(&CONFIG.cert.cert);
    let key = get_string(&CONFIG.cert.key);
    CertKey::new(cert, key)
}

fn get_string<P: AsRef<Path>>(path: P) -> Vec<u8> {
    std::fs::read(path).expect("读取文件失败")
}
