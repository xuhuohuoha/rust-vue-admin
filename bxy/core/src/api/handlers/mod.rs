//!
//! Blunka BPM Core Route
//!
//!
pub mod hdr_adtion;
pub mod hdr_adtion_ex;
pub mod hdr_app;
pub mod hdr_app_auth;
pub mod hdr_col_auth;
pub mod hdr_datasource;
pub mod hdr_dict;
pub mod hdr_dict_data;
pub mod hdr_dql;
pub mod hdr_duty;
pub mod hdr_login_log;
pub mod hdr_master_detail;
pub mod hdr_menu;
pub mod hdr_online;
pub mod hdr_oper_log;
pub mod hdr_org;
pub mod hdr_post;
pub mod hdr_role;
pub mod hdr_row_auth;
pub mod hdr_tree;
pub mod hdr_user;
pub mod hdr_user_api;
pub mod hdr_user_auth;

use axum::{
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_extra::extract::Query;

use crate::{
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{
        orm::srv::{self, ServiceTrait},
        srv_dql,
    },
};

use super::{
    request::req::{FormReq, SearchReq},
    response::res::ApiRes,
};

/// 需授权的核心路由入口
pub fn core_api() -> Router {
    Router::new()
        .route("/add", post(handler_add)) //通用保存
        .route("/edit", post(handler_edit)) //通用编辑
        .route("/remove", put(handler_remove)) //通用软删除
        .route("/delete", delete(handler_delete)) //通用硬删除
        .route("/clean", delete(handler_clean)) // 通用清空
        .route("/all", get(handler_all)) // 通用查询全部数据
        .route("/list", get(handler_list)) //通用分页数据
        .route("/single", get(handler_single)) //通用单条
        .nest("/app", app_api()) // 应用模块
        .nest("/adtion", adtion_api()) // 附件模块
        .nest("/auth", auth_api()) // 授权模块
        .nest("/duty", duty_api()) // 职务模块
        .nest("/dict", dict_api()) // 字典模块
        .nest("/dql", dql_api()) // 动态脚本模块
        .nest("/menu", menu_api()) // 菜单模块
        .nest("/md", md_api()) // 主从模块
        .nest("/user", user_api()) // 用户管理模块
        .nest("/online", online_api()) // 在线用户模块
        .nest("/org", org_api()) // 部门模块
        .nest("/role", role_api()) // 角色模块
        .nest("/post", post_api()) // 岗位模块
        .nest("/log", log_api()) // 日志模块
        .nest("/tree", tree_api()) // 关联树模块
}

/// 通用新增
#[utoipa::path(
    method(post),
    path = "/api/v1/core/add",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn handler_add(Json(req): Json<FormReq>) -> ApiRes<String> {
    let mcode = req.mcode;
    println!("mcode: {}", mcode.as_str());
    ApiRes::with_msg("success")
}

/// 通用编辑
#[utoipa::path(
    method(put),
    path = "/api/v1/core/edit",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn handler_edit(Json(req): Json<FormReq>) -> ApiRes<String> {
    let mcode = req.mcode;
    println!("mcode: {}", mcode.as_str());
    ApiRes::with_msg("success")
}

/// 通用软删除
#[utoipa::path(
    method(put),
    path = "/api/v1/core/remove",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn handler_remove(Json(req): Json<FormReq>) -> ApiRes<String> {
    let mcode = req.mcode;
    println!("mcode: {}", mcode.as_str());
    ApiRes::with_msg("success")
}

/// 通用硬删除
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/delete",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn handler_delete(Json(req): Json<FormReq>) -> ApiRes<String> {
    let mcode = req.mcode;
    println!("mcode: {}", mcode.as_str());
    ApiRes::with_msg("success")
}

/// 通用清空
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/clean",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn handler_clean(Json(req): Json<FormReq>) -> ApiRes<String> {
    let mcode = req.mcode;
    println!("mcode: {}", mcode.as_str());
    ApiRes::with_msg("success")
}

/// 通用所有数据
#[utoipa::path(
    method(get),
    path = "/api/v1/core/all",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn handler_all(Query(req): Query<SearchReq>) -> ApiRes<Vec<sea_orm::JsonValue>> {
    let db = DB.get_or_init(db_conn).await;
    let sign = srv_dql::find_by_sign(db, &req.sign).await.unwrap();
    let res = srv::Service::find_all(db, &sign.dql, req.to_value_vec()).await;
    match res {
        Ok(x) => ApiRes::with_data_msg(x, "获取所有数据成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 通用列表
#[utoipa::path(
    method(get),
    path = "/api/v1/core/list",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn handler_list(
    Query(page_params): Query<PageParams>,
    Query(req): Query<SearchReq>,
) -> ApiRes<PageData<sea_orm::JsonValue>> {
    let db = DB.get_or_init(db_conn).await;
    let sign = srv_dql::find_by_sign(db, &req.sign).await.unwrap();
    let res = srv::Service::find_list(db, &sign.dql, page_params, req.to_value_vec()).await;
    match res {
        Ok(x) => ApiRes::with_data_msg(x, "获取所有数据成功."),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 通用单条
#[utoipa::path(
    method(get),
    path = "/api/v1/core/single",
    tag = crate::api::v1::COMMON_TAG,
    responses(
        (status = OK, description = "Success", body = str, content_type = "application/json")
    )
)]
pub async fn handler_single(Json(req): Json<FormReq>) -> ApiRes<String> {
    let mcode = req.mcode;
    println!("mcode: {}", mcode.as_str());
    ApiRes::with_msg("success")
}

/// 应用模块接口
fn app_api() -> Router {
    Router::new()
        .route("/add", post(hdr_app::add))
        .route("/edit", put(hdr_app::edit))
        .route("/remove_by_id/{id}", put(hdr_app::remove_by_id))
        .route("/remove", put(hdr_app::remove))
        .route("/delete_by_id/{id}", delete(hdr_app::delete_by_id))
        .route("/delete", delete(hdr_app::delete))
        .route("/find_by_id/{id}", get(hdr_app::find_by_id))
        .route("/find_all", get(hdr_app::find_all))
}

/// 附件模块接口
fn adtion_api() -> Router {
    Router::new()
        .nest("/data", adtion_data_api()) // 附件数据模块
        .nest("/type", adtion_type_api()) // 附件类型模块
}

/// 附件数据模块接口
fn adtion_data_api() -> Router {
    Router::new()
        .route("/add", get(hdr_adtion::add))
        .route("/delete/{id}", delete(hdr_adtion::delete_by_id))
        .route("/delete", delete(hdr_adtion::delete))
        .route("/find_all", get(hdr_adtion::find_all))
        .route("/find_by_id/{id}", get(hdr_adtion::find_by_id))
        .route("/find_by_guid/{guid}", get(hdr_adtion::find_by_guid))
}

/// 附件类型模块接口
fn adtion_type_api() -> Router {
    Router::new()
        .route("/add", post(hdr_adtion_ex::add))
        .route("/edit", put(hdr_adtion_ex::edit))
        .route("/remove/{id}", put(hdr_adtion_ex::remove_by_id))
        .route("/remove", put(hdr_adtion_ex::remove))
        .route("/delete/{id}", delete(hdr_adtion_ex::delete_by_id))
        .route("/delete", delete(hdr_adtion_ex::delete))
        .route("/find_by_id/{id}", get(hdr_adtion_ex::find_by_id))
        .route("/find_all", get(hdr_adtion_ex::find_all))
}

/// 授权模块
fn auth_api() -> Router {
    Router::new()
        .nest("/app", app_auth_api()) // 应用授权模块
        .nest("/col", col_auth_api()) // 列级授权模块
        .nest("/row", row_auth_api()) // 行级授权模块
        .nest("/fn", fn_auth_api()) // 功能授权模块
        .nest("/api", fn_api_api()) // API授权模块
}

fn app_auth_api() -> Router {
    Router::new()
        .route("/delete", delete(hdr_app_auth::delete))
        .route("/find_all", get(hdr_app_auth::find_all))
}

fn col_auth_api() -> Router {
    Router::new()
        .route("/add", post(hdr_col_auth::add))
        .route("/edit", put(hdr_col_auth::edit))
        .route("/remove/{id}", put(hdr_col_auth::remove_by_id))
        .route("/remove", put(hdr_col_auth::remove))
        .route("/delete/{id}", delete(hdr_col_auth::delete_by_id))
        .route("/delete", delete(hdr_col_auth::delete))
        .route("/find_by_id/{id}", get(hdr_col_auth::find_by_id))
        .route("/find_all", get(hdr_col_auth::find_all))
}

fn row_auth_api() -> Router {
    Router::new()
        .route("/add", post(hdr_row_auth::add))
        .route("/edit", put(hdr_row_auth::edit))
        .route("/remove/{id}", put(hdr_row_auth::remove_by_id))
        .route("/remove", put(hdr_row_auth::remove))
        .route("/delete/{id}", delete(hdr_row_auth::delete_by_id))
        .route("/delete", delete(hdr_row_auth::delete))
        .route("/find_by_id/{id}", get(hdr_row_auth::find_by_id))
        .route("/find_all", get(hdr_row_auth::find_all))
}

fn fn_auth_api() -> Router {
    Router::new()
        .route("/add", post(hdr_user_auth::add))
        .route("/edit", put(hdr_user_auth::edit))
        .route("/remove/{id}", put(hdr_user_auth::remove_by_id))
        .route("/remove", put(hdr_user_auth::remove))
        .route("/delete/{id}", delete(hdr_user_auth::delete_by_id))
        .route("/delete", delete(hdr_user_auth::delete))
        .route("/find_by_id/{id}", get(hdr_user_auth::find_by_id))
        .route("/find_all", get(hdr_user_auth::find_all))
}

fn fn_api_api() -> Router {
    Router::new().route("/find_all", get(hdr_user_api::find_all))
}

/// 菜单模块接口
fn menu_api() -> Router {
    Router::new()
        .route("/add", post(hdr_menu::add))
        .route("/edit", put(hdr_menu::edit))
        .route("/remove_by_id/{id}", put(hdr_menu::remove_by_id))
        .route("/remove", put(hdr_menu::remove))
        .route("/delete_by_id/{id}", delete(hdr_menu::delete_by_id))
        .route("/delete", delete(hdr_menu::delete))
        .route("/find_by_id/{id}", get(hdr_menu::find_by_id))
        .route("/find_all", get(hdr_menu::find_all))
        .route("/find_menu_all", get(hdr_menu::find_menu_all))
        .route("/find_fn_all", get(hdr_menu::find_fn_all))
        .route("/find_menu_by_uid", get(hdr_menu::find_menu_by_uid))
        .route("/find_fn_by_uid", get(hdr_menu::find_fn_by_uid))
        .route("/find_by_guid/{guid}", get(hdr_menu::find_by_guid))
        .route("/find_by_pguid/{pguid}", get(hdr_menu::find_by_pguid))
        .route("/find_menu_fn/{mcode}", get(hdr_menu::find_menu_fn))
}

fn md_api() -> Router {
    Router::new()
        .route("/add", post(hdr_master_detail::add)) // 新建用户
        .route("/edit", put(hdr_master_detail::edit)) // 编辑用户
        .route("/remove_by_id/{id}", put(hdr_master_detail::remove_by_id))
        .route("/remove", put(hdr_master_detail::remove)) //逻辑删除用户
        .route(
            "/delete_by_id/{id}",
            delete(hdr_master_detail::delete_by_id),
        ) // 硬删除用户
        .route("/delete", delete(hdr_master_detail::delete)) // 硬删除用户
        .route("/find_by_id/{id}", get(hdr_master_detail::find_by_id)) // 按id获取用户
        .route("/find_all", get(hdr_master_detail::find_all)) // 查询所有
}

/// 用户模块接口
fn user_api() -> Router {
    Router::new()
        .route("/add", post(hdr_user::add)) // 新建用户
        .route("/edit", put(hdr_user::edit)) // 编辑用户
        .route("/remove_by_id/{id}", put(hdr_user::remove_by_id))
        .route("/remove", put(hdr_user::remove)) //逻辑删除用户
        .route("/delete_by_id/{id}", delete(hdr_user::delete_by_id)) // 硬删除用户
        .route("/delete", delete(hdr_user::delete)) // 硬删除用户
        .route("/find_by_id/{id}", get(hdr_user::find_by_id)) // 按id获取用户
        .route("/find_all", get(hdr_user::find_all)) // 查询所有
}

/// 日志模块接口
fn log_api() -> Router {
    Router::new()
        .nest("/login", log_login_api())
        .nest("/oper", log_oper_api())
}

fn tree_api() -> Router {
    Router::new()
        .route("/add", post(hdr_tree::add)) // 新建用户
        .route("/edit", put(hdr_tree::edit)) // 编辑用户
        .route("/remove_by_id/{id}", put(hdr_tree::remove_by_id))
        .route("/remove", put(hdr_tree::remove)) //逻辑删除用户
        .route("/delete_by_id/{id}", delete(hdr_tree::delete_by_id)) // 硬删除用户
        .route("/delete", delete(hdr_tree::delete)) // 硬删除用户
        .route("/find_by_id/{id}", get(hdr_tree::find_by_id)) // 按id获取用户
        .route("/find_all", get(hdr_tree::find_all)) // 查询所有
        .route(
            "/find_by_mcode_uid/{mcode}",
            get(hdr_tree::find_by_mcode_uid),
        )
}

/// 登录日志接口
fn log_login_api() -> Router {
    Router::new()
        .route("/find_all", get(hdr_login_log::find_all))
        .route("/delete", delete(hdr_login_log::delete))
        .route("/clean", delete(hdr_login_log::clean))
}

fn log_oper_api() -> Router {
    Router::new().route("/find_all", get(hdr_oper_log::find_all))
}

fn online_api() -> Router {
    Router::new()
        .route("/find_all", get(hdr_online::find_all))
        .route("/delete", delete(hdr_online::delete))
}

/// 部门模块接口
fn org_api() -> Router {
    Router::new()
        .route("/add", post(hdr_org::add))
        .route("/edit", put(hdr_org::edit))
        .route("/remove_by_id/{id}", put(hdr_org::remove_by_id))
        .route("/remove", put(hdr_org::remove))
        .route("/delete_by_id/{id}", delete(hdr_org::delete_by_id))
        .route("/delete", delete(hdr_org::delete))
        .route("/find_by_id/{id}", get(hdr_org::find_by_id))
        .route("/find_all", get(hdr_org::find_all))
}

/// 角色模块接口
fn role_api() -> Router {
    Router::new()
        .route("/add", post(hdr_role::add))
        .route("/edit", put(hdr_role::edit))
        .route("/remove_by_id/{id}", put(hdr_role::remove_by_id))
        .route("/remove", put(hdr_role::remove))
        .route("/delete_by_id/{id}", delete(hdr_role::delete_by_id))
        .route("/delete", delete(hdr_role::delete))
        .route("/find_by_id/{id}", get(hdr_role::find_by_id))
        .route("/find_all", get(hdr_role::find_all))
}

/// 岗位模块接口
fn post_api() -> Router {
    Router::new()
        .route("/add", post(hdr_post::add))
        .route("/edit", put(hdr_post::edit))
        .route("/remove_by_id/{id}", put(hdr_post::remove_by_id))
        .route("/remove", put(hdr_post::remove))
        .route("/delete_by_id/{id}", delete(hdr_post::delete_by_id))
        .route("/delete", delete(hdr_post::delete))
        .route("/find_by_id/{id}", get(hdr_post::find_by_id))
        .route("/find_all", get(hdr_post::find_all))
}

/// 职务模块接口
fn duty_api() -> Router {
    Router::new()
        .route("/add", post(hdr_duty::add))
        .route("/edit", put(hdr_duty::edit))
        .route("/remove_by_id/{id}", put(hdr_duty::remove_by_id))
        .route("/remove", put(hdr_duty::remove))
        .route("/delete_by_id/{id}", delete(hdr_duty::delete_by_id))
        .route("/delete", delete(hdr_duty::delete))
        .route("/find_by_id/{id}", get(hdr_duty::find_by_id))
        .route("/find_all", get(hdr_duty::find_all))
}

/// 字典模块接口
fn dict_api() -> Router {
    Router::new()
        .nest("/data", dict_data_api())
        .nest("/type", dict_type_api())
}

/// 字典数据接口
fn dict_type_api() -> Router {
    Router::new()
        .route("/add", post(hdr_dict::add))
        .route("/edit", put(hdr_dict::edit))
        .route("/remove_by_id/{id}", put(hdr_dict::remove_by_id))
        .route("/remove", put(hdr_dict::remove))
        .route("/delete_by_id/{id}", delete(hdr_dict::delete_by_id))
        .route("/delete", delete(hdr_dict::delete))
        .route("/find_by_id/{id}", get(hdr_dict::find_by_id))
        .route("/find_all", get(hdr_dict::find_all))
}

/// 字典类型接口
fn dict_data_api() -> Router {
    Router::new()
        .route("/add", post(hdr_dict_data::add))
        .route("/edit", put(hdr_dict_data::edit))
        .route("/remove_by_id/{id}", put(hdr_dict_data::remove_by_id))
        .route("/remove", put(hdr_dict_data::remove))
        .route("/delete_by_id/{id}", delete(hdr_dict_data::delete_by_id))
        .route("/delete", delete(hdr_dict_data::delete))
        .route("/find_by_id/{id}", get(hdr_dict_data::find_by_id))
        .route("/find_all", get(hdr_dict_data::find_all))
}

fn dql_api() -> Router {
    Router::new()
        .route("/add", post(hdr_dql::add))
        .route("/edit", put(hdr_dql::edit))
        .route("/remove_by_id/{id}", put(hdr_dql::remove_by_id))
        .route("/remove", put(hdr_dql::remove))
        .route("/delete_by_id/{id}", delete(hdr_dql::delete_by_id))
        .route("/delete", delete(hdr_dql::delete))
        .route("/find_by_id/{id}", get(hdr_dql::find_by_id))
        .route("/find_by_sign/{sign}", get(hdr_dql::find_by_sign))
        .route("/find_all", get(hdr_dql::find_all))
}
