//! 流程引擎 - 步骤动作
//!
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bpm_re_act")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id_: String,
    /// 动作ID
    pub act_: String,
    /// 动作排序
    pub ord_: u8,
    /// 动作状态：0 停用、1 正常
    pub status_: u8,
    /// 版本号
    pub rev_: u32,
    /// 动作名称
    pub name_: String,
    /// 步骤ID
    pub step_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
