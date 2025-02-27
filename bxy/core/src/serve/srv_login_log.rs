//!
//! 登录日志接口业务实现
//!
//! 新增登录日志
//! 清空登录日志
//! 硬删除日志
//! 获取日志列表
//!

use anyhow::{anyhow, Result};
use byz_entity::core::{sys_login_log, LoginLogEntity, LoginLogModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Table, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set, TransactionTrait,
};

use crate::api::request::req::DeleteReq;
use crate::api::request::req_login::LoginLogSearchReq;
use crate::model::app_structs::{PageData, PageParams, ToPageData};
use crate::{
    db::{db_conn, DB},
    model::app_structs::ClientInfo,
};

/// 新增登录日志
pub async fn add(req: ClientInfo, u_id: String, msg: String, status: String, module: String) {
    let db = DB.get_or_init(db_conn).await;
    let id = scru128::new_string().to_uppercase();
    let now = Local::now().naive_local();
    let new_module = if module.is_empty() {
        "系统后台".to_string()
    } else {
        module.to_string()
    };
    let active_model = sys_login_log::ActiveModel {
        id: Set(id),
        u_id: Set(u_id),
        net: Set(req.net.net_work),
        ip: Set(req.net.ip),
        login_location: Set(req.net.location),
        browser: Set(req.ua.browser),
        os: Set(req.ua.os),
        device: Set(req.ua.device),
        status: Set(status),
        msg: Set(msg),
        login_time: Set(now),
        module: Set(new_module),
    };
    let txn = db.begin().await.expect("begin txn error");
    let _ = LoginLogEntity::insert(active_model)
        .exec(db)
        .await
        .expect("insert error");
    txn.commit().await.expect("commit txn error");
}

/// 清空登录日志
pub async fn clean() -> Result<String> {
    let db = DB.get_or_init(db_conn).await;
    let stmt = Table::truncate().table(sys_login_log::Entity).to_owned();
    let db_backend = db.get_database_backend();
    db.execute(db_backend.build(&stmt)).await?;
    Ok("登录日志已清空".to_string())
}

/// 硬删除日志
pub async fn delete(req: DeleteReq) -> Result<String> {
    let db = DB.get_or_init(db_conn).await;
    match LoginLogEntity::delete_many()
        .filter(sys_login_log::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败，数据不存在",)),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 获取日志列表
pub async fn find_all(
    page_params: PageParams,
    req: LoginLogSearchReq,
) -> Result<PageData<LoginLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(10);

    // 条件处理
    let mut q = LoginLogEntity::find();

    if let Some(x) = req.ip {
        if !x.is_empty() {
            q = q.filter(sys_login_log::Column::Ip.contains(&x));
        }
    }

    if let Some(x) = req.u_id {
        if !x.is_empty() {
            q = q.filter(sys_login_log::Column::UId.contains(&x));
        }
    }

    if let Some(x) = req.status {
        if !x.is_empty() {
            q = q.filter(sys_login_log::Column::Status.eq(&x));
        }
    }

    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + "00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            q = q.filter(sys_login_log::Column::LoginTime.gte(t));
        }
    }

    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + "23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            q = q.filter(sys_login_log::Column::LoginTime.lte(t));
        }
    }

    // 分页
    let paginator = if let (Some(column), Some(order)) = (req.order_by_column, req.is_asc) {
        match (column.as_str(), order.as_str()) {
            ("login_name", "asc") => q.order_by_asc(sys_login_log::Column::UId),
            ("login_name", "desc") => q.order_by_desc(sys_login_log::Column::UId),
            ("login_time", "asc") => q.order_by_asc(sys_login_log::Column::LoginTime),
            ("login_time", "desc") => q.order_by_desc(sys_login_log::Column::LoginTime),
            (_, _) => q.order_by_desc(sys_login_log::Column::LoginTime),
        }
    } else {
        q.order_by_desc(sys_login_log::Column::LoginTime)
    }
    .paginate(db, page_size);

    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
