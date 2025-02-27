use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct MdReq {
    /// 物理主键
    pub id: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 主表模块
    pub mcode: String,
    /// 从表模块
    pub dcode: String,
    /// 主表关联字段
    pub m_fields: String,
    /// 从表关联字段
    pub d_fields: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct MdSearchReq {
    /// 主表模块
    pub mcode: Option<String>,
    /// 从表模块
    pub dcode: Option<String>,
}
