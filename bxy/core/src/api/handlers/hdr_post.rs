//!
//! 岗位业务接口
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_post::{PostReq, PostSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_post, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::PostModel;

pub async fn add(user: Claims, Json(req): Json<PostReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_post::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增岗位成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn edit(user: Claims, Json(req): Json<PostReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_post::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑岗位成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_post::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定岗位成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_post::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除岗位成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_post::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定岗位成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_post::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除岗位成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<PostModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_post::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定指定成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<PostSearchReq>,
) -> ApiRes<PageData<PostModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_post::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取岗位分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
