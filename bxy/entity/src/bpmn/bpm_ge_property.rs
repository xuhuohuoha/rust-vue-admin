//! 流程引擎-属性信息
//!
//! # 描述
//!
//! - 包括：
//!     流程引擎版本
//!     数据库版本
//!     数据库历史操作
//!
use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_ge_property")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 属性名称
    pub name_: String,
    /// 属性值
    pub value_: String,
    /// 版本
    pub rev_: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
