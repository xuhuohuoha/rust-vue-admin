//! 附件类别接口
//!
//! # 描述
//! 提供对附件类别进行相关操作的接口
//!
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_adtion::{AdtionExReq, AdtionExSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_adtion_ex, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::AdtionExModel;

#[utoipa::path(
    method(post),
    path = "/api/v1/core/adtion/type/add",
    tag = crate::api::v1::ADTION_TAG,
    request_body = AdtionExReq,
    responses(
        (status = 200, description = "新增附件类别成功.", body = ApiRes<String>),
        (status = 500, description = "新增附件类别时发生错误.", body = ApiRes<String>)
    ),
)]
/// 新增附件类别
pub async fn add(user: Claims, Json(req): Json<AdtionExReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion_ex::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增附件类别成功."),
        Err(e) => {
            ApiRes::with_err(&(format!("新建附件类别时发生错误，错误原因为：{}", &e.to_string())))
        }
    }
}

#[utoipa::path(
    method(put),
    path = "/api/v1/core/adtion/type/edit",
    tag = crate::api::v1::ADTION_TAG,
    request_body = AdtionExReq,
    responses(
        (status = 200, description = "编辑附件类别成功", body = ApiRes<String>),
        (status = 500, description = "编辑附件类别时发生错误.", body = ApiRes<String>)
    ),
)]
/// 编辑附件类别
pub async fn edit(user: Claims, Json(req): Json<AdtionExReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion_ex::edit(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "编辑附件类别成功."),
        Err(e) => {
            ApiRes::with_err(&(format!("编辑附件类别时发生错误，错误原因为：{}", &e.to_string())))
        }
    }
}

#[utoipa::path(
    method(put),
    path = "/api/v1/core/adtion/type/remove_by_id/{id}",
    tag = crate::api::v1::ADTION_TAG,
    responses(
        (status = 200, description = "删除附件类别成功", body = ApiRes<String>),
        (status = 500, description = "删除附件类别时发生错误.", body = ApiRes<String>),
    ),
)]
/// 软删除附件类别
pub async fn remove_by_id(user: Claims, Path(id): Path<String>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion_ex::remove_by_id(db, &id, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除指定附件类别成功."),
        Err(e) => ApiRes::with_err(
            &(format!(
                "软删除指定附件类别时发生错误，错误原因为：{}",
                &e.to_string()
            )),
        ),
    }
}

#[utoipa::path(
    method(put),
    path = "/api/v1/core/adtion/type/remove",
    tag = crate::api::v1::ADTION_TAG,
    request_body = DeleteReq,
    responses(
        (status = 200, description = "删除附件类别成功", body = ApiRes<String>),
        (status = 500, description = "删除附件类别时发生错误.", body = ApiRes<String>),
    ),
)]
/// 软删除附件类别
pub async fn remove(user: Claims, Json(req): Json<DeleteReq>) -> ApiRes<bool> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion_ex::remove(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "软删除附件类别成功."),
        Err(e) => ApiRes::with_err(
            &(format!(
                "批量软删除附件类别时发生错误，错误原因为：{}",
                &e.to_string()
            )),
        ),
    }
}

#[utoipa::path(
    method(delete),
    path = "/api/v1/core/adtion/type/delete_by_id/{id}",
    tag = crate::api::v1::ADTION_TAG,
    request_body = DeleteReq,
    responses(
        (status = 200, description = "删除指定附件类别成功", body = ApiRes<String>),
        (status = 500, description = "删除指定附件类别时发生错误.", body = ApiRes<String>),
    ),
)]
/// 硬删除附件类别
pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion_ex::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "硬删除指定附件类别成功."),
        Err(e) => ApiRes::with_err(
            &(format!(
                "硬删除指定附件类别时发生错误，错误原因为：{}",
                &e.to_string()
            )),
        ),
    }
}

#[utoipa::path(
    method(delete),
    path = "/api/v1/core/adtion/type/delete",
    tag = crate::api::v1::ADTION_TAG,
    request_body = DeleteReq,
    responses(
        (status = 200, description = "批量硬删除附件类别成功", body = ApiRes<String>),
        (status = 500, description = "批量删除附件类别时发生错误.", body = ApiRes<String>),
    ),
)]
/// 硬删除附件类别
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion_ex::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "批量硬删除附件类别成功."),
        Err(e) => ApiRes::with_err(
            &(format!(
                "批量硬删除附件类别时发生错误，错误原因为：{}",
                &e.to_string()
            )),
        ),
    }
}

#[utoipa::path(
    method(get),
    path = "/api/v1/core/adtion/type/find_by_id/{id}",
    tag = crate::api::v1::ADTION_TAG,
    responses(
        (status = 200, description = "查询指定附件类别成功", body = ApiRes<String>),
        (status = 500, description = "查询指定附件类别时发生错误.", body = ApiRes<String>),
    ),
)]
/// 查询附件类别（根据 ID）
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<AdtionExModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion_ex::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定附件类别成功."),
        Err(e) => ApiRes::with_err(
            &(format!("获取指定附件类别时发生错误，错误原因为：{}", &e.to_string())),
        ),
    }
}

#[utoipa::path(
    method(get),
    path = "/api/v1/core/adtion/type/find_all",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "获取附件类型分页列表成功", body = ApiRes<String>),
        (status = 500, description = "获取附件类型分页列表时发生错误.", body = ApiRes<String>),
    ),
)]
/// 查询附件类别（分页）
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<AdtionExSearchReq>,
) -> ApiRes<PageData<AdtionExModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion_ex::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取附件类型分页列表成功."),
        Err(e) => ApiRes::with_err(
            &(format!(
                "获取附件类型分页列表时发生错误，错误原因为：{}",
                &e.to_string()
            )),
        ),
    }
}
