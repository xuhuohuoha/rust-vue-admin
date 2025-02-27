//! 流程引擎—历史流程人员表

use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_hi_identitylink")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 类型
    pub type_: String,
    /// 用户ID
    pub user_id: Option<String>,
    /// 部门ID
    pub dept_id: Option<String>,
    /// 角色ID
    pub role_id: Option<String>,
    /// 岗位ID
    pub post_id: Option<String>,
    /// 职务ID
    pub duty_id: Option<String>,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 任务ID
    pub task_id_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
