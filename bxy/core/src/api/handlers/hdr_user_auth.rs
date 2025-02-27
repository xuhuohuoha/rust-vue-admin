//!
//! 功能权限业务接口
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_user::{UserAuthReq, UserAuthSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_user_auth, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::UserAuthModel;

/// 新增功能权限
#[utoipa::path(
    method(post),
    path = "/api/v1/core/auth/fn/add",
    tag = crate::api::v1::AUTH_TAG,
    request_body = UserAuthReq,
    responses(
        (status = 200, description = "新增功能权限成功", body = ApiRes<String>),
    ),
)]
pub async fn add(user: Claims, Json(req): Json<UserAuthReq>) -> ApiRes<Vec<String>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user_auth::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增功能权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 编辑功能权限
#[utoipa::path(
    method(put),
    path = "/api/v1/core/auth/fn/edit",
    tag = crate::api::v1::AUTH_TAG,
    request_body = UserAuthReq,
    responses(
        (status = 200, description = "编辑功能权限成功", body = ApiRes<String>),
    ),
)]
pub async fn edit(user: Claims, Json(req): Json<UserAuthReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user_auth::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑功能权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除功能权限（根据 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/auth/fn/remove_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "软删除功能权限成功", body = ApiRes<String>),
    ),
)]
pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user_auth::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除功能权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除功能权限（批量 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/auth/fn/remove",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "软删除功能权限成功", body = ApiRes<String>),
    ),
)]
pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user_auth::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定功能权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除功能权限
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/auth/fn/delete_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "硬删除功能权限成功", body = ApiRes<String>),
    ),
)]
pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user_auth::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定功能权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除功能权限
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/auth/fn/delete",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "硬删除功能权限成功", body = ApiRes<String>),
    ),
)]
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user_auth::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除功能权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 根据ID获取功能权限
#[utoipa::path(
    method(get),
    path = "/api/v1/core/auth/fn/find_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "获取指定功能权限成功", body = ApiRes<String>),
    ),
)]
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<UserAuthModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user_auth::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定功能权限成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 获取功能权限分页列表
#[utoipa::path(
    method(get),
    path = "/api/v1/core/auth/fn/find_all",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "获取功能权限分页列表成功", body = ApiRes<String>),
    ),
)]
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<UserAuthSearchReq>,
) -> ApiRes<PageData<UserAuthModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user_auth::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取功能权限分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
