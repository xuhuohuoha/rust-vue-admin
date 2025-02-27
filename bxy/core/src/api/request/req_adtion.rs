use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 附件新增、编辑请求体结构
pub struct AdtionReq {
    /// 物理主键
    pub id: String,
    /// 业务数据 GUID
    pub guid: String,
    /// 所属模块
    pub mcode: String,
    /// 附件名称
    pub fname: String,
    /// 附件类型
    pub ext: String,
    /// 附件大小
    pub fsize: String,
    /// 附件路径
    pub url: String,
    /// 扩展属性
    pub ext1: Option<String>,
    /// 扩展属性
    pub ext2: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 附件类型新增、编辑请求体结构
pub struct AdtionExReq {
    /// 物理主键
    pub id: String,
    /// 业务主键
    pub guid: String,
    /// 上级主键
    pub pguid: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 所属模块
    pub mcode: String,
    /// 分类名称
    pub exname: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 附件类型查询请求体结构
pub struct AdtionSearchReq {
    /// 业务ID
    pub guid: Option<String>,
    /// 所属菜单
    pub mcode: Option<String>,
    /// 附件名称
    pub fname: Option<String>,
    /// 附件类别
    pub ext: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 附件类型查询请求体结构
pub struct AdtionExSearchReq {
    /// 菜单
    pub mcode: Option<String>,
    /// 分类名称
    pub exname: Option<String>,
}
