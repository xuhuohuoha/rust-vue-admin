use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({
    "usercode": "admin",
    "password": "2023-09-09T15:53:00",
    "code": "X89G",
    "app_code": "ZSOBSONBA0021JR339I034",
    "app_key": "ZSOBSONBA0021JR339I034",
    "uuid": "ZSOBSONBA0021JR339I034",
    "temp_code": "ZSOBSONBA0021JR339I034"
}))]
/// 登录请求体结构
///
/// 应用授权码 + 应用授权密钥  与  临时授权码二选一
/// 临时授权码不为空，则以临时授权码为准，否则以应用授权码 + 应用授权密钥为准
pub struct LoginReq {
    /// 用户账号
    pub usercode: String,
    /// 用户密码
    pub password: String,
    /// 验证码
    pub code: String,
    /// 应用授权码
    pub app_code: String,
    /// 应用授权密钥
    pub app_key: String,
    /// 验证码加密串
    pub uuid: String,
    /// 临时授权码
    pub temp_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({
    "ip": "114.114.114.114",
    "u_id": "admin",
    "status": "0",
    "order_by_column": "login_time",
    "is_asc": "asc",
    "begin_time": "2024-11-11",
    "end_time": "2024-12-11"
}))]
/// 查询登录日志请求体结构
pub struct LoginLogSearchReq {
    /// IP地址
    pub ip: Option<String>,
    /// 用户账号
    pub u_id: Option<String>,
    /// 状态
    pub status: Option<String>,
    /// 排序字段
    pub order_by_column: Option<String>,
    /// 排序方式
    pub is_asc: Option<String>,
    /// 登录起始时间
    pub begin_time: Option<String>,
    /// 登录截止时间
    pub end_time: Option<String>,
}
