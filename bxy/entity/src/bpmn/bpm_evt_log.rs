//! 流程引擎 — 事件日志
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bpm_evt_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 类别
    pub type_: String,
    /// 流程定义ID
    pub proc_def_id_: String,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 执行ID
    pub exec_id_: String,
    /// 任务ID
    pub task_id_: String,
    /// 日志时间
    pub time_: DateTime,
    /// 用户ID
    pub user_id_: String,
    /// 数据
    pub data_: Vec<u8>,
    /// 锁定人
    pub lock_owner_: String,
    /// 锁定时间
    pub lock_time_: DateTime,
    /// 是否流程
    pub is_processed_: u8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
