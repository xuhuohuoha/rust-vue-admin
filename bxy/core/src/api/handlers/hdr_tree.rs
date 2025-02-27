//!
//! 关联树业务接口
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_tree::{TreeReq, TreeSearchReq},
        },
        response::{res::ApiRes, res_tree::TreeRes},
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_tree, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::TreeModel;

/// 新增关联树
pub async fn add(user: Claims, Json(req): Json<TreeReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_tree::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增关联树成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 编辑关联树
pub async fn edit(user: Claims, Json(req): Json<TreeReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_tree::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑关联树成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除（指定 ID)
pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_tree::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定关联树成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除（批量 ID）
pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_tree::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除关联树成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除（指定 ID）
pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_tree::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定关联树成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除（批量 ID）
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_tree::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除关联树成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询关联树（指定 ID）
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<TreeModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_tree::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定指定关联树成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询关联树（指定 ID）
pub async fn find_by_mcode_uid(
    user: Claims,
    Path(mcode): Path<String>,
) -> ApiRes<(TreeRes, Vec<sea_orm::JsonValue>)> {
    let db = DB.get_or_init(db_conn).await;
    match srv_tree::find_by_mcode_uid(db, &mcode, &user.u_id).await {
        Ok(x) => match srv_tree::find_mcode_uid_data(db, &x).await {
            Ok(y) => ApiRes::with_data((x, y)),
            Err(e) => ApiRes::with_err(&e.to_string()),
        },
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询关联树（分页）
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<TreeSearchReq>,
) -> ApiRes<PageData<TreeModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_tree::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取关联树分页列表成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
