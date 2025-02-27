use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
/// 新增、编辑职务请求体结构
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DutyReq {
    /// 物理主键
    pub id: String,
    /// 职务代码
    pub guid: String,
    /// 上级代码
    pub pguid: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 职务名称
    pub dname: String,
    /// 职务属性
    pub att: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 查询职务请求体结构
pub struct DutySearchReq {
    /// 职务代码
    pub guid: Option<Vec<String>>,
    /// 上级代码
    pub pguid: Option<Vec<String>>,
    /// 职务名称
    pub dname: Option<String>,
}
