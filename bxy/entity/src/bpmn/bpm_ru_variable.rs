use sea_orm::entity::prelude::*;
use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema)]
#[sea_orm(table_name = "bpm_ru_variable")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 主键
    pub id_: String,
    /// 数据版本
    pub rev_: u32,
    /// 流程实例ID
    pub proc_inst_id_: String,
    /// 参数执行ID
    pub exec_id_: String,
    /// 任务ID
    pub task_id_: String,
    /// 参数名称
    pub name_: String,
    /// 参数类型
    pub var_type_: String,
    /// 字节表ID
    pub byte_id_: String,
    /// 存储变量类型为Double
    pub double_: f64,
    /// 存储变量类型为Long
    pub long_: i64,
    /// 存储变量类型为String
    pub text_: String,
    /// 存储变量类型为持久化对象，值为对象ID
    pub text2_: String,
    /// 创建时间
    pub create_time_: DateTime,
    /// 最近一次修改时间
    pub last_modify_time_: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
