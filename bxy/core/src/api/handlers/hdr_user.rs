use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::UserModel;
use hyper::HeaderMap;

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_login::LoginReq,
            req_user::{ResetPasswordReq, UpdatePasswordReq, UserReq, UserSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{
        srv_online, srv_user,
        utils::jwt::{AuthBody, Claims},
    },
};

/// 新建用户
#[utoipa::path(
    method(post),
    path = "/api/v1/core/user/add",
    tag = crate::api::v1::USER_TAG,
    request_body = UserReq,
    responses(
        (status = 200, description = "新建用户成功", body = ApiRes<String>),
        (status = 401, description = "未授权", body = ApiRes<String>),
    ),
)]
pub async fn add(user: Claims, Json(req): Json<UserReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 编辑用户
#[utoipa::path(
    method(put),
    path = "/api/v1/core/user/edit",
    tag = crate::api::v1::USER_TAG,
    request_body = UserReq,
    responses(
        (status = 200, description = "编辑用户成功", body = ApiRes<String>),
        (status = 401, description = "未授权", body = ApiRes<String>),
    ),
)]
pub async fn edit(user: Claims, Json(req): Json<UserReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 软删除用户
#[utoipa::path(
    method(put),
    path = "/api/v1/core/user/remove_by_id/:id",
    tag = crate::api::v1::USER_TAG,
    responses(
        (status = 200, description = "Remove success", body = ApiRes<bool>),
        (status = 401, description = "未授权", body = ApiRes<bool>),
    ),
)]
pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    method(put),
    path = "/api/v1/core/user/remove",
    tag = crate::api::v1::USER_TAG,
    responses(
        (status = 200, description = "Remove success", body = ApiRes<bool>),
        (status = 401, description = "未授权", body = ApiRes<bool>),
    ),
)]
pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除用户
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/user/delete_by_id/:id",
    tag = crate::api::v1::USER_TAG,
    request_body = DeleteReq,
    responses(
        (status = 200, description = "Delete success", body = ApiRes<String>),
        (status = 401, description = "Unauthorized", body = ApiRes<String>),
    ),
)]
pub async fn delete_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::delete_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 硬删除用户
#[utoipa::path(
    method(delete),
    path = "/api/v1/core/user/delete",
    tag = crate::api::v1::USER_TAG,
    request_body = DeleteReq,
    responses(
        (status = 200, description = "Delete success", body = ApiRes<String>),
        (status = 401, description = "Unauthorized", body = ApiRes<String>),
    ),
)]
pub async fn delete(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::delete(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询用户（根据 ID）
#[utoipa::path(
    method(get),
    path = "/api/v1/core/user/find_by_id/:id",
    tag = crate::api::v1::USER_TAG,
    responses(
        (status = 200, description = "Remove success", body = ApiRes<String>),
        (status = 401, description = "Unauthorized", body = ApiRes<String>),
    ),
)]
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<UserModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询用户（根据 用户ID）
pub async fn find_by_uid(Path(u_id): Path<String>) -> ApiRes<UserModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::find_by_uid(db, &u_id).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询用户（根据 用户账号）
pub async fn find_by_ucode(Path(usercode): Path<String>) -> ApiRes<UserModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::find_by_ucode(db, &usercode).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 查询所有用户
#[utoipa::path(
    method(get),
    path = "/api/v1/core/user/find_all",
    tag = crate::api::v1::USER_TAG,
    responses(
        (status = 200, description = "Remove success", body = ApiRes<String>),
        (status = 401, description = "Unauthorized", body = ApiRes<String>),
    ),
)]
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<UserSearchReq>,
) -> ApiRes<PageData<UserModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 修改密码
pub async fn change_password(Json(req): Json<UpdatePasswordReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::update_password(db, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 重置密码
pub async fn reset_password(Json(req): Json<ResetPasswordReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::reset_password(db, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 退出
#[utoipa::path(
    method(get),
    path = "/api/v1/comm/logout",
    tag = crate::api::v1::LOGIN_TAG,
    responses(
        (status = OK, description = "退出成功", body = str, content_type = "text/plain")
    )
)]
pub async fn logout(user: Claims) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = srv_online::logout(db, &user.token_id).await;
    match res {
        Ok(x) => ApiRes::with_msg(&x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

/// 登录
#[utoipa::path(
    method(post),
    path = "/api/v1/comm/login",
    tag = crate::api::v1::LOGIN_TAG,
    responses(
        (status = OK, description = "登录成功", body = str, content_type = "application/json")
    )
)]
pub async fn login(header: HeaderMap, Json(req): Json<LoginReq>) -> ApiRes<AuthBody> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user::login(db, req, header).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
