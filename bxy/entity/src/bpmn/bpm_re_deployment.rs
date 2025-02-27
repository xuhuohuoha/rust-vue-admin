//! 流程引擎—流程部署
use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_re_deployment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 部署名称
    pub name_: String,
    /// 类别，流程定义的Namespace
    pub category_: String,
    /// 流程定义ID
    pub key_: String,
    /// 租户ID
    pub tenant_id_: String,
    /// 部署时间
    pub deploy_time_: i64,
    /// 引擎版本
    pub engine_version_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
