use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_row_auth::{RowAuthReq, RowAuthSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_row_auth, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::RowAuthModel;

/// 新增行级权限
#[utoipa::path(
    method(post),
    path = "/api/v1/core/auth/row/add",
    tag = crate::api::v1::AUTH_TAG,
    request_body = RowAuthReq,
    responses(
        (status = 200, description = "新增行级权限成功", body = ApiRes<String>),
    ),
)]
pub async fn add(user: Claims, Json(req): Json<RowAuthReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_row_auth::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增行级权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 编辑行级权限
#[utoipa::path(
    method(put),
    path = "/api/v1/core/auth/row/edit",
    tag = crate::api::v1::AUTH_TAG,
    request_body = RowAuthReq,
    responses(
        (status = 200, description = "编辑行级权限成功", body = ApiRes<String>),
    ),
)]
pub async fn edit(user: Claims, Json(req): Json<RowAuthReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_row_auth::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑行级权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除行级权限（根据 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/auth/row/remove_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "软删除行级权限成功", body = ApiRes<String>),
    ),
)]
pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_row_auth::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除行级权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除行级权限（批量 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/auth/row/remove",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "软删除行级权限成功", body = ApiRes<String>),
    ),
)]
pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_row_auth::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定行级权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除行级权限
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/auth/row/delete_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "硬删除行级权限成功", body = ApiRes<String>),
    ),
)]
pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_row_auth::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定行级权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除行级权限
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/auth/row/delete",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "硬删除行级权限成功", body = ApiRes<String>),
    ),
)]
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_row_auth::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除行级权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 根据ID获取行级权限
#[utoipa::path(
    method(get),
    path = "/api/v1/core/auth/row/find_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "获取指定行级权限成功", body = ApiRes<String>),
    ),
)]
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<RowAuthModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_row_auth::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定行级权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 获取行级权限分页列表
#[utoipa::path(
    method(get),
    path = "/api/v1/core/auth/row/find_all",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "获取行级权限分页列表成功", body = ApiRes<String>),
    ),
)]
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<RowAuthSearchReq>,
) -> ApiRes<PageData<RowAuthModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_row_auth::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取行级权限分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
