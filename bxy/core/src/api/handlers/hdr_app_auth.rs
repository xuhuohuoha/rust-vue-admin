use axum::Json;
use axum_extra::extract::Query;
use byz_entity::core::AppAuthModel;

use crate::{
    api::{
        request::{req::DeleteReq, req_app::AppAuthSearchReq},
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::srv_app_auth,
};

/// 获取应用分页列表
#[utoipa::path(
    method(get),
    path = "/api/v1/core/auth/app/find_all",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "获取应用临时授权分页列表成功", body = ApiRes<String>),
    ),
)]
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<AppAuthSearchReq>,
) -> ApiRes<PageData<AppAuthModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app_auth::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取应用临时授权分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除应用
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/auth/app/delete",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "硬删除应用临时授权成功", body = ApiRes<String>),
    ),
)]
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app_auth::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除应用临时授权成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
