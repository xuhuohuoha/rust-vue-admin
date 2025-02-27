use anyhow::{anyhow, Result};
use byz_entity::core::{sys_app_auth, AppAuthEntity, AppAuthModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, Set,
};

use crate::api::request::req::DeleteReq;
use crate::api::request::req_app::{AppAuthSearchReq, AppTempReq};
use crate::model::app_structs::{PageData, PageParams, ToPageData};

/// 分页查询列表
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: AppAuthSearchReq,
) -> Result<PageData<AppAuthModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = AppAuthEntity::find();

    // 查询条件处理
    if let Some(x) = req.app_code {
        if !x.is_empty() {
            q = q.filter(sys_app_auth::Column::AppCode.eq(&x));
        }
    }
    if let Some(x) = req.temp_code {
        if !x.is_empty() {
            q = q.filter(sys_app_auth::Column::TempCode.eq(&x));
        }
    }

    let paginator = q.paginate(db, page_size);

    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}

/// 应用授权码置换
///
/// 应用临时授权码为临时的，仅可使用一次，置换后则删除
pub async fn auth_code(db: &DbConn, req: AppTempReq) -> Result<String> {
    let auth_s = AppAuthEntity::find()
        .filter(sys_app_auth::Column::AppCode.eq(&req.app_code))
        .filter(sys_app_auth::Column::TempCode.eq(&req.temp_code))
        .one(db)
        .await?;
    AppAuthEntity::delete_many()
        .filter(sys_app_auth::Column::TempCode.eq(&req.temp_code))
        .exec(db)
        .await?;
    match auth_s {
        None => Err(anyhow!("临时授权码不存在.")),
        Some(u) => {
            delete_auth(db, req).await?;
            Ok(u.temp_code)
        }
    }
}

/// 新增应用临时授权
pub async fn add_auth(db: &DbConn, app_code: &str) -> Result<AppAuthModel> {
    let id = scru128::new_string();
    let temp_code = scru128::new_string();
    let auth = sys_app_auth::ActiveModel {
        id: Set(id),
        app_code: Set(app_code.to_string()),
        temp_code: Set(temp_code.clone()),
    }
    .insert(db)
    .await?;
    Ok(auth)
}

/// 硬删除应用临时授权
pub async fn delete_auth(db: &DbConn, req: AppTempReq) -> Result<String> {
    match AppAuthEntity::delete_many()
        .filter(sys_app_auth::Column::AppCode.eq(req.app_code))
        .filter(sys_app_auth::Column::TempCode.eq(req.temp_code))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 硬删除应用临时授权
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match AppAuthEntity::delete_many()
        .filter(sys_app_auth::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}
