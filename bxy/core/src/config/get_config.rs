//! 加载配置
//!
//! # 描述
//! 加载配置文件，初始化配置
//!
//! - 配置文件路径：config/config.toml
//! - 加载使用方式：保存在内存中
//!

use once_cell::sync::Lazy;
use std::{fs::File, io::Read};

use super::configs::Configs;

/// 配置文件
const CONFIG_FILE: &str = "config/config.toml";

/// 加载配置文件中的配置项，并保存到内存中供使用
pub static CONFIG: Lazy<Configs> = Lazy::new(self::Configs::init);

impl Configs {
    /// 加载配置文件，完成配置项初始化
    pub fn init() -> Self {
        let mut file = match File::open(CONFIG_FILE) {
            Ok(f) => f,
            Err(e) => panic!("不存在配置文件：{}，错误信息：{}", CONFIG_FILE, e),
        };
        let mut config_contents = String::new();
        match file.read_to_string(&mut config_contents) {
            Ok(s) => s,
            Err(e) => panic!("读取配置文件失败，错误信息：{}", e),
        };
        toml::from_str(&config_contents).expect("解析配置文件错误.")
    }
}
