//! Blunka BPM Api Version 1.0
//!

use super::route;
use axum::Router;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

/// 通用
pub const COMMON_TAG: &str = "Common";
/// 附件
pub const ADTION_TAG: &str = "Adtion";
/// 应用
pub const APP_TAG: &str = "App";
/// 接口
pub const API_TAG: &str = "Api";
/// 授权
pub const AUTH_TAG: &str = "Auth";
/// 字典
pub const DICT_TAG: &str = "Dict";
/// 动态脚本
pub const DQL_TAG: &str = "DQL";
/// 数据源
pub const DS_TAG: &str = "DataSource";
/// 登录
pub const LOGIN_TAG: &str = "Login";
/// 日志
pub const LOG_TAG: &str = "Log";
/// 菜单
pub const MENU_TAG: &str = "Menu";
/// 用户
pub const USER_TAG: &str = "User";

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::get_uuid,   // 获取UUID
        crate::api::get_captcha,    // 获取验证码
        crate::api::get_salt,   // 获取长度为10的盐
        crate::api::get_salt_len,   // 获取指定长度的盐
        crate::api::get_encrypt_password,   // 密码加密
        crate::api::get_server_info,
        // crate::api::handlers::handler_add,
        // crate::api::handlers::handler_edit,
        // crate::api::handlers::handler_update,
        // crate::api::handlers::handler_remove,
        // crate::api::handlers::handler_delete,
        // crate::api::handlers::handler_list,
        // crate::api::handlers::handler_single,
        crate::api::handlers::hdr_adtion::add,
        crate::api::handlers::hdr_adtion::delete,
        crate::api::handlers::hdr_adtion::delete_by_id,
        crate::api::handlers::hdr_adtion::find_by_id,
        crate::api::handlers::hdr_adtion::find_by_guid,
        crate::api::handlers::hdr_adtion::find_all,
        crate::api::handlers::hdr_adtion_ex::add,
        crate::api::handlers::hdr_adtion_ex::edit,
        crate::api::handlers::hdr_adtion_ex::remove,
        crate::api::handlers::hdr_adtion_ex::remove_by_id,
        crate::api::handlers::hdr_adtion_ex::delete_by_id,
        crate::api::handlers::hdr_adtion_ex::delete,
        crate::api::handlers::hdr_adtion_ex::find_by_id,
        crate::api::handlers::hdr_adtion_ex::find_all,
        crate::api::handlers::hdr_app::add,
        crate::api::handlers::hdr_app::edit,
        crate::api::handlers::hdr_app::remove_by_id,
        crate::api::handlers::hdr_app::remove,
        crate::api::handlers::hdr_app::delete_by_id,
        crate::api::handlers::hdr_app::delete,
        crate::api::handlers::hdr_app::find_by_id,
        crate::api::handlers::hdr_app::find_all,
        crate::api::handlers::hdr_app::app_auth,
        crate::api::handlers::hdr_app::app_auth_code,
        crate::api::handlers::hdr_user::login,
        crate::api::handlers::hdr_user::logout,
        crate::api::handlers::hdr_user::add,
        crate::api::handlers::hdr_user::edit,
        crate::api::handlers::hdr_user::remove_by_id,
        crate::api::handlers::hdr_user::remove,
        crate::api::handlers::hdr_user::delete,
        crate::api::handlers::hdr_user::delete_by_id,
        crate::api::handlers::hdr_user::find_by_id,
        crate::api::handlers::hdr_user::find_all,
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = COMMON_TAG, description = "平台提供通用操作相关接口"),
        (name = ADTION_TAG, description = "平台提供附件操作相关接口"),
        (name = APP_TAG, description = "平台提供应用操作相关接口"),
        (name = API_TAG, description = "平台提供接口操作相关接口"),
        (name = AUTH_TAG, description = "平台提供授权操作相关接口"),
        (name = DICT_TAG, description = "平台提供字典操作相关接口"),
        (name = DQL_TAG, description = "平台提供动态脚本操作相关接口"),
        (name = DS_TAG, description = "平台提供数据源操作相关接口"),
        (name = LOG_TAG, description = "平台提供日志操作相关接口"),
        (name = LOGIN_TAG, description = "平台提供登录操作相关接口"),
        (name = MENU_TAG, description = "平台提供菜单操作相关接口"),
        (name = USER_TAG, description = "平台提供用户操作相关接口"),
    ),
    servers(
        (url = "/v1", description = "Blunka Bpm 2024"),
    ),
)]

/// Api 文档
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there are already components registered.
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

/// 当前版本全局路由入口
pub fn api() -> Router {
    route::api()
}
