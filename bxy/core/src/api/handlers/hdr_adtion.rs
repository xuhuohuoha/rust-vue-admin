//! 附件数据业务接口
//!

use crate::{
    api::{
        request::{
            req::DeleteReq,
            req_adtion::{AdtionReq, AdtionSearchReq},
        },
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::{srv_adtion, utils::jwt::Claims},
};
use axum::{extract::Path, Json};
use axum_extra::extract::Query;
use byz_entity::core::AdtionModel;

#[utoipa::path(
    method(post),
    path = "/api/v1/core/adtion/data/add",
    tag = crate::api::v1::ADTION_TAG,
    request_body = AdtionReq,
    responses(
        (status = 200, description = "新增附件数据成功", body = ApiRes<String>),
    ),
)]
/// 新增附件
pub async fn add(user: Claims, Json(req): Json<AdtionReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion::add(db, req, &user.u_id).await {
        Ok(x) => ApiRes::with_data_msg(x, "新增附件数据成功."),
        Err(e) => {
            ApiRes::with_err(&(format!("新增附件数据时发生错误，错误原因为：{}", &e.to_string())))
        }
    }
}

#[utoipa::path(
    method(delete),
    path = "/api/v1/core/adtion/data/delete/{id}",
    tag = crate::api::v1::ADTION_TAG,
    responses(
        (status = 200, description = "删除附件数据成功", body = ApiRes<String>),
    ),
)]
/// 硬删除附件（根据 ID）
pub async fn delete_by_id(Path(id): Path<String>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion::delete_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "删除指定附件数据成功."),
        Err(e) => ApiRes::with_err(
            &(format!(
                "硬删除指定附件数据时发生错误，错误原因为：{}",
                &e.to_string()
            )),
        ),
    }
}

#[utoipa::path(
    method(delete),
    path = "/api/v1/core/adtion/data/delete",
    tag = crate::api::v1::ADTION_TAG,
    request_body = DeleteReq,
    responses(
        (status = 200, description = "批量删除附件数据成功", body = ApiRes<String>),
    ),
)]
/// 硬删除附件
pub async fn delete(Json(req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion::delete(db, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "批量删除附件数据成功."),
        Err(e) => ApiRes::with_err(
            &(format!(
                "批量硬删除附件数据时发生错误，错误原因为：{}",
                &e.to_string()
            )),
        ),
    }
}

#[utoipa::path(
    method(get),
    path = "/api/v1/core/adtion/data/find_by_id/{id}",
    tag = crate::api::v1::ADTION_TAG,
    responses(
        (status = 200, description = "获取附件数据成功", body = ApiRes<String>),
    ),
)]
/// 查询附件（根据 ID）
pub async fn find_by_id(Path(id): Path<String>) -> ApiRes<AdtionModel> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion::find_by_id(db, &id).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定附件数据成功."),
        Err(e) => ApiRes::with_err(
            &(format!("获取指定附件数据时发生错误，错误原因为：{}", &e.to_string())),
        ),
    }
}

#[utoipa::path(
    method(get),
    path = "/api/v1/core/adtion/data/find_by_guid/{guid}",
    tag = crate::api::v1::ADTION_TAG,
    responses(
        (status = 200, description = "获取指定业务所有附件数据成功", body = ApiRes<String>),
    ),
)]
/// 查询附件（根据 业务主键）
pub async fn find_by_guid(Path(guid): Path<String>) -> ApiRes<Vec<AdtionModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion::find_by_guid(db, &guid).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取指定业务所有附件数据成功."),
        Err(e) => ApiRes::with_err(
            &(format!(
                "获取指定业务所有附件数据时发生错误，错误原因为：{}",
                &e.to_string()
            )),
        ),
    }
}
#[utoipa::path(
    method(get),
    path = "/api/v1/core/adtion/data/find_all",
    tag = crate::api::v1::APP_TAG,
    responses(
        (status = 200, description = "获取附件数据分页列表成功", body = ApiRes<String>),
    ),
)]
/// 查询附件类别（分页）
pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<AdtionSearchReq>,
) -> ApiRes<PageData<AdtionModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_adtion::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data_msg(x, "获取附件数据分页列表成功."),
        Err(e) => ApiRes::with_err(
            &(format!(
                "获取附件数据分页列表时发生错误，错误原因为：{}",
                &e.to_string()
            )),
        ),
    }
}
