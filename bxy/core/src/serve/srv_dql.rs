use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use byz_entity::core::{sys_dql, DqlEntity, DqlModel};

use crate::{
    api::request::{
        req::DeleteReq,
        req_dql::{DqlReq, DqlSearchReq},
    },
    model::{
        app_structs::{PageData, PageParams, ToPageData},
        dict_structs::DataStatus,
    },
};

/// 新增动态脚本
pub async fn add(db: &DbConn, req: DqlReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let now: NaiveDateTime = Local::now().naive_local();

    sys_dql::ActiveModel {
        id: Set(id.clone()),
        created_at: Set(now),
        create_by: Set(op_user.to_string()),
        version: Set(1),
        ord: Set(req.ord),
        status: Set(req.status),
        remark: Set(req.remark),
        mcode: Set(req.mcode),
        sign: Set(req.sign),
        dql: Set(req.dql),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

/// 编辑动态脚本
pub async fn edit(db: &DbConn, req: DqlReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now = Local::now().naive_local();
    DqlEntity::update_many()
        .col_expr(sys_dql::Column::Mcode, Expr::value(req.mcode))
        .col_expr(sys_dql::Column::Sign, Expr::value(req.sign))
        .col_expr(sys_dql::Column::Dql, Expr::value(req.dql))
        .col_expr(sys_dql::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_dql::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_dql::Column::Status, Expr::value(req.status))
        .col_expr(sys_dql::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_dql::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_dql::Column::Version,
            Expr::value(Expr::col(sys_dql::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_dql::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok(id)
}

/// 软删除动态脚本（指定 ID）
pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    DqlEntity::update_many()
        .col_expr(
            sys_dql::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_dql::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_dql::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_dql::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

/// 软删除动态脚本（批量 ID）
pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    DqlEntity::update_many()
        .col_expr(sys_dql::Column::Status, Expr::value("2".to_string()))
        .col_expr(
            sys_dql::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_dql::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_dql::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

/// 硬删除动态脚本（指定 ID）
pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match DqlEntity::delete_many()
        .filter(sys_dql::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

/// 硬删除动态脚本（批量 ID）
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match DqlEntity::delete_many()
        .filter(sys_dql::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 查询动态脚本（指定 ID）
pub async fn find_by_id(db: &DbConn, id: &str) -> Result<DqlModel> {
    match DqlEntity::find()
        .filter(sys_dql::Column::DeletedAt.is_null())
        .filter(sys_dql::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("动态脚本不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询动态脚本（指定 sign）
pub async fn find_by_sign(db: &DbConn, sign: &str) -> Result<DqlModel> {
    match DqlEntity::find()
        .filter(sys_dql::Column::DeletedAt.is_null())
        .filter(sys_dql::Column::Sign.eq(sign))
        .one(db)
        .await?
    {
        None => Err(anyhow!("动态脚本不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询动态脚本（分页）
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: DqlSearchReq,
) -> Result<PageData<DqlModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = DqlEntity::find();

    // 所属菜单
    if let Some(x) = req.mcode {
        if !x.is_empty() {
            q = q.filter(sys_dql::Column::Mcode.eq(&x))
        }
    }
    // 脚本名称
    if let Some(x) = req.sign {
        if !x.is_empty() {
            q = q.filter(sys_dql::Column::Sign.contains(&x))
        }
    }

    let paginator = q.order_by_asc(sys_dql::Column::Ord).paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
