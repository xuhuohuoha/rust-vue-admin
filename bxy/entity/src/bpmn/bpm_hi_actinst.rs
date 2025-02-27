//! 流程引擎——历史行为

use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_hi_actinst")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 流程定义ID
    pub proc_def_id_: String,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 执行实例ID
    pub exec_id_: String,
    /// 环节实例ID
    pub act_id_: String,
    /// 任务ID
    pub task_id_: String,
    /// 调用外部的流程实例ID
    pub call_proc_inst_id_: String,
    /// 环节名称
    pub act_name_: String,
    /// 环节类型
    pub act_type_: String,
    /// 环节签收人
    pub assignee_: String,
    /// 开始时间
    pub start_time_: DateTime,
    /// 结束时间
    pub end_time_: DateTime,
    /// 持续时间
    pub duration_: i64,
    /// 删除原因
    pub delete_reason_: String,
    /// 租户ID
    pub tenant_id_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
