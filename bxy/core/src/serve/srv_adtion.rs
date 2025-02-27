//!
//! 附件业务接口
//!
//!

use anyhow::{anyhow, Result};
use byz_entity::core::{sys_adtion, AdtionEntity, AdtionModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::PaginatorTrait;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set,
};

use crate::api::request::req_adtion::AdtionSearchReq;
use crate::api::request::{req::DeleteReq, req_adtion::AdtionReq};
use crate::model::app_structs::{PageData, PageParams, ToPageData};

/// 新增附件
pub async fn add(db: &DbConn, req: AdtionReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let now: NaiveDateTime = Local::now().naive_local();

    sys_adtion::ActiveModel {
        id: Set(id.clone()),
        guid: Set(req.guid),
        create_by: Set(op_user.to_string()), // 创建人
        created_at: Set(now),                // 创建时间
        mcode: Set(req.mcode),
        fname: Set(req.fname),
        ext: Set(req.ext),
        fsize: Set(req.fsize),
        url: Set(req.url),
        ext1: Set(req.ext1),
        ext2: Set(req.ext2),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

/// 软删除附件
pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    let now = Local::now().naive_local();
    AdtionEntity::update_many()
        .col_expr(sys_adtion::Column::DeletedAt, Expr::value(now))
        .col_expr(sys_adtion::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_adtion::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

/// 硬删除附件
pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match AdtionEntity::delete_many()
        .filter(sys_adtion::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在.")),
        _i => Ok("成功删除数据.".to_string()),
    }
}

/// 硬删除附件
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match AdtionEntity::delete_many()
        .filter(sys_adtion::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在.")),
        i => Ok(format!("成功删除{}条数据.", i)),
    }
}

/// 查询附件（根据 id）
pub async fn find_by_id(db: &DbConn, id: &str) -> Result<AdtionModel> {
    match AdtionEntity::find()
        .filter(sys_adtion::Column::DeletedAt.is_null())
        .filter(sys_adtion::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("附件不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询所有附件（根据业务主键）
pub async fn find_by_guid(db: &DbConn, guid: &str) -> Result<Vec<AdtionModel>> {
    let adtion = AdtionEntity::find()
        .filter(sys_adtion::Column::DeletedAt.is_null())
        .filter(sys_adtion::Column::Guid.eq(guid))
        .all(db)
        .await?;
    Ok(adtion)
}

/// 查询附件类别（分页）
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: AdtionSearchReq,
) -> Result<PageData<AdtionModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = AdtionEntity::find();

    // 查询条件处理
    if let Some(x) = req.guid {
        if !x.is_empty() {
            q = q.filter(sys_adtion::Column::Guid.eq(&x));
        }
    }
    if let Some(x) = req.mcode {
        if !x.is_empty() {
            q = q.filter(sys_adtion::Column::Mcode.eq(&x))
        }
    }
    if let Some(x) = req.fname {
        if !x.is_empty() {
            q = q.filter(sys_adtion::Column::Fname.eq(&x))
        }
    }
    if let Some(x) = req.ext {
        if !x.is_empty() {
            q = q.filter(sys_adtion::Column::Ext.eq(&x))
        }
    }

    let paginator = q.paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
