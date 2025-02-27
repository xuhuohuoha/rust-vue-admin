use axum_extra::extract::Query;
use byz_entity::core::UserApiModel;

use crate::{
    api::{request::req_user::UserApiSearchReq, response::res::ApiRes},
    db::{db_conn, DB},
    model::app_structs::{PageData, PageParams},
    serve::srv_user_api,
};

pub async fn find_all(
    Query(page_params): Query<PageParams>,
    Query(req): Query<UserApiSearchReq>,
) -> ApiRes<PageData<UserApiModel>> {
    let db = DB.get_or_init(db_conn).await;
    match srv_user_api::find_all(db, page_params, req).await {
        Ok(x) => ApiRes::with_data(x),
        Err(e) => ApiRes::with_err(&e.to_string()),
    }
}
