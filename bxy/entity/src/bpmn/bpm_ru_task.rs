use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bpm_ru_task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 说明
    pub description_: String,
    /// 任务定义ID
    pub task_def_id_: String,
    /// 任务拥有者
    pub owner_: String,
    /// 被指派执行该任务的人
    pub assignee_: String,
    /// 委托
    pub delegation_: String,
    /// 优先级
    pub priority_: u8,
    /// 创建时间
    pub create_time_: DateTime,
    /// 耗时
    pub duration_: i64,
    /// 类别
    pub category_: String,
    /// 是否挂起：1-激活、2-挂起
    pub suspension_state_: u8,
    /// 租户ID
    pub tenant_id_: String,
    /// 表单key
    pub form_key_: String,
    /// 签收时间
    pub claim_time_: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
