use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_hi_comment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 类型：event、comment
    pub type_: String,
    /// 时间
    pub time_: DateTime,
    /// 用户ID
    pub user_id_: String,
    /// 任务ID
    pub task_id_: String,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 行为类型
    pub action_: String,
    /// 信息
    /// 用于存放流程产生的信息，例如审批意见
    pub message_: Option<String>,
    /// 全部内容
    pub full_msg_: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
