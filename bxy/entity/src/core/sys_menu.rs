//! 菜单实体
//!
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "bxy_menu")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    /// 物理主键
    pub id: String,
    /// 菜单代码：唯一标识
    /// 通过用户自定义规则由接口传入，接口不传入则由系统自动生成UUID
    pub guid: String,
    /// 上级菜单代码
    /// 当上级菜单代码为空时，默认与菜单代码相同，代表根菜单
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
    /// 菜单状态
    /// 默认：1
    /// 状态：0停用、1正常、2删除、其他自定义
    pub status: String,
    /// 备注
    pub remark: Option<String>,
    /// 所属应用代码
    pub app: String,
    /// 菜单功能名称
    /// 当配置为菜单时表示菜单名称，配置为功能时表示功能名称
    pub mname: String,
    /// 菜单类型
    /// 目录：C
    /// 功能：F + 代码，例如：F100
    pub mtype: String,
    /// 路由类型
    /// 0内链：应用内部
    /// 1外链：应用外部
    pub uio: u8,
    /// 路由地址
    pub path: String,
    /// 接口地址
    pub api: String,
    /// 请求类型，GET/PUT/POST/DELETE/HEAD/TRACE/CONNECT/OPTIONS/
    pub method: String,
    /// 打开方式：0 Tab、1 Page、2 Dialog
    pub opt: u8,
    /// 显示名称
    pub alias: String,
    /// 列表类型：D\V\DT\VT
    /// 预留，暂不使用
    pub tbl: String,
    /// 查询头组件
    pub query: Option<String>,
    /// 查询头脚本
    pub qscript: Option<String>,
    /// 列表头组件
    pub cols: Option<String>,
    /// 列表头脚本
    pub cscript: Option<String>,
    /// 图标
    /// 当配置为菜单时表示菜单图标，配置为功能时表示功能图标
    pub icon: String,
    /// 自定义样式
    /// 当配置为菜单时表示菜单样式，配置为功能时表示功能样式
    pub style: String,
    /// 显示区域
    /// 当配置为功能时表示功能按钮显示区域
    /// 0：列表工具条，支持批量操作
    /// 1：列表操作列，对当前记录进行操作
    pub show: u8,
    /// 组件路径
    pub comp: Option<String>,
    /// 是否隐藏
    /// 0 否
    /// 1 是
    pub visible: String,
    /// 缓存 keepAlive
    pub is_cache: String,
    /// 日志：0不记录、1文件记录、2数据库记录、3文件+数据库记录
    pub log_method: String,
    /// 数据缓存方法：
    pub data_cache_method: String,
    /// 数据范围
    pub data_scope: String,
    /// 国际化
    pub i18n: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
