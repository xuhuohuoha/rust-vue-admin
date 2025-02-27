//!
//! 动态脚本业务接口
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_dql::{DqlReq, DqlSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_dql, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::DqlModel;

pub async fn add(user: Claims, Json(req): Json<DqlReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dql::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增动态脚本成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn edit(user: Claims, Json(req): Json<DqlReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dql::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑动态脚本成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dql::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定动态脚本成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dql::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除动态脚本成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dql::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定动态脚本成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dql::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除动态脚本成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<DqlModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dql::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定动态脚本成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_by_sign(Path(sign): Path<String>) -> ApiRes<DqlModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dql::find_by_sign(db, &sign).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定动态脚本成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<DqlSearchReq>,
) -> ApiRes<PageData<DqlModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_dql::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取动态脚本分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
