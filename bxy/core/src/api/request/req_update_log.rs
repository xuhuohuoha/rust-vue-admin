//!
//! 更新日志请求体结构定义
//!

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 新增、编辑请求体结构
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateLogReq {
    /// 物理主键
    pub id: String,
    /// 前端版本
    pub app_version: String,
    /// 后端版本
    pub backend_version: String,
    /// 更新概要
    pub title: String,
    /// 更新内容
    pub content: String,
}

/// 查询请求体结构
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateLogSearchReq {
    pub title: Option<String>,
    pub content: Option<String>,
}
