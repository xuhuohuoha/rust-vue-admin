use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub struct UserMenu {
    /// 菜单代码
    pub guid: String,
    /// 上级菜单
    pub pguid: String,
    pub ord: u32,
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
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct MenuTreeRes {
    #[serde(flatten)]
    pub menu: UserMenu,
    pub valid: bool,
    pub children: Option<Vec<MenuTreeRes>>,
}
