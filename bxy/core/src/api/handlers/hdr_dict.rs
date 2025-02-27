//!
//! 数据字典业务接口
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_dict::{DictReq, DictSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_dict, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::DictModel;

/// 新增数据字典
#[utoipa::path(
    method(post),
    path = "/api/v1/core/dict/type/add",
    tag = crate::api::v1::AUTH_TAG,
    request_body = DictReq,
    responses(
        (status = 200, description = "新增数据字典成功", body = ApiRes<String>),
    ),
)]
pub async fn add(user: Claims, Json(req): Json<DictReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dict::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增数据字典成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 编辑数据字典
#[utoipa::path(
    method(put),
    path = "/api/v1/core/dict/type/edit",
    tag = crate::api::v1::AUTH_TAG,
    request_body = DictReq,
    responses(
        (status = 200, description = "编辑数据字典成功", body = ApiRes<String>),
    ),
)]
pub async fn edit(user: Claims, Json(req): Json<DictReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dict::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑数据字典成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除数据字典（根据 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/dict/type/remove_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "软删除数据字典成功", body = ApiRes<String>),
    ),
)]
pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dict::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除数据字典成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除数据字典（批量 ID）
#[utoipa::path(
    method(put),
    path = "/api/v1/core/dict/type/remove",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "软删除数据字典成功", body = ApiRes<String>),
    ),
)]
pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dict::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定数据字典成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除数据字典
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/dict/type/delete_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "硬删除数据字典成功", body = ApiRes<String>),
    ),
)]
pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dict::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定数据字典成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除数据字典
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/dict/type/delete",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "硬删除数据字典成功", body = ApiRes<String>),
    ),
)]
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dict::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除数据字典成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 根据ID获取数据字典
#[utoipa::path(
    method(get),
    path = "/api/v1/core/dict/type/find_by_id/{id}",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "获取指定数据字典成功", body = ApiRes<String>),
    ),
)]
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<DictModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dict::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定数据字典成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 获取数据字典分页列表
#[utoipa::path(
    method(get),
    path = "/api/v1/core/dict/type/find_all",
    tag = crate::api::v1::AUTH_TAG,
    responses(
        (status = 200, description = "获取数据字典分页列表成功", body = ApiRes<String>),
    ),
)]
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<DictSearchReq>,
) -> ApiRes<PageData<DictModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dict::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取数据字典分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
