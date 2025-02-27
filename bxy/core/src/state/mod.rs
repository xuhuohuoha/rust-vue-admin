//! Application State
//!
//! # 描述
//!
//!

use arc_swap::ArcSwap;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::config::Configs;

/// 全局共享状态结构体
pub struct AppState {
    /// 数据库连接
    pub db_conn: ArcSwap<DatabaseConnection>,
    /// 配置
    pub configs: ArcSwap<Configs>,
}

impl AppState {
    /// 创建新的 AppState 实例
    pub fn new(configs: &Configs, db_conn: DatabaseConnection) -> anyhow::Result<Self> {
        Ok(Self {
            db_conn: ArcSwap::new(Arc::new(db_conn)),
            configs: ArcSwap::new(Arc::new((*configs).clone())),
        })
    }
}
