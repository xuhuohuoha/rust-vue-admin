use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 新增、编辑岗位请求体结构
pub struct PostReq {
    /// 物理主键
    pub id: String,
    /// 岗位代码
    pub guid: String,
    /// 上级岗位代码
    pub pguid: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 岗位名称
    pub pname: String,
    // 岗位属性
    pub att: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 查询岗位请求体结构
pub struct PostSearchReq {
    /// 岗位代码
    pub guid: Option<Vec<String>>,
    /// 上级代码
    pub pguid: Option<Vec<String>>,
    /// 岗位名称
    pub pname: Option<String>,
}
