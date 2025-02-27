//! 附件实体
//!

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bxy_adtion")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 业务数据 GUID
    pub guid: String,
    /// 创建人
    pub create_by: String,
    /// 逻辑删除人
    pub delete_by: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
    /// 删除时间
    pub deleted_at: Option<DateTime>,
    /// 所属模块
    pub mcode: String,
    /// 附件名称
    pub fname: String,
    /// 附件类型
    pub ext: String,
    /// 附件大小
    pub fsize: String,
    /// 附件路径
    pub url: String,
    /// 扩展属性
    pub ext1: Option<String>,
    /// 扩展属性
    pub ext2: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
