//!
//! 部门业务接口
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_org::{OrgReq, OrgSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_org, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::OrgModel;

pub async fn add(user: Claims, Json(req): Json<OrgReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_org::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增部门成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn edit(user: Claims, Json(req): Json<OrgReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_org::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑部门成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_org::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定部门成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_org::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除部门成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_org::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定部门成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_org::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除部门成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<OrgModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_org::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定指定成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<OrgSearchReq>,
) -> ApiRes<PageData<OrgModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_org::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取部门分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
