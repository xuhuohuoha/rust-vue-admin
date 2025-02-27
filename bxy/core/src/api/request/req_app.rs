use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
    "app_code": "ZSOBSONBA0021JR339I034",
    "app_key": "ZSOBSONBA0021JR339I034"
})
)]
/// 应用授权验证请求体结构
pub struct AppAuthReq {
    /// 应用授权码
    pub app_code: String,
    /// 应用授权密钥
    pub app_key: String,
}

/// 临时授权码置换请求体结构
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
    "app_code": "ZSOBSONBA0021JR339I034",
    "temp_code": "ZSOBSONBA0021JR339I034"
})
)]
pub struct AppTempReq {
    /// 应用代码
    pub app_code: String,
    /// 临时授权码
    pub temp_code: String,
}

/// 应用新增、编辑请求体结构
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct AppReq {
    /// 物理主键
    pub id: String,
    /// 业务主键
    pub guid: String,
    /// 排序
    pub ord: u32,
    /// 状态
    pub status: String,
    /// 备注
    pub remark: String,
    /// 应用授权码
    pub app_code: String,
    /// 应用授权密钥
    pub app_key: String,
    /// 应用名称
    pub app_name: String,
    /// 应用属性
    pub app_att: String,
    /// 应用标志
    pub logo: String,
    /// 应用类型
    pub app_type: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 应用查询请求体结构
pub struct AppSearchReq {
    /// 应用代码
    pub guid: Option<String>,
    /// 应用授权码
    pub app_code: Option<String>,
    /// 应用授权密钥
    pub app_key: Option<String>,
    /// 应用名称
    pub app_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
/// 应用授权查询请求体结构
pub struct AppAuthSearchReq {
    pub app_code: Option<String>,
    pub temp_code: Option<String>,
}
