//! 登录日志实体
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "bxy_login_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 用户账号
    pub u_id: String,
    /// 网络
    pub net: String,
    /// ip地址
    pub ip: String,
    /// 登录地点
    pub login_location: String,
    /// 浏览器
    pub browser: String,
    /// 操作系统
    pub os: String,
    /// 设备
    pub device: String,
    /// 登录状态
    pub status: String,
    /// 消息
    pub msg: String,
    /// 登录时间
    pub login_time: DateTime,
    /// 模块
    pub module: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
