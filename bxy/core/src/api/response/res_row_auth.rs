use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RowAuthRes {
    /// 菜单
    pub mcode: String,
    /// 标题
    pub title: String,
    /// 条件
    pub content: String,
}
