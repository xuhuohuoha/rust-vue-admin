//! 流程引擎—事件订阅
use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_ru_event_subscr")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 数据版本
    pub rev_: u32,
    /// 事件类型
    pub event_type_: String,
    /// 事件名称
    pub event_name_: String,
    /// 执行ID
    pub exec_id_: String,
    /// 流程定义ID
    pub proc_def_id_: String,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 具体事件ID
    pub activity_id_: String,
    /// 事件的配置属性
    pub config_: String,
    /// 创建时间
    pub create_time_: DateTime,
    /// 租户ID
    pub tenant_id_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
