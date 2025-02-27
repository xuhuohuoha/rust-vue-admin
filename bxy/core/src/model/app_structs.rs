use sea_orm::{ConnectionTrait, DbErr, Paginator, SelectorTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 验证码结构
#[derive(Debug, Serialize, ToSchema)]
pub struct CaptchaImage {
    /// 是否启用验证码
    pub captcha_on_off: bool,
    /// 验证码加密串
    pub uuid: String,
    /// 验证码图片base64
    pub img: String,
}

/// 客户端
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ClientNetInfo {
    /// ip地址
    pub ip: String,
    /// 所在地区
    pub location: String,
    /// 运营商
    pub net_work: String,
}

/// 用户代理
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct UserAgentInfo {
    /// 浏览器
    pub browser: String,
    /// 操作系统
    pub os: String,
    /// 设备
    pub device: String,
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct ClientInfo {
    /// 客户端网络信息
    pub net: ClientNetInfo,
    /// 用户代理信息
    pub ua: UserAgentInfo,
}

/// 请求上下文
#[derive(Clone, Debug, Default)]
pub struct RequestContext {
    /// 原始URI
    pub ori_uri: String,
    /// 请求路径
    pub path: String,
    /// 路径参数
    pub path_params: String,
    /// 请求方法
    pub method: String,
    /// 请求数据
    pub data: String,
}

/// 用户信息
#[derive(Debug, Clone, Default)]
pub struct UserInfoContext {
    /// 用户ID
    pub u_id: String,
    /// Token
    pub token: String,
    /// 用户账号
    pub u_code: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ApiInfo {
    /// API标识
    pub api_id: String,
    /// API名称
    pub name: String,
    /// 数据缓存方法
    pub data_cache_method: String,
    /// 日志记录方法
    pub log_method: String,
    /// 具体API
    pub api: Option<String>,
}

/// 分页查询数据结构
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PageData<T> {
    /// 当前页数据
    pub list: Vec<T>,
    /// 当前页码
    pub page_num: u64,
    /// 当前页大小
    pub page_size: u64,
    /// 总记录数
    pub total: u64,
    /// 总页数
    pub total_pages: u64,
}

/// 分页查询参数
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct PageParams {
    /// 当前页码
    pub page_num: Option<u64>,
    /// 当前页大小
    pub page_size: Option<u64>,
}

pub trait ToPageData<T> {
    // async fn to_page_data(self, page_num: u64, page_size: u64) -> Result<PageData<T>, DbErr>
    // where
    //     Self: Send + Sized;
    fn to_page_data(
        self,
        page_num: u64,
        page_size: u64,
    ) -> impl std::future::Future<Output = Result<PageData<T>, DbErr>> + Send;
}

impl<'db, C, S, T> ToPageData<T> for Paginator<'db, C, S>
where
    C: ConnectionTrait,
    S: SelectorTrait<Item = T> + 'db + std::marker::Sync + std::marker::Send,
{
    async fn to_page_data(self, page_num: u64, page_size: u64) -> Result<PageData<T>, DbErr> {
        to_page_data(self, page_num, page_size).await
    }
}

/// 转换paginator为PageData结构
pub async fn to_page_data<'db, C, S, T>(
    mut paginator: Paginator<'db, C, S>,
    page_num: u64,
    page_size: u64,
) -> Result<PageData<T>, DbErr>
where
    C: ConnectionTrait,
    S: SelectorTrait<Item = T> + 'db,
{
    let range = 1..page_num;
    range.for_each(|_| {
        paginator.next();
    });
    let total_pages = paginator.num_pages().await?;
    let total = paginator.num_items().await?;
    let list = paginator.fetch().await?;
    let page_num = paginator.cur_page() + 1;
    let data = PageData {
        list,
        page_num,
        page_size,
        total,
        total_pages,
    };
    Ok(data)
}

/// 分页查询公用结构
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageQuery<T> {
    /// 当前页码
    pub page_num: u64,
    /// 当前页大小
    pub page_size: u64,
    /// 当前页数据
    pub data: T,
}
