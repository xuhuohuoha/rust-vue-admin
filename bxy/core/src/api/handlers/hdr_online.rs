//!
//!  用户在线接口
//!

use axum::Json;
use axum_extra::extract::Query;
use byz_entity::core::OnlineModel;

use crate::{
    api::{
        request::{req::DeleteReq, req_online::OnlineSearchReq},
        response::res::ApiRes,
    },
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::srv_online,
};

/// 删除
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> ApiRes<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = srv_online::delete(db, delete_req).await;
    match res {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}

pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<OnlineSearchReq>,
) -> ApiRes<PageData<OnlineModel>> {
    match srv_online::find_all(page_params, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
