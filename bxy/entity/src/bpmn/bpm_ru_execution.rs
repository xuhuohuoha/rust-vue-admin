//! 流程引擎—流程实例
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bpm_ru_execution")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 版本
    pub rev_: u32,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 业务主键ID
    pub business_key_: String,
    /// 父执行流的ID
    pub parent_id_: String,
    /// 流程定义ID
    pub proc_def_id_: String,
    pub super_exec_: String,
    pub root_proc_inst_id_: String,
    /// 环节实例ID
    pub act_id_: String,
    /// 是否存活
    pub is_active_: u8,
    /// 执行流是否正在并行
    pub is_concurrent_: u8,
    pub is_scope_: u8,
    pub is_event_scope_: u8,
    pub is_mi_root_: u8,
    /// 流程终端状态
    pub suspension_state_: u32,
    pub cached_ent_state_: u32,
    /// 租户ID
    pub tenant_id_: String,
    /// 名称
    pub name_: String,
    /// 开始时间
    pub start_time_: DateTime,
    /// 起始用户ID
    pub start_user_id_: String,
    /// 锁定时间
    pub lock_time_: DateTime,
    pub is_count_enabled_: u8,
    pub evt_subscr_count: u32,
    pub task_count_: u32,
    pub job_count_: u32,
    pub timer_job_count_: u32,
    pub susp_job_count_: u32,
    pub deadletter_job_count_: u32,
    pub var_count_: u32,
    pub id_link_count_: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
