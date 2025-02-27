//! 关联树实体
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, ToSchema,
)]
#[sea_orm(table_name = "bxy_tree")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 树代码字段
    pub guid: String,
    /// 树上级代码字段
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
    /// 树来源
    pub tv: String,
    /// 显示字段
    pub sfield: String,
    /// 条件
    pub twhere: String,
    /// 排序
    pub tord: String,
    /// 树名称
    pub tname: String,
    /// 关联模块
    pub mcode: String,
    /// 树关联字段
    pub tfields: String,
    /// 菜单关联字段
    pub mfields: String,
    /// 授权类型
    pub atype: u8,
    /// 授权方式
    pub amethod: u8,
    /// 用户账号
    pub u_id: String,
    /// 角色
    pub r_id: String,
    /// 部门
    pub o_id: String,
    /// 岗位
    pub p_id: String,
    /// 职务
    pub d_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
