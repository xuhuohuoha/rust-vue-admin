//!
//! 应用业务接口
//!
//!
use anyhow::{anyhow, Result};
use byz_entity::core::{sys_app, AppAuthModel, AppEntity, AppModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::api::request::req::DeleteReq;
use crate::api::request::req_app::{AppAuthReq, AppReq, AppSearchReq};
use crate::model::app_structs::{PageData, PageParams, ToPageData};
use crate::model::dict_structs::DataStatus;

use super::srv_app_auth;

/// 新增应用
///
/// # 描述
/// - 物理主键 id 随机生成
/// - 业务主键 guid 接口传入或随机生成
/// - 自动生成创建人、创建时间、版本号（默认1）
pub async fn add(db: &DbConn, req: AppReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let guid = if req.guid.is_empty() {
        scru128::new_string().to_uppercase()
    } else {
        req.guid
    };
    let now: NaiveDateTime = Local::now().naive_local();

    (sys_app::ActiveModel {
        id: Set(id.clone()),
        guid: Set(guid),
        create_by: Set(op_user.to_string()), // 创建人
        created_at: Set(now),                // 创建时间
        version: Set(1),                     // 版本默认为 1
        ord: Set(req.ord),
        status: Set(DataStatus::Status(req.status).to_string()),
        remark: Set(Some(req.remark)),
        app_code: Set(req.app_code),
        app_key: Set(req.app_key),
        app_name: Set(req.app_name),
        app_att: Set(Some(req.app_att)),
        app_type: Set(Some(req.app_type)),
        logo: Set(Some(req.logo)),
        ..Default::default()
    })
    .insert(db)
    .await?;
    Ok(id)
}

/// 编辑应用
///
/// 自动生成修改人、修改时间、版本号（+1）
pub async fn edit(db: &DbConn, req: AppReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now = Local::now().naive_local();
    AppEntity::update_many()
        .col_expr(sys_app::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_app::Column::Status, Expr::value(req.status))
        .col_expr(sys_app::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_app::Column::AppCode, Expr::value(req.app_code))
        .col_expr(sys_app::Column::AppKey, Expr::value(req.app_key))
        .col_expr(sys_app::Column::AppName, Expr::value(req.app_name))
        .col_expr(sys_app::Column::AppAtt, Expr::value(req.app_att))
        .col_expr(sys_app::Column::AppType, Expr::value(req.app_type))
        .col_expr(sys_app::Column::Logo, Expr::value(req.logo))
        .col_expr(sys_app::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_app::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_app::Column::Version,
            Expr::value(Expr::col(sys_app::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_app::Column::Id.eq(req.id))
        .exec(db)
        .await?;
    Ok(id)
}

/// 软删除应用
///
/// 设置状态为 DataStatus.Delete = 2
/// 自动生成删除人、删除时间
pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    // 更新
    AppEntity::update_many()
        .col_expr(
            sys_app::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .col_expr(
            sys_app::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_app::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_app::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

/// 软删除应用
///
/// 设置状态为 DataStatus.Delete = 2
/// 自动生成删除人、删除时间
pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    // 更新
    AppEntity::update_many()
        .col_expr(sys_app::Column::Status, Expr::value("2".to_string()))
        .col_expr(
            sys_app::Column::DeletedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(sys_app::Column::DeleteBy, Expr::value(op_user))
        .filter(sys_app::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

/// 硬删除应用
pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match AppEntity::delete_many()
        .filter(sys_app::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

/// 硬删除应用
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match AppEntity::delete_many()
        .filter(sys_app::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 查找应用（根据 id）
pub async fn find_by_id(db: &DbConn, id: &str) -> Result<AppModel> {
    match AppEntity::find()
        .filter(sys_app::Column::DeletedAt.is_null())
        .filter(sys_app::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("应用不存在")),
        Some(u) => Ok(u),
    }
}

/// 分页查询列表
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: AppSearchReq,
) -> Result<PageData<AppModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = AppEntity::find().filter(sys_app::Column::DeletedAt.is_null());

    // 查询条件处理
    if let Some(x) = req.guid {
        if !x.is_empty() {
            q = q.filter(sys_app::Column::Guid.eq(&x));
        }
    }
    if let Some(x) = req.app_code {
        if !x.is_empty() {
            q = q.filter(sys_app::Column::AppCode.eq(&x));
        }
    }
    if let Some(x) = req.app_key {
        if !x.is_empty() {
            q = q.filter(sys_app::Column::AppKey.eq(&x));
        }
    }
    if let Some(x) = req.app_name {
        if !x.is_empty() {
            q = q.filter(sys_app::Column::AppName.contains(&x));
        }
    }

    let paginator = q.order_by_asc(sys_app::Column::Ord).paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}

/// 应用授权检验
///
/// 若检验成功，则返回临时授权码
pub async fn auth(db: &DbConn, req: AppAuthReq) -> Result<AppAuthModel> {
    match AppEntity::find()
        .filter(sys_app::Column::DeletedAt.is_null())
        .filter(sys_app::Column::AppCode.eq(&req.app_code))
        .filter(sys_app::Column::AppKey.eq(req.app_key))
        .one(db)
        .await?
    {
        None => Err(anyhow!("应用授权检验错误.")),
        Some(x) => Ok(srv_app_auth::add_auth(db, &x.guid).await?),
    }
}
