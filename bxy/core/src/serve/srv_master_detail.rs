use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use byz_entity::core::{sys_master_detail, MdEntity, MdModel};

use crate::{
    api::request::{
        req::DeleteReq,
        req_master_detail::{MdReq, MdSearchReq},
    },
    model::{
        app_structs::{PageData, PageParams, ToPageData},
        dict_structs::DataStatus,
    },
};

pub async fn add(db: &DbConn, req: MdReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let now: NaiveDateTime = Local::now().naive_local();

    sys_master_detail::ActiveModel {
        id: Set(id.clone()),
        created_at: Set(now),
        create_by: Set(op_user.to_string()),
        version: Set(1),
        ord: Set(req.ord),
        status: Set(req.status),
        remark: Set(req.remark),
        mcode: Set(req.mcode),
        dcode: Set(req.dcode),
        m_fields: Set(req.m_fields),
        d_fields: Set(req.d_fields),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

pub async fn edit(db: &DbConn, req: MdReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now = Local::now().naive_local();
    MdEntity::update_many()
        .col_expr(sys_master_detail::Column::Mcode, Expr::value(req.mcode))
        .col_expr(sys_master_detail::Column::Dcode, Expr::value(req.dcode))
        .col_expr(
            sys_master_detail::Column::MFields,
            Expr::value(req.m_fields),
        )
        .col_expr(
            sys_master_detail::Column::DFields,
            Expr::value(req.d_fields),
        )
        .col_expr(sys_master_detail::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_master_detail::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_master_detail::Column::Status, Expr::value(req.status))
        .col_expr(sys_master_detail::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_master_detail::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_master_detail::Column::Version,
            Expr::value(Expr::col(sys_master_detail::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_master_detail::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok(id)
}

pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    MdEntity::update_many()
        .col_expr(
            sys_master_detail::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_master_detail::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_master_detail::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_master_detail::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    MdEntity::update_many()
        .col_expr(
            sys_master_detail::Column::Status,
            Expr::value("2".to_string()),
        )
        .col_expr(
            sys_master_detail::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_master_detail::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_master_detail::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match MdEntity::delete_many()
        .filter(sys_master_detail::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match MdEntity::delete_many()
        .filter(sys_master_detail::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<MdModel> {
    match MdEntity::find()
        .filter(sys_master_detail::Column::DeletedAt.is_null())
        .filter(sys_master_detail::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("动态脚本不存在.")),
        Some(u) => Ok(u),
    }
}

pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: MdSearchReq,
) -> Result<PageData<MdModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = MdEntity::find();

    // 查询条件处理
    if let Some(x) = req.mcode {
        if !x.is_empty() {
            q = q.filter(sys_master_detail::Column::Mcode.eq(&x))
        }
    }
    if let Some(x) = req.dcode {
        if !x.is_empty() {
            q = q.filter(sys_master_detail::Column::Dcode.eq(&x))
        }
    }
    let paginator = q
        .order_by_asc(sys_master_detail::Column::Ord)
        .paginate(db, page_size);

    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
