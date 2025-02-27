use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::MenuModel;

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_menu::{MenuReq, MenuSearchReq, UserMenuReq},
        },
        response::{res::ApiRes, res_menu::MenuTreeRes},
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_menu, utils::jwt::Claims},
};

/// 新增菜单
pub async fn add(user: Claims, Json(req): Json<MenuReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 编辑菜单
pub async fn edit(user: Claims, Json(req): Json<MenuReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除菜单
pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除菜单
pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::remove(db, req, user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除菜单
pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除菜单
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::delete(db, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询菜单（指定 ID）
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<MenuModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::find_by_id(db, id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询菜单（指定 ID）
pub async fn find_by_guid(Path(guid): Path<String>) -> ApiRes<MenuModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::find_by_guid(db, &guid).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询菜单（指定 ID）
pub async fn find_by_pguid(Path(pguid): Path<String>) -> ApiRes<MenuModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::find_by_pguid(db, &pguid).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询菜单功能（指定 菜单代码）
pub async fn find_menu_fn(Path(mcode): Path<String>) -> ApiRes<Vec<MenuModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::find_menu_fn(db, &mcode).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 菜单查询列表
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<MenuSearchReq>,
) -> ApiRes<PageData<MenuModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_menu_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<MenuSearchReq>,
) -> ApiRes<PageData<MenuModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::find_menu_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_fn_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<MenuSearchReq>,
) -> ApiRes<PageData<MenuModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::find_fn_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 用户权限菜单（指定应用、指定用户）
pub async fn find_menu_by_uid(
    user: Claims,
    Query(req): Query<UserMenuReq>,
) -> ApiRes<Vec<MenuTreeRes>> {
    let db = DB.get_or_init(db_conn).await;
    let mut u_id = user.u_id;
    if !req.u_id.is_empty() {
        u_id = req.u_id;
    }
    let r = UserMenuReq {
        app_code: req.app_code,
        mcode: req.mcode,
        u_id,
    };
    match srv_menu::find_menu_by_uid(db, r).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 菜单功能（指定菜单、指定用户）
pub async fn find_fn_by_uid(
    user: Claims,
    Query(req): Query<UserMenuReq>,
) -> ApiRes<Vec<MenuModel>> {
    let db = DB.get_or_init(db_conn).await;
    let mut u_id = user.u_id;
    if !req.u_id.is_empty() {
        u_id = req.u_id;
    }
    let r = UserMenuReq {
        app_code: req.app_code,
        mcode: req.mcode,
        u_id,
    };
    match srv_menu::find_fn_by_uid(db, r).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_app_root_menu(Query(req): Query<UserMenuReq>) -> ApiRes<Vec<MenuModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_menu::find_app_root_menu(db, &req.app_code).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_app_menu_children_tree(Query(req): Query<UserMenuReq>) -> ApiRes<MenuTreeRes> {
    let db = DB.get_or_init(db_conn).await;
    let menu_tree_res = srv_menu::find_app_menu_children_tree(db, &req.app_code, &req.mcode).await;
    ApiRes::with_data(menu_tree_res)
}
