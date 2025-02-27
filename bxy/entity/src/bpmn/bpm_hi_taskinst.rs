//! 流程引擎—历史任务
use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_hi_taskinst")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 流程定义ID
    pub proc_def_id_: String,
    /// 任务定义ID
    pub task_def_id_: String,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 执行ID
    pub exec_id_: String,
    /// 父任务ID
    pub parent_task_id_: String,
    /// 名称
    pub name_: String,
    /// 说明
    pub description_: String,
    /// 实际签收人（任务拥有者）
    /// 默认为空，委托时有值
    pub owner_: String,
    /// 被指派执行该任务的人
    pub assignee_: String,
    /// 开始时间
    pub start_time_: DateTime,
    /// 提醒时间
    pub claim_time_: DateTime,
    /// 结束时间
    pub end_time_: DateTime,
    /// 持续时间
    pub duration_: i64,
    /// 删除原因
    pub delete_reason_: String,
    /// 优先级别
    pub priority_: i32,
    /// 过期时间
    pub due_time_: DateTime,
    /// 表单key
    pub form_key_: String,
    /// 类别
    pub category_: String,
    /// 租户ID
    pub tenant_id_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
