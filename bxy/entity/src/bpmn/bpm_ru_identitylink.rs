use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_ru_identitylink")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 版本
    pub rev_: u32,
    /// 关系数据类型：assignee-参与者，candidate-候选者，owner-拥有者
    pub type_: String,
    /// 用户ID
    pub user_id_: String,
    /// 任务ID
    pub task_id_: String,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 流程定义ID
    pub proc_def_id_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
