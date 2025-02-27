//!
//! 环境变量
//!
use std::sync::Arc;

use once_cell::sync::Lazy;
use tracing::Level;
use tracing_subscriber::{
    fmt,
    fmt::format::{Compact, Format},
};

use crate::config::CONFIG;

pub static RT: Lazy<Arc<tokio::runtime::Runtime>> = Lazy::new(|| {
    let rt = tokio::runtime::Runtime::new().unwrap();
    Arc::new(rt)
});

/// 加载环境变量
pub fn setup() {
    //   打印logo
    self::show_log();
}

/// 打印日志
fn show_log() {
    let logo = r#"BLUNKA-BPM"#;
    println!("{}", logo);
    println!("系统架构：{}", std::env::var("OS").unwrap());
    println!("系统类型：{}", std::env::consts::ARCH);
    println!("操作系统：{}", std::env::consts::FAMILY);
    println!()
}

/// 获取日志级别
pub fn get_log_level() -> Level {
    match CONFIG.log.log_level.as_str() {
        "TRACE" => tracing::Level::TRACE,
        "DEBUG" => tracing::Level::DEBUG,
        "INFO" => tracing::Level::INFO,
        "WARN" => tracing::Level::WARN,
        "ERROR" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    }
}

#[cfg(target_os = "windows")]
use tracing_subscriber::fmt::time::SystemTime;

#[cfg(target_os = "windows")]
pub fn get_log_format() -> Format<Compact, SystemTime> {
    fmt::format()
        .with_level(true) // don't include levels in formatted output
        .with_target(true) // don't include targets
        .with_thread_ids(true)
        // include the thread ID of the current thread
        // .with_thread_names(true)
        // .with_file(true)
        // .with_ansi(true)
        // .with_line_number(true) // include the name of the current thread
        .with_timer(SystemTime) // use RFC 3339 timestamps
        .compact()
}

#[cfg(not(target_os = "windows"))]
pub fn get_log_format() -> Format<Compact> {
    fmt::format()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(true)
        .compact()
}
