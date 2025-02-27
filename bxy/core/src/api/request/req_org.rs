use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 新增、编辑部门请求体结构
pub struct OrgReq {
    /// 物理主键
    pub id: String,
    /// 部门代码
    pub guid: String,
    /// 上级部门代码
    pub pguid: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 部门名称
    pub oname: String,
    /// 部门负责人
    pub leader: Option<String>,
    /// 电子邮箱
    pub email: Option<String>,
    /// 电话
    pub phone: Option<String>,
    /// 属性
    pub att: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 查询部门请求体结构
pub struct OrgSearchReq {
    /// 部门代码
    pub guid: Option<Vec<String>>,
    /// 上级代码
    pub pguid: Option<Vec<String>>,
    /// 部门名称
    pub oname: Option<String>,
}
