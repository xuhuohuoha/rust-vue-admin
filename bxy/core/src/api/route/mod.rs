//!
//! 全局路由
//!
use axum::{
    middleware,
    routing::{get, get_service, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::{config::CONFIG, middlewarer, serve::utils::jwt::Claims};

use super::{
    get_captcha, get_encrypt_password, get_salt, get_salt_len, get_server_info, get_uuid,
    handlers::{core_api, hdr_app, hdr_user},
};
/// 全局路由
///
/// 包括：
///     1）附件上传接口
///     2）无需鉴权接口
///     3）需要鉴权接口
pub fn api() -> Router {
    Router::new()
        // 文件上传api
        .nest_service(
            &CONFIG.web.upload_url,
            get_service(ServeDir::new(&CONFIG.web.upload_dir)),
        )
        // 无需授权Api.通用模块
        .nest("/comm", no_auth_api())
        // 系统管理模块
        .nest("/core", set_auth_middleware(core_api()))
}

/// 无需授权 api
fn no_auth_api() -> Router {
    Router::new()
        .route("/login", post(hdr_user::login)) // 登录
        .route("/logout", post(hdr_user::logout)) // 退出
        .route("/get_salt", get(get_salt)) // 获取长度为10的盐
        .route("/get_salt_len/{len}", get(get_salt_len)) //获取指定长度的盐
        .route("/get_encrypt_password", get(get_encrypt_password)) //密码加密
        .route("/get_captcha", get(get_captcha)) // 获取验证码
        .route("/get_uuid", get(get_uuid)) // 获取uuid
        .route("/get_server_info", get(get_server_info)) // 获取服务器信息
        .route("/app_auth", get(hdr_app::app_auth)) // 应用授权校验
        .route("/app_auth_code", get(hdr_app::app_auth_code)) // 换取临时授权码
}

/// 设置授权路由的中间件
fn set_auth_middleware(router: Router) -> Router {
    let router = match &CONFIG.log.enable_oper_log {
        true => router.layer(middleware::from_fn(middlewarer::OperLog)),
        false => router,
    };
    let router = match CONFIG.server.cache_time {
        0 => router,
        _ => {
            if CONFIG.server.cache_method == 0 {
                // router.layer(middleware::from_fn(middlewarer::Cache));
                router.layer(())
            } else {
                // router.layer(middleware::from_fn(middlewarer::SkyTableCache))
                router.layer(())
            }
        }
    };
    #[allow(clippy::let_and_return)]
    let router = router
        .layer(middleware::from_fn(middlewarer::ApiAuth)) // Api 鉴权
        .layer(middleware::from_fn(middlewarer::Context)) // Context 注入
        .layer(middleware::from_extractor::<Claims>());
    router
}
