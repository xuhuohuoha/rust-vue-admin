use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use byz_entity::core::{sys_datasource, DatasourceEntity, DatasourceModel};

use crate::{
    api::request::{
        req::DeleteReq,
        req_datasource::{DatasourceReq, DatasourceSearchReq},
    },
    model::{
        app_structs::{PageData, PageParams, ToPageData},
        dict_structs::DataStatus,
    },
};

pub async fn add(db: &DbConn, req: DatasourceReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let now: NaiveDateTime = Local::now().naive_local();

    sys_datasource::ActiveModel {
        id: Set(id.clone()),
        created_at: Set(now),
        create_by: Set(op_user.to_string()),
        version: Set(1),
        ord: Set(req.ord),
        status: Set(req.status),
        remark: Set(req.remark),
        db: Set(req.db),
        url: Set(req.url),
        maxc: Set(req.maxc),
        minc: Set(req.minc),
        conn_timeout: Set(req.conn_timeout),
        idle_timeout: Set(req.idle_timeout),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

pub async fn edit(db: &DbConn, req: DatasourceReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now = Local::now().naive_local();
    DatasourceEntity::update_many()
        .col_expr(sys_datasource::Column::Db, Expr::value(req.db))
        .col_expr(sys_datasource::Column::Url, Expr::value(req.url))
        .col_expr(sys_datasource::Column::Maxc, Expr::value(req.maxc))
        .col_expr(sys_datasource::Column::Minc, Expr::value(req.minc))
        .col_expr(
            sys_datasource::Column::ConnTimeout,
            Expr::value(req.conn_timeout),
        )
        .col_expr(
            sys_datasource::Column::IdleTimeout,
            Expr::value(req.idle_timeout),
        )
        .col_expr(sys_datasource::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_datasource::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_datasource::Column::Status, Expr::value(req.status))
        .col_expr(sys_datasource::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_datasource::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_datasource::Column::Version,
            Expr::value(Expr::col(sys_datasource::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_datasource::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok(id)
}

pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    DatasourceEntity::update_many()
        .col_expr(
            sys_datasource::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_datasource::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_datasource::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_datasource::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    DatasourceEntity::update_many()
        .col_expr(sys_datasource::Column::Status, Expr::value("2".to_string()))
        .col_expr(
            sys_datasource::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_datasource::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_datasource::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match DatasourceEntity::delete_many()
        .filter(sys_datasource::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match DatasourceEntity::delete_many()
        .filter(sys_datasource::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<DatasourceModel> {
    match DatasourceEntity::find()
        .filter(sys_datasource::Column::DeletedAt.is_null())
        .filter(sys_datasource::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("职务不存在.")),
        Some(u) => Ok(u),
    }
}

pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: DatasourceSearchReq,
) -> Result<PageData<DatasourceModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = DatasourceEntity::find();

    // 查询条件处理
    if let Some(x) = req.db {
        if !x.is_empty() {
            q = q.filter(sys_datasource::Column::Db.contains(&x));
        }
    }

    let paginator = q
        .order_by_asc(sys_datasource::Column::Ord)
        .paginate(db, page_size);

    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
