use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 列级授权新增、编辑请求体结构
pub struct ColAuthReq {
    /// 物理主键
    pub id: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 列名
    pub field: String,
    /// 正则
    pub regx: String,
    /// 所属模块
    pub mcode: String,
    /// 授权类型
    pub atype: u8,
    /// 授权方式
    pub amethod: u16,
    /// 用户账号
    pub u_id: String,
    /// 角色
    pub r_id: String,
    /// 部门
    pub o_id: String,
    /// 岗位
    pub p_id: String,
    /// 职务
    pub d_id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 列级授权查询请求体结构
pub struct ColAuthSearchReq {
    pub mcode: Option<String>,
    pub field: Option<String>,
}
