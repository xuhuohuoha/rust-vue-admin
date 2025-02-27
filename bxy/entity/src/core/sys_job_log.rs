//! 作业日志实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "bxy_job_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 任务 GUID
    pub job_id: String,
    pub lot_id: i64,
    pub lot_order: i64,
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub job_params: Option<String>,
    pub job_message: Option<String>,
    /// 状态
    pub status: String,
    pub exception_info: Option<String>,
    pub is_once: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
    pub elapsed_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
