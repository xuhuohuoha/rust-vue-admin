use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};

use byz_entity::core::{sys_update_log, UpdateLogEntity, UpdateLogModel};

use crate::{
    api::request::{
        req::DeleteReq,
        req_update_log::{UpdateLogReq, UpdateLogSearchReq},
    },
    model::app_structs::{PageData, PageParams, ToPageData},
};

pub async fn add(db: &DbConn, req: UpdateLogReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let now: NaiveDateTime = Local::now().naive_local();

    sys_update_log::ActiveModel {
        id: Set(id.clone()),
        created_at: Set(now),
        create_by: Set(op_user.to_string()),
        app_version: Set(req.app_version),
        backend_version: Set(req.backend_version),
        title: Set(req.title),
        content: Set(req.content),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

pub async fn edit(db: &DbConn, req: UpdateLogReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now = Local::now().naive_local();
    UpdateLogEntity::update_many()
        .col_expr(
            sys_update_log::Column::AppVersion,
            Expr::value(req.app_version),
        )
        .col_expr(
            sys_update_log::Column::BackendVersion,
            Expr::value(req.backend_version),
        )
        .col_expr(sys_update_log::Column::Title, Expr::value(req.title))
        .col_expr(sys_update_log::Column::Content, Expr::value(req.content))
        .col_expr(sys_update_log::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_update_log::Column::UpdateBy, Expr::value(op_user))
        .filter(sys_update_log::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok(id)
}

pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match UpdateLogEntity::delete_many()
        .filter(sys_update_log::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match UpdateLogEntity::delete_many()
        .filter(sys_update_log::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<UpdateLogModel> {
    match UpdateLogEntity::find()
        .filter(sys_update_log::Column::DeletedAt.is_null())
        .filter(sys_update_log::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("更新日志不存在.")),
        Some(u) => Ok(u),
    }
}

pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: UpdateLogSearchReq,
) -> Result<PageData<UpdateLogModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = UpdateLogEntity::find();

    // 查询条件处理
    if let Some(x) = req.title {
        q = q.filter(sys_update_log::Column::Title.contains(&x))
    }

    if let Some(x) = req.content {
        q = q.filter(sys_update_log::Column::Content.eq(&x))
    }

    let paginator = q.paginate(db, page_size);

    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
