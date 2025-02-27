use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TreeRes {
    /// 树代码字段
    pub guid: String,
    /// 树上级代码字段
    pub pguid: String,
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
}
