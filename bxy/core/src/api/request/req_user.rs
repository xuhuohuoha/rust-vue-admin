use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{req_duty::DutyReq, req_org::OrgReq, req_post::PostReq, req_role::RoleReq};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({
    "id": "",
    "u_id": "",
    "ord": 0,
    "status": "1",
    "remark": "",
    "ucode": "admin",
    "uname": "系统管理员",
    "upwd": "123456",
    "sex": "",
    "email": "",
    "qq": "",
    "webchat": "",
    "phone": "",
    "pin": "",
    "pass": "",
    "avatar": "",
    "ext1": "",
    "ext2": "",
    "ext3": ""}
))]
/// 新建、编辑用户请求体结构
pub struct UserReq {
    /// 物理主键
    pub id: String,
    /// 用户ID
    pub u_id: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 用户账号
    pub ucode: String,
    /// 用户名称
    pub uname: String,
    /// 用户密码
    pub upwd: String,
    /// 性别
    pub sex: String,
    /// 邮箱
    pub email: Option<String>,
    /// QQ/TIM
    pub qq: Option<String>,
    /// 微信
    pub webchat: Option<String>,
    /// 电话
    pub phone: Option<String>,
    /// 授权密码
    pub pin: Option<String>,
    /// 审批密码
    pub pass: Option<String>,
    /// 头像
    pub avatar: String,
    /// 扩展属性
    pub ext1: Option<String>,
    /// 扩展属性
    pub ext2: Option<String>,
    /// 扩展属性
    pub ext3: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 修改密码请求体结构
pub struct UpdatePasswordReq {
    /// 用户ID
    pub u_id: String,
    /// 旧密码
    pub old_password: String,
    /// 新密码
    pub new_password: String,
}

/// 重置密码请求体结构
pub struct ResetPasswordReq {
    /// 用户ID
    pub u_id: String,
    /// 新密码
    pub new_password: String,
}

/// 设置用户状态请求体结构
pub struct UserStatusReq {
    /// 用户ID
    pub u_id: String,
    /// 用户状态
    pub status: String,
}

/// 用户详情
///
/// 包括：
///     1、用户部门信息
///     2、用户角色信息
///     3、用户岗位信息
///     4、用户职务信息
pub struct UserAllReq {
    /// 用户基本信息
    pub user: UserReq,
    /// 用户部门信息
    pub org: OrgReq,
    /// 用户角色信息
    pub role: RoleReq,
    /// 用户岗位信息
    pub post: PostReq,
    /// 用户职务信息
    pub duty: DutyReq,
}

/// 查询用户请求体结构
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserSearchReq {
    /// 用户 ID
    pub u_id: Option<String>,
    /// 状态
    pub status: Option<String>,
    /// 用户账号
    pub ucode: Option<String>,
    /// 用户名称
    pub uname: Option<String>,
    /// 排序字段
    pub order_by_column: Option<String>,
    /// 排序方式
    pub is_asc: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 查询用户权限请求体结构
pub struct UserAuthSearchReq {
    /// 菜单
    pub mcode: Option<String>,
    /// 用户ID
    pub u_id: Option<String>,
    /// 部门ID
    pub o_id: Option<String>,
    /// 角色ID
    pub r_id: Option<String>,
    /// 岗位ID
    pub p_id: Option<String>,
    /// 职务ID
    pub d_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
/// 新增、编辑用户权限请求体结构
pub struct UserAuthReq {
    /// 物理主键
    pub id: String,
    /// 业务主键
    pub guid: String,
    /// 排序
    pub ord: u32,
    /// 备注
    pub remark: Option<String>,
    /// 所属模块集合
    pub mcodes: Vec<String>,
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
pub struct UserApiReq {
    pub u_id: String,
    /// 菜单GUID
    pub mcode: String,
    /// Api
    pub api: String,
    /// 请求方式
    pub method: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserApiSearchReq {
    /// 用户账号
    pub u_id: Option<String>,
    /// 菜单GUID
    pub mcode: Option<String>,
    /// Api
    pub api: Option<String>,
    /// 请求方式
    pub method: Option<String>,
    /// 创建开始时间
    pub begin_time: Option<String>,
    /// 创建结束时间
    pub end_time: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct UserOrgReq {
    /// 用户ID
    pub u_id: String,
    /// 所属部门ID
    pub o_id: String,
    /// 用户类型
    pub u_type: u8,
}
