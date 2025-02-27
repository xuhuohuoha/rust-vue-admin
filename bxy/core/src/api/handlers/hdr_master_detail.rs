//!
//! 主从关联业务接口
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_master_detail::{MdReq, MdSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_master_detail, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::MdModel;

pub async fn add(user: Claims, Json(req): Json<MdReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_master_detail::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增主从关联成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn edit(user: Claims, Json(req): Json<MdReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_master_detail::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑主从关联成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_master_detail::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定主从关联成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_master_detail::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除主从关联成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_master_detail::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定主从关联成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_master_detail::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除主从关联成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<MdModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_master_detail::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定指定成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<MdSearchReq>,
) -> ApiRes<PageData<MdModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_master_detail::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取主从关联分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
