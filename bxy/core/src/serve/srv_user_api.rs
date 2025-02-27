use crate::{
    api::request::{
        req::DeleteReq,
        req_user::{UserApiReq, UserApiSearchReq},
    },
    model::app_structs::{PageData, PageParams, ToPageData},
};
use anyhow::{anyhow, Result};
use byz_entity::core::{sys_user_api, UserApiEntity, UserApiModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, Set,
};

/// 新增接口
pub async fn add(db: &DbConn, req: UserApiReq) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let now: NaiveDateTime = Local::now().naive_local();

    sys_user_api::ActiveModel {
        id: Set(id.clone()),
        created_at: Set(now),
        u_id: Set(req.u_id),
        mcode: Set(req.mcode),
        api: Set(req.api),
        method: Set(req.method),
    }
    .insert(db)
    .await?;
    Ok(id)
}

/// 硬删除接口（指定 ID）
pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match UserApiEntity::delete_many()
        .filter(sys_user_api::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

/// 硬删除接口（批量 ID）
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match UserApiEntity::delete_many()
        .filter(sys_user_api::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 查询用户接口（指定 ID）
pub async fn find_by_id(db: &DbConn, id: &str) -> Result<UserApiModel> {
    match UserApiEntity::find()
        .filter(sys_user_api::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("用户接口不存在.")),
        Some(u) => Ok(u),
    }
}

/// 查询用户接口（指定 用户ID）
pub async fn find_by_uid(db: &DbConn, uid: &str) -> Result<UserApiModel> {
    match UserApiEntity::find()
        .filter(sys_user_api::Column::UId.eq(uid))
        .one(db)
        .await?
    {
        None => Err(anyhow!("用户接口不存在.")),
        Some(u) => Ok(u),
    }
}

pub async fn check_api_permission(
    db: &DbConn,
    u_id: &str,
    mcode: &str,
    api: &str,
    method: &str,
) -> Result<bool> {
    match UserApiEntity::find()
        .filter(sys_user_api::Column::UId.eq(u_id))
        .filter(sys_user_api::Column::Mcode.eq(mcode))
        .filter(sys_user_api::Column::Api.eq(api))
        .filter(sys_user_api::Column::Method.eq(method))
        .one(db)
        .await?
    {
        Some(_) => Ok(true),
        None => Err(anyhow!("用户接口不存在.")),
    }
}

/// 查询用户接口（分页）
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: UserApiSearchReq,
) -> Result<PageData<UserApiModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = UserApiEntity::find();

    if let Some(x) = req.u_id {
        if !x.is_empty() {
            q = q.filter(sys_user_api::Column::UId.eq(x));
        }
    }
    if let Some(x) = req.mcode {
        if !x.is_empty() {
            q = q.filter(sys_user_api::Column::Mcode.eq(x));
        }
    }
    if let Some(x) = req.api {
        if !x.is_empty() {
            q = q.filter(sys_user_api::Column::Api.eq(x));
        }
    }
    if let Some(x) = req.method {
        if !x.is_empty() {
            q = q.filter(sys_user_api::Column::Method.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + "00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            q = q.filter(sys_user_api::Column::CreatedAt.gte(t));
        }
    }

    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + "23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            q = q.filter(sys_user_api::Column::CreatedAt.lte(t));
        }
    }

    let paginator = q.paginate(db, page_size);

    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}
