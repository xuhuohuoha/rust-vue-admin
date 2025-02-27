//!
//! 数据字典业务接口
//!

use anyhow::{anyhow, Result};
use byz_entity::core::{sys_dict, DictEntity, DictModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::api::request::req::DeleteReq;
use crate::api::request::req_dict::{DictReq, DictSearchReq};
use crate::model::app_structs::{PageData, PageParams, ToPageData};
use crate::model::dict_structs::DataStatus;

/// 新增数据字典
///
/// #描述
/// -物理主键 id 随机生成
/// -业务主键 guid 接口传入或随机生成
/// -自动生成创建人、创建时间、版本号（默认1）
pub async fn add(db: &DbConn, req: DictReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let guid = if req.guid.is_empty() {
        scru128::new_string().to_uppercase()
    } else {
        req.guid.clone()
    };
    let pguid = if req.pguid.is_empty() {
        req.guid
    } else {
        req.pguid
    };
    let now: NaiveDateTime = Local::now().naive_local();

    sys_dict::ActiveModel {
        id: Set(id.clone()),
        guid: Set(guid),
        pguid: Set(pguid),
        create_by: Set(op_user.to_string()), // 创建人
        created_at: Set(now),                // 创建时间
        version: Set(1),                     // 版本默认为 1
        ord: Set(req.ord),
        status: Set(req.status),
        remark: Set(Some(req.remark)),
        dname: Set(req.dname),
        att: Set(Some(req.att)),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

/// 编辑数据字典
///
/// 自动生成修改人、修改时间、版本号（+1）
pub async fn edit(db: &DbConn, req: DictReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now = Local::now().naive_local();
    DictEntity::update_many()
        .col_expr(sys_dict::Column::Guid, Expr::value(req.guid))
        .col_expr(sys_dict::Column::Pguid, Expr::value(req.pguid))
        .col_expr(sys_dict::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_dict::Column::Status, Expr::value(req.status))
        .col_expr(sys_dict::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_dict::Column::Dname, Expr::value(req.dname))
        .col_expr(sys_dict::Column::Att, Expr::value(req.att))
        .col_expr(sys_dict::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_dict::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_dict::Column::Version,
            Expr::value(Expr::col(sys_dict::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_dict::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok(id)
}

/// 软删除数据字典
///
/// 设置状态为 DataStatus.Delete = 2
/// 自动生成删除人、删除时间
pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    // 更新
    DictEntity::update_many()
        .col_expr(
            sys_dict::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_dict::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_dict::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_dict::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

/// 软删除数据字典
///
/// 设置状态为 DataStatus.Delete = 2
/// 自动生成删除人、删除时间
pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    // 更新
    DictEntity::update_many()
        .col_expr(sys_dict::Column::Status, Expr::value("2".to_string()))
        .col_expr(
            sys_dict::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_dict::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_dict::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

/// 硬删除数据字典
pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match DictEntity::delete_many()
        .filter(sys_dict::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

/// 硬删除数据字典
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match DictEntity::delete_many()
        .filter(sys_dict::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 查找数据字典（根据 id）
pub async fn find_by_id(db: &DbConn, id: &str) -> Result<DictModel> {
    match DictEntity::find()
        .filter(sys_dict::Column::DeletedAt.is_null())
        .filter(sys_dict::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("数据字典不存在")),
        Some(u) => Ok(u),
    }
}

/// 分页查询列表
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: DictSearchReq,
) -> Result<PageData<DictModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = DictEntity::find();

    // 查询条件处理
    if let Some(x) = req.guid {
        if !x.is_empty() {
            q = q.filter(sys_dict::Column::Guid.contains(&x));
        }
    }
    if let Some(x) = req.dname {
        if !x.is_empty() {
            q = q.filter(sys_dict::Column::Dname.contains(&x));
        }
    }

    let paginator = q
        .order_by_asc(sys_dict::Column::Ord)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
