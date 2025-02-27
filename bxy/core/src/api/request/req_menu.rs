use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 菜单请求体结构
pub struct MenuReq {
    /// 物理主键
    pub id: String,
    /// 菜单代码：唯一标识、系统自动生成UUID 或 用户自定义规则
    pub guid: String,
    /// 上级菜单代码
    pub pguid: String,
    /// 排序
    pub ord: u32,
    /// 状态：默认：1正常，0停用、1正常、2删除、其他自定义
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 所属应用
    pub app: String,
    /// 菜单名称/功能名称
    pub mname: String,
    /// 菜单类型：C目录、M菜单、P页面、F功能
    pub mtype: String,
    /// 路由类型：0内链、1外链
    pub uio: u8,
    /// 路由地址
    pub path: String,
    /// 接口地址
    pub api: String,
    /// 请求类型
    pub method: String,
    /// 打开方式：0 Tab、1 Pagep、2 Dialog
    pub opt: u8,
    /// 显示名称
    pub alias: String,
    /// 列表类型：
    pub tbl: String,
    /// 查询头
    pub query: Option<String>,
    /// 查询头脚本
    pub qscript: Option<String>,
    /// 列表头
    pub cols: Option<String>,
    /// 列表头脚本
    pub cscript: Option<String>,
    /// 图标
    pub icon: String,
    /// 样式
    pub style: String,
    /// 显示区域
    pub show: u8,
    /// 组件路径
    pub comp: Option<String>,
    /// 是否隐藏
    pub visible: String,
    /// 缓存 keepAlive
    pub is_cache: String,
    /// 日志：0不记录、1文件记录、2数据库记录、3文件+数据库记录
    pub log_method: String,
    /// 缓存
    pub data_cache_method: String,
    /// 数据范围
    pub data_scope: String,
    /// 国际化
    pub i18n: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 菜单查询请求体结构
pub struct MenuSearchReq {
    /// 菜单代码
    #[serde(default)]
    pub guid: Option<Vec<String>>,
    /// 菜单代码
    #[serde(default)]
    pub pguid: Option<Vec<String>>,
    /// 菜单名称
    pub mname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 用户菜单请求结构
pub struct UserMenuReq {
    /// 应用代码
    pub app_code: String,
    /// 菜单 GUID
    pub mcode: String,
    /// 用户 ID
    pub u_id: String,
}
