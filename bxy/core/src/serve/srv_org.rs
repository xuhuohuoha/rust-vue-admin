//!
//! 部门业务接口
//!

use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use byz_entity::core::{sys_org, OrgEntity, OrgModel};

use crate::{
    api::request::{
        req::DeleteReq,
        req_org::{OrgReq, OrgSearchReq},
    },
    model::{
        app_structs::{PageData, PageParams, ToPageData},
        dict_structs::DataStatus,
    },
};

pub async fn add(db: &DbConn, req: OrgReq, op_user: &str) -> Result<String> {
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

    sys_org::ActiveModel {
        id: Set(id.clone()),
        created_at: Set(now),
        create_by: Set(op_user.to_string()),
        guid: Set(guid),
        pguid: Set(pguid),
        oname: Set(req.oname),
        version: Set(1),
        ord: Set(req.ord),
        status: Set(req.status),
        remark: Set(req.remark),
        att: Set(req.att),
        leader: Set(req.leader),
        email: Set(req.email),
        phone: Set(req.phone),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

pub async fn edit(db: &DbConn, req: OrgReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now = Local::now().naive_local();
    OrgEntity::update_many()
        .col_expr(sys_org::Column::Guid, Expr::value(req.guid))
        .col_expr(sys_org::Column::Pguid, Expr::value(req.pguid))
        .col_expr(sys_org::Column::Oname, Expr::value(req.oname))
        .col_expr(sys_org::Column::Leader, Expr::value(req.leader))
        .col_expr(sys_org::Column::Email, Expr::value(req.email))
        .col_expr(sys_org::Column::Phone, Expr::value(req.phone))
        .col_expr(sys_org::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_org::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_org::Column::Att, Expr::value(req.att))
        .col_expr(sys_org::Column::Status, Expr::value(req.status))
        .col_expr(sys_org::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_org::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_org::Column::Version,
            Expr::value(Expr::col(sys_org::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_org::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok(id)
}

pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    OrgEntity::update_many()
        .col_expr(
            sys_org::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_org::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_org::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_org::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    OrgEntity::update_many()
        .col_expr(sys_org::Column::Status, Expr::value("2".to_string()))
        .col_expr(
            sys_org::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_org::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_org::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match OrgEntity::delete_many()
        .filter(sys_org::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match OrgEntity::delete_many()
        .filter(sys_org::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<OrgModel> {
    match OrgEntity::find()
        .filter(sys_org::Column::DeletedAt.is_null())
        .filter(sys_org::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("岗位不存在.")),
        Some(u) => Ok(u),
    }
}

pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: OrgSearchReq,
) -> Result<PageData<OrgModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = OrgEntity::find();

    // 查询条件处理
    if let Some(x) = req.guid {
        if !x.is_empty() {
            q = q.filter(sys_org::Column::Guid.is_in(&x));
        }
    }
    if let Some(x) = req.pguid {
        if !x.is_empty() {
            q = q.filter(sys_org::Column::Pguid.is_in(&x));
        }
    }
    if let Some(x) = req.oname {
        if !x.is_empty() {
            q = q.filter(sys_org::Column::Oname.contains(&x));
        }
    }

    let paginator = q.order_by_asc(sys_org::Column::Ord).paginate(db, page_size);

    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
