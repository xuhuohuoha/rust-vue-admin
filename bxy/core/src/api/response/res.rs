use std::fmt::Debug;

use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use chrono::Local;
use serde::Serialize;
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

/// 请求通用返回结构
#[derive(Debug, Serialize, Default, ToSchema)]
pub struct ApiRes<T> {
    /// 是否成功
    success: bool,
    /// 状态码
    code: Option<u16>,
    /// 消息
    message: Option<String>,
    /// 数据
    data: Option<T>,
    /// 时间戳
    timestamp: i64,
}

/// 填入到extensions中的数据
#[derive(Debug, Clone)]
pub struct ResJsonString(pub String);

#[allow(unconditional_recursion)]
impl<T> IntoResponse for ApiRes<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        let data = Self {
            success: self.success,
            code: self.code,
            data: self.data,
            message: self.message,
            timestamp: Local::now().timestamp(),
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )
                    .body(Body::from(e.to_string()))
                    .unwrap();
            }
        };
        let res_json_string = ResJsonString(json_string.clone());
        let mut response = json_string.into_response();
        response.extensions_mut().insert(res_json_string);
        response
    }
}

impl<T: Serialize> ApiRes<T> {
    /// 成功并返回数据
    pub fn with_data(data: T) -> Self {
        Self {
            success: true,
            code: Some(200),
            data: Some(data),
            message: Some("success".to_string()),
            timestamp: Local::now().timestamp(),
        }
    }
    /// 成功并返回消息
    pub fn with_msg(msg: &str) -> Self {
        Self {
            success: true,
            code: Some(200),
            data: None,
            message: Some(msg.to_string()),
            timestamp: Local::now().timestamp(),
        }
    }
    /// 成功并返回数据、消息
    pub fn with_data_msg(data: T, msg: &str) -> Self {
        Self {
            success: true,
            code: Some(200),
            data: Some(data),
            message: Some(msg.to_string()),
            timestamp: Local::now().timestamp(),
        }
    }
    /// 错误并返回错误信息
    pub fn with_err(err: &str) -> Self {
        Self {
            success: false,
            code: Some(500),
            data: None,
            message: Some(err.to_string()),
            timestamp: Local::now().timestamp(),
        }
    }
    /// 错误并返回错误信息、错误码
    pub fn with_err_code(err: &str, code: u16) -> Self {
        Self {
            success: false,
            code: Some(code),
            data: None,
            message: Some(err.to_string()),
            timestamp: Local::now().timestamp(),
        }
    }
}
