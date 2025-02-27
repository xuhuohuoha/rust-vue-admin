//! Adtion Ex Service
//!
//! # 描述
//! 附件服务
//!

use anyhow::{anyhow, Result};
use byz_entity::core::{sys_adtion_ex, AdtionExEntity, AdtionExModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set,
};
use sea_orm::{PaginatorTrait, QueryOrder};

use crate::api::request::req::DeleteReq;
use crate::api::request::req_adtion::{AdtionExReq, AdtionExSearchReq};
use crate::model::app_structs::{PageData, PageParams, ToPageData};
use crate::model::dict_structs::DataStatus;

/// 新增附件类型
pub async fn add(db: &DbConn, req: AdtionExReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let guid = if req.guid.is_empty() {
        scru128::new_string().to_uppercase()
    } else {
        req.guid.clone()
    };
    let pguid = if req.pguid.is_empty() {
        guid.clone()
    } else {
        req.pguid
    };
    let now: NaiveDateTime = Local::now().naive_local();
    sys_adtion_ex::ActiveModel {
        id: Set(id.clone()),
        guid: Set(guid),
        pguid: Set(pguid),
        create_by: Set(op_user.to_string()), // 创建人
        created_at: Set(now),                // 创建时间
        version: Set(1),                     // 版本号
        ord: Set(req.ord),
        status: Set(req.status),
        remark: Set(req.remark),
        mcode: Set(req.mcode),
        exname: Set(req.exname),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

/// 编辑附件类别
pub async fn edit(db: &DbConn, req: AdtionExReq, op_user: &str) -> Result<String> {
    let id = req.id;
    let now = Local::now().naive_local();
    AdtionExEntity::update_many()
        .col_expr(sys_adtion_ex::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_adtion_ex::Column::Status, Expr::value(req.status))
        .col_expr(sys_adtion_ex::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_adtion_ex::Column::Guid, Expr::value(req.guid))
        .col_expr(sys_adtion_ex::Column::Pguid, Expr::value(req.pguid))
        .col_expr(sys_adtion_ex::Column::Mcode, Expr::value(req.mcode))
        .col_expr(sys_adtion_ex::Column::Exname, Expr::value(req.exname))
        .col_expr(sys_adtion_ex::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_adtion_ex::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_adtion_ex::Column::Version,
            Expr::value(Expr::col(sys_adtion_ex::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_adtion_ex::Column::Id.eq(&id))
        .exec(db)
        .await?;
    Ok(id.clone())
}

/// 软删除（根据 ID）
pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    let now = Local::now().naive_local();
    AdtionExEntity::update_many()
        .col_expr(
            sys_adtion_ex::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(sys_adtion_ex::Column::DeletedAt, Expr::value(now))
        .col_expr(sys_adtion_ex::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_adtion_ex::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

/// 软删除（批量 ID）
pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    AdtionExEntity::update_many()
        .col_expr(
            sys_adtion_ex::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_adtion_ex::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_adtion_ex::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_adtion_ex::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

/// 硬删除（根据 ID）
pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match AdtionExEntity::delete_many()
        .filter(sys_adtion_ex::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在.")),
        _ => Ok("成功删除数据.".to_string()),
    }
}

/// 硬删除（批量 ID）
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match AdtionExEntity::delete_many()
        .filter(sys_adtion_ex::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在.")),
        i => Ok(format!("成功删除{}条数据.", i)),
    }
}

/// 查询附件类别（根据 ID）
pub async fn find_by_id(db: &DbConn, id: &str) -> Result<AdtionExModel> {
    match AdtionExEntity::find()
        .filter(sys_adtion_ex::Column::DeletedAt.is_null())
        .filter(sys_adtion_ex::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("附件类别不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询附件类别（分页）
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: AdtionExSearchReq,
) -> Result<PageData<AdtionExModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = AdtionExEntity::find();

    // 查询条件处理
    if let Some(x) = req.mcode {
        q = q.filter(sys_adtion_ex::Column::Mcode.eq(x));
    }
    if let Some(x) = req.exname {
        q = q.filter(sys_adtion_ex::Column::Exname.contains(x));
    }

    let paginator = q
        .order_by_asc(sys_adtion_ex::Column::Ord)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
