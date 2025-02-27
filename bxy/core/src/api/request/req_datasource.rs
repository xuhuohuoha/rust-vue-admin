use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DatasourceReq {
    /// 物理主键
    pub id: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 数据源
    pub db: String,
    /// database_url
    pub url: String,
    /// max_connections
    pub maxc: u32,
    /// min_connections
    pub minc: u32,
    // connect_timeout
    pub conn_timeout: u64,
    // idle_timeout
    pub idle_timeout: u64,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DatasourceSearchReq {
    pub db: Option<String>,
}
