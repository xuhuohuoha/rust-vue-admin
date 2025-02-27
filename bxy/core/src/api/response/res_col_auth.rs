use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ColAuthRes {
    /// 菜单
    pub mcode: String,
    /// 列名
    pub field: String,
    /// 正则
    pub regx: String,
}
