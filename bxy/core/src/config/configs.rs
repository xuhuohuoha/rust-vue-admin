//! 配置文件 config/config.toml 中的所有配置
//!
//! # 描述
//! 伯伦卡业务流程平台通过配置文件加载的配置
//!
//! # 详细配置信息
//! - 程序配置
//! - 静态网站配置
//! - 证书配置
//! - 系统配置
//! - 数据库配置
//! - JWT配置
//! - 日志配置
//! - SkyTable配置
//!
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Configs {
    /// 程序配置
    pub server: Server,
    /// 静态网站配置
    pub web: Web,
    /// cert配置
    pub cert: Cert,
    /// 系统配置
    pub system: System,
    /// 数据库配置
    pub database: Database,
    /// JWT配置
    pub jwt: Jwt,
    /// 日志配置
    pub log: Log,
    /// skytable
    pub skytable: SkyTable,
}

/// server 配置文件
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Server {
    /// 服务器名称
    pub name: String,
    /// 服务器(IP地址:端口)
    /// `0.0.0.0:3000`
    pub address: String,
    /// 服务器ssl
    pub ssl: bool,
    /// 响应数据gzip
    pub content_gzip: bool,
    /// 缓存时间
    pub cache_time: u64,
    /// 缓存方式
    pub cache_method: u32,
    /// api 前缀  例如："/api_v1"
    pub api_prefix: String,
}

/// server 配置文件
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Web {
    /// 静态网站根目录
    pub dir: String,
    /// 静态网站index文件名
    /// `index.html`
    pub index: String,
    /// 文件上传路径
    pub upload_dir: String,
    /// 文件上传路径
    pub upload_url: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Cert {
    /// 证书
    pub cert: String,
    /// 密钥
    pub key: String,
}

/// system 系统配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct System {
    /// 超级管理员账号列表
    pub super_user: Vec<String>,
    /// user agent 解析
    pub user_agent_parser: String,
}

/// jwt 配置文件
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Jwt {
    /// JWT 密钥
    pub jwt_secret: String,
    /// JWT 过期时间
    pub jwt_exp: i64,
}

/// 日志配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Log {
    /// `log_level` 日志输出等级
    pub log_level: String,
    /// `dir` 日志输出文件夹
    pub dir: String,
    /// `file` 日志输出文件名
    pub file: String,
    /// 允许操作日志输出
    pub enable_oper_log: bool,
}

/// 数据库连接配置
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Database {
    /// 数据库连接
    pub url: String,
    /// 最大连接数
    pub max_connections: u32,
    /// 最小连接数
    pub min_connections: u32,
    /// 等待连接池分配连接的最大时长
    pub connect_timeout: u64,
    /// 连接 idle 状态最大时长
    pub idle_timeout: u64,
    /// 是否启用 sqlx 日志
    pub sqlx_logging: bool,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct SkyTable {
    /// 服务地址
    pub server: String,
    /// 服务端口
    pub port: u16,
}
