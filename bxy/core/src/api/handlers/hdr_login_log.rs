//!
//!  登录日志业务接口
//!

use axum::Json;
use axum_extra::extract::Query;
use byz_entity::core::LoginLogModel;

use crate::{
    api::{
        request::{req::DeleteReq, req_login::LoginLogSearchReq},
        response::res::ApiRes,
    },
    model::app_structs::{PageData, PageParams},
    serve::srv_login_log,
};

/// 获取登录日志分页列表
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<LoginLogSearchReq>,
) -> ApiRes<PageData<LoginLogModel>> {
    match srv_login_log::find_all(page_params, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 删除登录日志
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    match srv_login_log::delete(req).await {
        Ok(x) => ApiRes::with_msg(&x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 清空登录日志
pub async fn clean() -> ApiRes<String> {
    match srv_login_log::clean().await {
        Ok(x) => ApiRes::with_msg(&x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
