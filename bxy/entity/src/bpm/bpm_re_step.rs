//! 流程环节定义
//!
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bpm_re_step")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id_: String,
    /// 步骤ID（用于数据迁移）
    pub step_: String,
    /// 版本号
    pub rev_: u32,
    /// 步骤状态：0 停用、1 正常
    pub status_: String,
    /// 备注说明
    pub description_: Option<String>,
    /// 所属流程
    pub prcs_: String,
    /// 步骤名称
    pub name_: String,
    /// 步骤类型：
    pub type_: u8,
    /// 业务表单
    pub form_: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
