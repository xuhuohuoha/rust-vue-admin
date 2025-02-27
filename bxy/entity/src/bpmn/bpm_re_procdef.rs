//! 流程引擎—已部署的流程定义
use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_re_procdef")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 数据版本
    pub rev_: u32,
    /// 流程定义分类，XML文件的targetNamespace
    pub category_: String,
    /// 流程定义的名称，XML文件中process元素的name属性
    pub name_: String,
    /// 流程定义的key，XML文件中process元素的id属性
    pub key_: String,
    /// 版本
    pub version_: String,
    /// 部署ID，对应bpm_re_deployment表的ID
    pub deployment_id_: String,
    /// bpmn文件名称，流程文件的相对路径
    pub resource_name_: String,
    /// 流程定义对应的流程图资源名称
    pub dgrm_resource_name_: String,
    /// 说明
    pub description_: String,
    /// 是否存在开始环节formkey，0-否，1-是
    pub has_start_form_key_: u8,
    /// 是否存在流程图形表示，0-否，1-是
    pub has_graphical_notations_: u8,
    /// 流程定义状态，1-激活，2-挂起
    pub suspension_state_: u8,
    /// 租户ID
    pub tenant_id_: String,
    /// 引擎版本
    pub engine_version_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
