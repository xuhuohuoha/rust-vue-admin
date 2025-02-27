//! 数据字典业务接口
//!

use anyhow::{anyhow, Result};
use byz_entity::core::{sys_dict_data, DictDataEntity, DictDataModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::api::request::req::DeleteReq;
use crate::api::request::req_dict::{DictDataReq, DictDataSearchReq};
use crate::model::app_structs::{PageData, PageParams, ToPageData};
use crate::model::dict_structs::DataStatus;

/// 新增字典
///
/// 物理主键 id 随机生成
/// 业务主键 guid 接口传入或随机生成
/// 自动生成创建人、创建时间、版本号（默认1）
pub async fn add(db: &DbConn, req: DictDataReq, op_user: &str) -> Result<String> {
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

    sys_dict_data::ActiveModel {
        id: Set(id.clone()),
        guid: Set(guid),
        pguid: Set(pguid),
        create_by: Set(op_user.to_string()), // 创建人
        created_at: Set(now),                // 创建时间
        version: Set(1),                     // 版本默认为 1
        ord: Set(req.ord),
        status: Set(req.status),
        remark: Set(req.remark),
        dname: Set(req.dname),
        att: Set(req.att),
        lvl: Set(req.lvl),
        ext1: Set(req.ext1),
        ext2: Set(req.ext2),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

/// 编辑字典
///
/// 自动生成修改人、修改时间、版本号（+1）
pub async fn edit(db: &DbConn, req: DictDataReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now = Local::now().naive_local();
    DictDataEntity::update_many()
        .col_expr(sys_dict_data::Column::Guid, Expr::value(req.guid))
        .col_expr(sys_dict_data::Column::Pguid, Expr::value(req.pguid))
        .col_expr(sys_dict_data::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_dict_data::Column::Status, Expr::value(req.status))
        .col_expr(sys_dict_data::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_dict_data::Column::Dname, Expr::value(req.dname))
        .col_expr(sys_dict_data::Column::Att, Expr::value(req.att))
        .col_expr(sys_dict_data::Column::Lvl, Expr::value(req.lvl))
        .col_expr(sys_dict_data::Column::Ext1, Expr::value(req.ext1))
        .col_expr(sys_dict_data::Column::Ext2, Expr::value(req.ext2))
        .col_expr(sys_dict_data::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_dict_data::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_dict_data::Column::Version,
            Expr::value(Expr::col(sys_dict_data::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_dict_data::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok(id)
}

/// 软删除字典
///
/// 设置状态为 DataStatus.Delete = 2
/// 自动生成删除人、删除时间
pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    // 更新
    DictDataEntity::update_many()
        .col_expr(
            sys_dict_data::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_dict_data::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_dict_data::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_dict_data::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

/// 软删除字典
///
/// 设置状态为 DataStatus.Delete = 2
/// 自动生成删除人、删除时间
pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    // 更新
    DictDataEntity::update_many()
        .col_expr(sys_dict_data::Column::Status, Expr::value("2".to_string()))
        .col_expr(
            sys_dict_data::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_dict_data::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_dict_data::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

/// 硬删除字典
pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match DictDataEntity::delete_many()
        .filter(sys_dict_data::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

/// 硬删除字典
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match DictDataEntity::delete_many()
        .filter(sys_dict_data::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 查找字典（根据 id）
pub async fn find_by_id(db: &DbConn, id: &str) -> Result<DictDataModel> {
    match DictDataEntity::find()
        .filter(sys_dict_data::Column::DeletedAt.is_null())
        .filter(sys_dict_data::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("字典不存在")),
        Some(u) => Ok(u),
    }
}

/// 分页查询列表
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: DictDataSearchReq,
) -> Result<PageData<DictDataModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = DictDataEntity::find();

    // 查询条件处理
    if let Some(x) = req.guid {
        if !x.is_empty() {
            q = q.filter(sys_dict_data::Column::Guid.contains(&x));
        }
    }
    if let Some(x) = req.dname {
        if !x.is_empty() {
            q = q.filter(sys_dict_data::Column::Dname.is_in(&x));
        }
    }
    if let Some(x) = req.att {
        if !x.is_empty() {
            q = q.filter(sys_dict_data::Column::Att.contains(&x));
        }
    }

    let paginator = q
        .order_by_asc(sys_dict_data::Column::Ord)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
