//! 流程引擎-流程定义
//!
//! # 描述
//!
//!
use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_re_prcs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id_: String,
    /// 流程ID（用于数据迁移）
    pub prcs_: String,
    /// 版本号
    pub rev_: u32,
    /// 流程状态：0 停用、1 正常
    pub status_: u8,
    /// 备注说明
    pub description_: Option<String>,
    /// 流程编码
    pub key_: String,
    /// 流程名称
    pub name_: String,
    /// 流程类型：10普通流程、20公有子流程、21私有子流程
    pub type_: u8,
    /// 父流程ID
    pub sup_: String,
    /// 流程标题
    pub title_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
