use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DqlReq {
    /// 物理主键
    pub id: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 所属模块
    pub mcode: String,
    /// Sql名称
    pub sign: String,
    /// 动态Sql
    pub dql: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DqlSearchReq {
    pub mcode: Option<String>,
    pub sign: Option<String>,
}
