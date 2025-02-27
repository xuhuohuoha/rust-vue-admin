//!
//! 应用业务接口
//!

use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::{AppAuthModel, AppModel};

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_app::{AppAuthReq, AppReq, AppSearchReq, AppTempReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_app, srv_app_auth, utils::jwt::Claims},
};

/// 新增应用
#[utoipa::path(
    method(post),
    path = "/api/v1/core/app/add",
    tag = crate::api::v1::APP_TAG,
    request_body = AppReq,
    responses(
        (status = 200, description = "新增应用成功", body = ApiRes<String>),
    ),
)]
pub async fn add(user: Claims, Json(req): Json<AppReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增应用成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 编辑应用
#[utoipa::path(
    method(put),
    path = "/api/v1/core/app/edit",
    tag = crate::api::v1::APP_TAG,
    request_body = AppReq,
    responses(
        (status = 200, description = "编辑应用成功", body = ApiRes<String>),
    ),
)]
pub async fn edit(user: Claims, Json(req): Json<AppReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑应用成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除应用（根据 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/app/remove_by_id/{id}",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "软删除应用成功", body = ApiRes<String>),
    ),
)]
pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除应用成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除应用（批量 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/app/remove",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "软删除应用成功", body = ApiRes<String>),
    ),
)]
pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定应用成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除应用
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/app/delete_by_id/{id}",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "硬删除应用成功", body = ApiRes<String>),
    ),
)]
pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定应用成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除应用
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/app/delete",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "硬删除应用成功", body = ApiRes<String>),
    ),
)]
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除应用成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 根据ID获取应用
#[utoipa::path(
    method(get),
    path = "/api/v1/core/app/find_by_id/{id}",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "获取指定应用成功", body = ApiRes<String>),
    ),
)]
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<AppModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定应用成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 获取应用分页列表
#[utoipa::path(
    method(get),
    path = "/api/v1/core/app/find_all",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "获取应用分页列表成功", body = ApiRes<String>),
    ),
)]
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<AppSearchReq>,
) -> ApiRes<PageData<AppModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取应用分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 应用授权校验
#[utoipa::path(
    method(get),
    path = "/api/v1/comm/app/app_auth",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = 200, description = "应用授权检验成功", body = ApiRes<String>, content_type = "text/plain"),
    ),
)]
pub async fn app_auth(Query(req): Query<AppAuthReq>) -> ApiRes<AppAuthModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app::auth(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "应用授权检验成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 应用临时授权码校验
#[utoipa::path(
    method(get),
    path = "/api/v1/comm/app/app_auth_code",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = 200, description = "应用临时授权码校验功", body = ApiRes<String>, content_type = "text/plain"),
    ),
)]
pub async fn app_auth_code(Query(req): Query<AppTempReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_app_auth::auth_code(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "应用临时授权码校验功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
