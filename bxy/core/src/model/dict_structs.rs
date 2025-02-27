//! 通用数据字典
//!
//!

use std::fmt;

/// 数据默认状态
///
/// # 描述
/// - 定义数据默认状态
/// - 支持业务自定义扩展
///
/// # 状态值
/// - Disabled：禁用 0
/// - Enabled：正常 1
/// - Delete：软删除 2
/// - Status：自定义状态
///
pub enum DataStatus {
    /// 禁用 0
    Disabled,
    /// 正常 1
    Enabled,
    /// 软删除 2
    Delete,
    /// 自定义状态
    Status(String),
}
impl fmt::Display for DataStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DataStatus::Disabled => "0",
            DataStatus::Enabled => "1",
            DataStatus::Delete => "2",
            DataStatus::Status(s) => s,
        };
        write!(f, "{}", s)
    }
}

/// 授权类型
///
/// # 描述
/// - 定义授权类型
///
/// # 类型值
/// - In：包含 0
/// - Ex：排除 1
pub enum AuthType {
    /// 包含
    In = 0,
    /// 排除
    Ex = 1,
}
impl AuthType {
    pub fn to_int(&self) -> i32 {
        match self {
            AuthType::In => 0,
            AuthType::Ex => 1,
        }
    }
}

/// 授权方式
///
/// # 描述
/// - 定义授权方式
///
/// # 方式值
/// - All：所有人 0
///
pub enum AuthMethod {
    /// 所有人
    All = 0,
    /// 指定用户
    User = 1,
    /// 指定角色
    Role = 2,
    /// 指定部门
    Org = 3,
    /// 指定岗位
    Post = 4,
    /// 指定职务
    Duty = 5,
    /// 指定角色、岗位
    RolePost = 24,
    /// 指定角色、职务
    RoleDuty = 25,
    /// 指定部门、角色
    OrgRole = 32,
    /// 指定部门、岗位
    OrgPost = 34,
    /// 指定部门、职务
    OrgDuty = 35,
    /// 指定岗位、职务
    PostDuty = 45,
    /// 指定部门、角色、岗位
    OrgRolePost = 324,
    /// 指定部门、角色、职务
    OrgRoleDuty = 325,
    /// 指定部门、岗位、职务
    OrgPostDuty = 345,
    /// 指定部门、角色、岗位、职务
    OrgRolePostDuty = 3245,
}

impl AuthMethod {
    pub fn to_int(&self) -> i32 {
        match self {
            AuthMethod::All => 0,
            AuthMethod::User => 1,
            AuthMethod::Role => 2,
            AuthMethod::Org => 3,
            AuthMethod::Post => 4,
            AuthMethod::Duty => 5,
            AuthMethod::RolePost => 24,
            AuthMethod::RoleDuty => 25,
            AuthMethod::OrgRole => 32,
            AuthMethod::OrgPost => 34,
            AuthMethod::OrgDuty => 35,
            AuthMethod::PostDuty => 45,
            AuthMethod::OrgRolePost => 324,
            AuthMethod::OrgRoleDuty => 325,
            AuthMethod::OrgPostDuty => 345,
            AuthMethod::OrgRolePostDuty => 3245,
        }
    }
}
/// Http 请求类型
pub enum HttpMethod {
    GET,
    PUT,
    POST,
    DELETE,
    HEAD,
    PATCH,
    TRACE,
    OPTIONS,
    CONNECT,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HttpMethod::GET => "GET",
            HttpMethod::PUT => "PUT",
            HttpMethod::POST => "POST",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::CONNECT => "CONNECT",
        };
        write!(f, "{}", s)
    }
}

/// 菜单类型
///
/// # 描述
/// - 定义菜单类型，支持目录和功能两种类型，C为目录，F开头为功能
///
/// # 类型值
/// - C：目录
/// - F：功能
pub enum MenuType {
    C,
    F,
}

impl fmt::Display for MenuType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MenuType::C => "C",
            MenuType::F => "F",
        };
        write!(f, "{}", s)
    }
}

/// 操作结果
///
/// # 描述
/// - Success：成功 1
/// - Failed：失败 0
pub enum OperResult {
    Success = 1,
    Failed = 0,
}

impl fmt::Display for OperResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OperResult::Success => "1",
            OperResult::Failed => "0",
        };
        write!(f, "{}", s)
    }
}
