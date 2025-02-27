//! 附件类别实体
//!
//! # 描述
//!
//!

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bxy_adtion_ex")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 附件类别代码
    pub guid: String,
    /// 上级类别代码
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
    /// 状态，默认：1，0停用、1正常、2删除
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 所属模块
    pub mcode: String,
    /// 分类名称
    pub exname: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
