//!
//! 字段权限业务接口
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_col_auth::{ColAuthReq, ColAuthSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_col_auth, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::ColAuthModel;

/// 新增字段权限
#[utoipa::path(
    method(post),
    path = "/api/v1/core/auth/col/add",
    tag = crate::api::v1::AUTH_TAG,
    request_body = ColAuthReq,
    responses(
        (status = 200, description = "新增字段权限成功", body = ApiRes<String>),
    ),
)]
pub async fn add(user: Claims, Json(req): Json<ColAuthReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_col_auth::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增字段权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 编辑字段权限
#[utoipa::path(
    method(put),
    path = "/api/v1/core/auth/col/edit",
    tag = crate::api::v1::AUTH_TAG,
    request_body = ColAuthReq,
    responses(
        (status = 200, description = "编辑字段权限成功", body = ApiRes<String>),
    ),
)]
pub async fn edit(user: Claims, Json(req): Json<ColAuthReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_col_auth::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑字段权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除字段权限（根据 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/auth/col/remove_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "软删除字段权限成功", body = ApiRes<String>),
    ),
)]
pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_col_auth::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除字段权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除字段权限（批量 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/auth/col/remove",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "软删除字段权限成功", body = ApiRes<String>),
    ),
)]
pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_col_auth::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定字段权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除字段权限
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/auth/col/delete_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "硬删除字段权限成功", body = ApiRes<String>),
    ),
)]
pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_col_auth::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定字段权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除字段权限
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/auth/col/delete",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "硬删除字段权限成功", body = ApiRes<String>),
    ),
)]
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_col_auth::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除字段权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 根据ID获取字段权限
#[utoipa::path(
    method(get),
    path = "/api/v1/core/auth/col/find_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "获取指定字段权限成功", body = ApiRes<String>),
    ),
)]
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<ColAuthModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_col_auth::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定字段权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 获取字段权限分页列表
#[utoipa::path(
    method(get),
    path = "/api/v1/core/auth/col/find_all",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "获取字段权限分页列表成功", body = ApiRes<String>),
    ),
)]
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<ColAuthSearchReq>,
) -> ApiRes<PageData<ColAuthModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_col_auth::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取字段权限分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
