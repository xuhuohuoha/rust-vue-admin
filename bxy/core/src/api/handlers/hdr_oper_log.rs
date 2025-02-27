use axum::Json;
use axum_extra::extract::Query;
use byz_entity::core::OperLogModel;

use crate::{
    api::{
        request::{req::DeleteReq, req_log::OperLogSearchReq},
        response::res::ApiRes,
    },
    model::app_structs::{PageData, PageParams},
    serve::srv_oper_log,
};

/// 获取操作日志分页列表
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<OperLogSearchReq>,
) -> ApiRes<PageData<OperLogModel>> {
    match srv_oper_log::find_all(page_params, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 删除登录日志
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    match srv_oper_log::delete(req).await {
        Ok(x) => ApiRes::with_msg(&x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 清空登录日志
pub async fn clean() -> ApiRes<String> {
    match srv_oper_log::clean().await {
        Ok(x) => ApiRes::with_msg(&x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
