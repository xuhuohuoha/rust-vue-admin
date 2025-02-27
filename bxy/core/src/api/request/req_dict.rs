use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DictReq {
    /// 物理主键
    pub id: String,
    /// 字典代码
    pub guid: String,
    /// 上级代码
    pub pguid: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: String,
    /// 字典名称
    pub dname: String,
    /// 字典属性
    pub att: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DictDataReq {
    /// 物理主键
    pub id: String,
    /// 字典数据代码
    pub guid: String,
    /// 上级数据代码
    pub pguid: String,
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DictSearchReq {
    pub guid: Option<String>,
    pub dname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DictDataSearchReq {
    pub guid: Option<String>,
    #[serde(default)]
    pub dname: Option<Vec<String>>,
    pub att: Option<String>,
}
