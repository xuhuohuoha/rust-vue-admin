//! 流程环节办理人定义
//!
use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_re_trans")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id_: String,
    /// 步骤ID
    pub step_: String,
    /// 办理人类型：0静态、1动态、2全部
    pub category_: u8,
    /// 授权类型
    pub type_: u8,
    /// 授权方法
    pub method_: u8,
    /// 用户账号
    pub user_: String,
    /// 部门
    pub org_: String,
    /// 角色
    pub role_: String,
    /// 岗位
    pub post_: String,
    /// 职务
    pub duty_: String,
    /// 领导者：0否、1是
    pub leader_: u8,
    /// 动态用户
    pub dyn_user_: String,
    /// 动态部门
    pub dyn_org_: String,
    /// 动态角色
    pub dyn_role_: String,
    /// 动态岗位
    pub dyn_post_: String,
    /// 动态职务
    pub dyn_duty_: String,
    /// 动态领导者：0否、1是
    pub dyn_leader_: u8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
