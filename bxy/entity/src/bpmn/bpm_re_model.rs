//! 流程引擎—流程设计模型
use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_re_model")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 版本
    pub rev_: u32,
    /// 名称
    pub name_: String,
    /// 流程定义ID
    pub key_: String,
    /// 类别
    pub category_: String,
    /// 创建时间
    pub create_time_: DateTime,
    /// 最近一次更新时间
    pub last_modify_time_: DateTime,
    /// 版本
    pub version_: String,
    /// 以json格式保存流程定义的信息
    pub meta_info_: String,
    /// 部署ID
    pub deployment_id_: String,
    /// 编辑器源文件
    pub editor_source_value_id_: String,
    /// 编辑器扩展文件
    pub editor_source_extra_value_id_: String,
    /// 租户ID
    pub tenant_id_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
