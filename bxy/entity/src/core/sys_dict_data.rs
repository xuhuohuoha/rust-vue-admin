//! 字典数据实体
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "bxy_dict_data")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 字典数据代码
    pub guid: String,
    /// 上级数据代码
    pub pguid: String,
    /// 创建人
    pub create_by: String,
    /// 最近一次修改人
    pub update_by: Option<String>,
    /// 逻辑删除人
    pub delete_by: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
    /// 修改时间
    pub updated_at: Option<DateTime>,
    /// 删除时间
    pub deleted_at: Option<DateTime>,
    /// 版本号
    pub version: u32,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 级别
    pub lvl: u8,
    /// 字典名称
    pub dname: String,
    /// 字典属性
    pub att: Option<String>,
    /// 扩展属性
    pub ext1: Option<String>,
    /// 扩展属性
    pub ext2: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
