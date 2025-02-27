//! 流程引擎-图形化
//!
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bpm_re_graph")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id_: String,
    /// 流程ID
    pub prcs_: String,
    /// 步骤ID
    pub step_: String,
    /// 步骤类型
    pub type_: u8,
    /// X坐标
    pub x_: i32,
    /// Y坐标
    pub y_: i32,
    /// 高度
    pub h_: u16,
    /// 宽度
    pub w_: u16,
    /// 样式
    pub clazz_: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
