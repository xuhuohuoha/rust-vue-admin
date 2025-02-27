//! 流程引擎—无法执行工作
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bpm_ru_deadletter_job")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 版本
    pub rev_: u32,
    /// 类型
    pub type_: String,
    /// 排他
    pub exclusive_: u8,
    /// 执行实例ID
    pub exec_id_: String,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 流程定义ID
    pub proc_def_id_: String,
    /// 重试
    pub retries_: u32,
    /// 异常信息ID
    pub exception_stack_id_: String,
    /// 异常信息
    pub exception_msg_: String,
    /// 到期时间
    pub duedate_: DateTime,
    /// 重复
    pub repeat_: String,
    /// 处理类型
    pub handler_type_: String,
    /// 处理配置
    pub handler_cfg_: String,
    /// 租户ID
    pub tenant_id_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
