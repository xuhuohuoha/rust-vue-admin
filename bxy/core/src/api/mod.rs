pub mod handlers;
pub mod request;
pub mod response;
pub mod route;
pub mod v1;

use std::{convert::Infallible, time::Duration};

use crate::{
    model::{app_structs::CaptchaImage, server_info::SysInfo},
    serve::{self, srv_server},
    utils::{EncryptUtils, RandUtils},
};
use axum::{
    extract::Path,
    response::{sse::Event, Sse},
    Json, Router,
};
use futures::stream::{self, Stream};
use request::req::EncryptPasswordReq;
use response::res::ApiRes;
use tokio_stream::StreamExt;

/// 获取UUID
///
/// # 描述
/// 获取一个UUID
///
#[utoipa::path(
    method(get),
    path = "/api/v1/comm/get_uuid",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn get_uuid() -> ApiRes<String> {
    ApiRes::with_data(scru128::new_string().to_uppercase())
}

/// 获取长度为10的盐
///
/// # 描述
/// 获取长度为10的盐
///
#[utoipa::path(
    method(get),
    path = "/api/v1/comm/get_salt",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn get_salt() -> ApiRes<String> {
    ApiRes::with_data(RandUtils::rand_str(10))
}

/// 获取指定长度的盐
#[utoipa::path(
    method(get),
    path = "/api/v1/comm/get_salt_len/{len}",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn get_salt_len(Path(len): Path<usize>) -> ApiRes<String> {
    ApiRes::with_data(RandUtils::rand_str(len))
}

/// 用指定盐对密码进行加密
#[utoipa::path(
    method(get),
    path = "/api/v1/comm/get_encrypt_password",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn get_encrypt_password(Json(req): Json<EncryptPasswordReq>) -> ApiRes<String> {
    ApiRes::with_data(EncryptUtils::encrypt_password(&req.password, &req.salt))
}

/// 获取验证码
#[utoipa::path(
    method(get),
    path = "/api/v1/comm/get_captcha",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn get_captcha() -> ApiRes<CaptchaImage> {
    let res = serve::get_captcha();
    ApiRes::with_data(res)
}

/// 获取系统信息
#[utoipa::path(
    method(get),
    path = "/api/v1/comm/get_server_info",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn get_server_info() -> ApiRes<SysInfo> {
    let res = srv_server::get_oper_sys_info();
    ApiRes::with_data(res)
}

/// V1 版本接口入口
pub fn api_v1() -> Router {
    Router::new().nest("/v1", v1::api())
}

/// 获取系统信息
pub async fn get_server_info_sse() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let sys_info = srv_server::get_oper_sys_info();
        Event::default().data(serde_json::to_string(&sys_info).unwrap_or_else(|_| "0".to_string()))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
