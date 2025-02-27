//! 应用授权实体
//!
//! # 描述
//! - 第一步：调用应用授权接口进行验证，验证通过则返回一个临时授权码
//! - 第二步：调用身份认证接口，用临时授权码换取Token
//! - 第三步：换取Token成功，删除临时授权码
//!

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "bxy_app_auth")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 应用代码
    pub app_code: String,
    /// 临时授权码
    pub temp_code: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
