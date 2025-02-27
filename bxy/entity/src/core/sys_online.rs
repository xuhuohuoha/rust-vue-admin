//! 在线用户实体
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bxy_online")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 用户 ID
    pub u_id: String,
    /// Token
    pub token_id: String,
    /// Token 过期时间
    pub token_exp: i64,
    /// 用户名称
    pub uname: String,
    /// 网络
    pub net: String,
    /// IP
    pub ip: String,
    /// 登录地点
    pub login_location: String,
    /// 设备
    pub device: String,
    /// 浏览器
    pub browser: String,
    /// 操作系统
    pub os: String,
    /// 登录时间
    pub login_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
