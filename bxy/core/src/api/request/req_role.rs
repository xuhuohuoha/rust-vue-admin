use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 新增、编辑角色请求体结构
pub struct RoleReq {
    /// 物理主键
    pub id: String,
    /// 角色代码
    pub guid: String,
    // 上级角色代码
    pub pguid: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 角色名称
    pub rname: String,
    /// 角色属性
    pub att: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 查询角色请求体结构
pub struct RoleSearchReq {
    /// 角色代码
    pub guid: Option<Vec<String>>,
    /// 上级代码
    pub pguid: Option<Vec<String>>,
    /// 角色名称
    pub rname: Option<String>,
}
