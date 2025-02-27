//! 鉴权中间件
//!
//! # 描述
//! - 配置文件用户列表中的账号直接放行
//! - 非配置文件用户列表中的账号，对于存在于路由表中的Api进行鉴权
//!

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::{
    config::CONFIG,
    model::app_structs::{RequestContext, UserInfoContext},
    serve::utils::ApiUtils,
};

/// 鉴权中间件
pub async fn auth_fn_mid(req: Request, next: Next) -> Result<Response, StatusCode> {
    let ctx = req
        .extensions()
        .get::<RequestContext>()
        .expect("RequestContext not found");
    let user = req
        .extensions()
        .get::<UserInfoContext>()
        .expect("UserInfoContext not found");
    // 如果是超级用户，则不需要验证权限，直接放行
    if CONFIG.system.super_user.contains(&user.u_id) {
        return Ok(next.run(req).await);
    }
    // 验证api权限，如果不在路由表中，则放行，否则验证权限
    if ApiUtils::is_api_in(&ctx.path).await
        && !ApiUtils::check_api_permission(&user.u_id, "", &ctx.path, &ctx.method).await
    {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(req).await)
}
