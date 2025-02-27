//! 数据库连接
//!
//! # 描述
//!
//!

use std::time::Duration;

use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use tokio::sync::OnceCell;

use crate::config::CONFIG;

///  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

/// 从连接池返回连接
pub async fn db_conn() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(CONFIG.database.url.to_owned());
    opt.max_connections(CONFIG.database.max_connections) // 最大连接数
        .min_connections(CONFIG.database.min_connections) // 最小连接数
        .connect_timeout(Duration::from_secs(CONFIG.database.connect_timeout)) // 连接超时时间
        .idle_timeout(Duration::from_secs(CONFIG.database.idle_timeout)) // 空闲时间时间
        .sqlx_logging(CONFIG.database.sqlx_logging); // 是否启用日志
    match Database::connect(opt).await {
        Ok(db) => {
            tracing::info!("=============== Database Connected. ==============="); // 数据库连接成功日志
            db
        }
        Err(e) => {
            tracing::error!("Database Connection Failed: {}", e);
            panic!("Database Connection Failed.");
        }
    }
}
