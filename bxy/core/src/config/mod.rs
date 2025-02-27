//! 配置模块
//!
//! 包括：
//! - 配置读取
//! - 配置存储
//!
pub mod configs;
pub mod get_config;

// 重新导出
pub use configs::*;
pub use get_config::CONFIG;
