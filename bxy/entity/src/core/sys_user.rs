//! 用户实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bxy_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 业务主键
    pub u_id: String,
    /// 创建人
    pub create_by: String,
    /// 最近一次修改人
    pub update_by: Option<String>,
    /// 逻辑删除人
    pub delete_by: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
    /// 修改时间
    pub updated_at: Option<DateTime>,
    /// 删除时间
    pub deleted_at: Option<DateTime>,
    /// 版本号
    pub version: u32,
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
    /// 盐
    pub salt: String,
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
    pub avatar: Option<String>,
    /// 最近一次登录IP
    pub last_login_ip: Option<String>,
    /// 最近一次登录时间
    pub last_login_time: Option<DateTime>,
    /// 扩展属性
    pub ext1: Option<String>,
    /// 扩展属性
    pub ext2: Option<String>,
    /// 扩展属性
    pub ext3: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
