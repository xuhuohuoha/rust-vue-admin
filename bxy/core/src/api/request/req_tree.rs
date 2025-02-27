use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TreeReq {
    /// 物理主键
    pub id: String,
    /// 树代码字段
    pub guid: String,
    /// 树上级代码字段
    pub pguid: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 树来源
    pub tv: String,
    /// 显示字段
    pub sfield: String,
    /// 条件
    pub twhere: String,
    /// 排序
    pub tord: String,
    /// 树名称
    pub tname: String,
    /// 关联模块
    pub mcode: String,
    /// 树关联字段
    pub tfields: String,
    /// 菜单关联字段
    pub mfields: String,
    /// 授权类型
    pub atype: u8,
    /// 授权方式
    pub amethod: u8,
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
pub struct TreeSearchReq {
    /// 树名称
    pub tname: Option<String>,
    /// 所属模块
    pub mcode: Option<String>,
}
