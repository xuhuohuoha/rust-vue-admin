//! 流程引擎—历史附件实体
use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_hi_attachment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 数据版本
    pub rev_: u32,
    /// 用户ID
    pub user_id_: String,
    /// 附件名称
    pub name_: String,
    /// 附件类型
    pub type_: String,
    /// 附件说明
    pub description_: String,
    /// 任务ID
    pub task_id_: String,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 附件URL
    pub url_: String,
    /// 字节表ID
    pub byte_id_: String,
    /// 时间
    pub time_: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
