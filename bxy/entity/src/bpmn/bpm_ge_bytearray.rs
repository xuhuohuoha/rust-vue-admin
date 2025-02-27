//! 流程引擎-资源信息
//!
//! # 描述
//! 通用的流程定义和流程资源实体

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bpm_ge_bytearray")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键 nvarchar2(64)
    pub id_: String,
    /// 数据版本 integer
    pub rev_: u32,
    /// 资源名称 nvarchar2(255)
    pub name_: String,
    /// 部署序号 nvarchar2(64)
    /// 一次部署可以包含多个资源，该字段与部署表的主键关联
    pub deployment_id_: String,
    /// 资源内容 blob
    pub bytes_: Vec<u8>,
    /// 是否由引擎自动产生：0-否，1-是
    pub generated_: u8,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
