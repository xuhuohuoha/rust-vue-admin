//! 流程引擎—历史流程实例

use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_hi_procinst")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 业务主键
    pub biz_key: String,
    /// 流程定义ID
    pub proc_def_id_: String,
    /// 开始时间
    pub start_time: DateTime,
    /// 结束时间
    pub end_time: DateTime,
    /// 持续时间
    pub duration: i64,
    /// 起始人
    pub start_user_id: String,
    /// 起始环节
    pub start_act_id: String,
    /// 结束环节
    pub end_act_id: String,
    /// 父流程实例ID
    pub parent_proc_inst_id: String,
    /// 删除原因
    pub delete_reason: String,
    /// 租户ID
    pub tenant_id: String,
    /// 名称
    pub name_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
