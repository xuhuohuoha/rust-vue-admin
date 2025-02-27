//!
//! 在线用户接口业务实现
//!

use anyhow::{anyhow, Result};
use chrono::Local;
use byz_entity::core::{sys_online, OnlineEntity, OnlineModel};
use sea_orm::{
    sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use sea_orm::{PaginatorTrait, QueryOrder};

use crate::model::app_structs::ToPageData;
use crate::{
    api::request::{req::DeleteReq, req_online::OnlineSearchReq},
    db::{db_conn, DB},
    model::app_structs::{ClientInfo, PageData, PageParams},
};

use super::srv_user;

/// 新增登录日志
pub async fn add(req: ClientInfo, u_id: &str, token_id: &str, token_exp: i64) {
    let db = DB.get_or_init(db_conn).await;
    let uid = scru128::new_string().to_uppercase();
    let now = Local::now().naive_local();
    let user = srv_user::find_by_id(db, u_id).await.unwrap();
    let active_model = sys_online::ActiveModel {
        id: Set(uid.clone()),
        u_id: Set(u_id.to_string()),
        token_id: Set(token_id.to_string()),
        token_exp: Set(token_exp),
        uname: Set(user.uname),
        net: Set(req.net.net_work),
        ip: Set(req.net.ip),
        login_location: Set(req.net.location),
        browser: Set(req.ua.browser),
        os: Set(req.ua.os),
        device: Set(req.ua.device),
        login_time: Set(now),
    };
    let txn = db.begin().await.expect("begin txn error");
    let _ = OnlineEntity::insert(active_model)
        .exec(&txn)
        .await
        .expect("insert error");
    txn.commit().await.expect("commit txn error");
}

/// 更新 Token
pub async fn update_token(token_id: &str, token_exp: i64) -> Result<String> {
    let db = DB.get_or_init(db_conn).await;
    let txn = db.begin().await?;
    OnlineEntity::update_many()
        .col_expr(sys_online::Column::TokenExp, Expr::value(token_exp))
        .filter(sys_online::Column::TokenId.eq(token_id))
        .exec(&txn)
        .await?;
    txn.commit().await?;
    Ok("token更新成功".to_string())
}

/// 退出登录
pub async fn logout(db: &DatabaseConnection, token_id: &str) -> Result<String> {
    OnlineEntity::delete_many()
        .filter(sys_online::Column::TokenId.eq(token_id))
        .exec(db)
        .await?;
    Ok("成功退出登录".to_string())
}

/// 查看用户是否在线
pub async fn check_online(
    db: Option<&DatabaseConnection>,
    token_id: &str,
) -> (bool, Option<sys_online::Model>) {
    let db = match db {
        Some(x) => x,
        None => DB.get_or_init(db_conn).await,
    };
    let model = OnlineEntity::find()
        .filter(sys_online::Column::TokenId.eq(token_id))
        .one(db)
        .await
        .expect("查询失败");
    (model.is_some(), model)
}

/// 硬删除
pub async fn delete(db: &DatabaseConnection, req: DeleteReq) -> Result<String> {
    match OnlineEntity::delete_many()
        .filter(sys_online::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

pub async fn find_all(
    page_params: PageParams,
    req: OnlineSearchReq,
) -> Result<PageData<OnlineModel>> {
    let db = DB.get_or_init(db_conn).await;
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(10);

    // 条件处理
    let mut q = OnlineEntity::find();

    if let Some(x) = req.uname {
        if !x.is_empty() {
            q = q.filter(sys_online::Column::Uname.contains(&x))
        }
    }

    let paginator = q
        .order_by_desc(sys_online::Column::LoginTime)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
