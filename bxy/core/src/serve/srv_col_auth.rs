//!
//! 列级权限业务接口
//!

use anyhow::{anyhow, Result};
use byz_entity::core::{sys_col_auth, ColAuthEntity, ColAuthModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::sea_query::{Expr, Query, UnionType};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, EntityTrait, FromQueryResult,
    JsonValue, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use sea_query::SelectStatement;

use crate::api::request::req::DeleteReq;
use crate::api::request::req_col_auth::ColAuthSearchReq;
use crate::api::response::res_col_auth::ColAuthRes;
use crate::model::app_structs::{PageData, PageParams, ToPageData};
use crate::model::dict_structs::{AuthMethod, AuthType};
use crate::{api::request::req_col_auth::ColAuthReq, model::dict_structs::DataStatus};

use super::{srv_user_duty, srv_user_org, srv_user_post, srv_user_role};

/// 新增列级权限
pub async fn add(db: &DbConn, req: ColAuthReq, op_user: &str) -> Result<String> {
    let id = scru128::new_string().to_uppercase();
    let now: NaiveDateTime = Local::now().naive_local();
    sys_col_auth::ActiveModel {
        id: Set(id.clone()),
        create_by: Set(op_user.to_string()),
        created_at: Set(now),
        version: Set(1),
        ord: Set(req.ord),
        status: Set(req.status),
        remark: Set(req.remark),
        field: Set(req.field),
        regx: Set(req.regx),
        mcode: Set(req.mcode),
        atype: Set(req.atype),
        amethod: Set(req.amethod),
        u_id: Set(req.u_id),
        r_id: Set(req.r_id),
        o_id: Set(req.o_id),
        p_id: Set(req.p_id),
        d_id: Set(req.d_id),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(id)
}

/// 编辑列级权限
pub async fn edit(db: &DbConn, req: ColAuthReq, op_user: &str) -> Result<String> {
    let id = req.id.clone();
    let now: NaiveDateTime = Local::now().naive_local();
    ColAuthEntity::update_many()
        .col_expr(sys_col_auth::Column::UpdateBy, Expr::value(op_user))
        .col_expr(sys_col_auth::Column::UpdatedAt, Expr::value(now))
        .col_expr(sys_col_auth::Column::Ord, Expr::value(req.ord))
        .col_expr(sys_col_auth::Column::Status, Expr::value(req.status))
        .col_expr(sys_col_auth::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_col_auth::Column::Field, Expr::value(req.field))
        .col_expr(sys_col_auth::Column::Regx, Expr::value(req.regx))
        .col_expr(sys_col_auth::Column::Mcode, Expr::value(req.mcode))
        .col_expr(sys_col_auth::Column::Atype, Expr::value(req.atype))
        .col_expr(sys_col_auth::Column::Amethod, Expr::value(req.amethod))
        .col_expr(sys_col_auth::Column::UId, Expr::value(req.u_id))
        .col_expr(sys_col_auth::Column::RId, Expr::value(req.r_id))
        .col_expr(sys_col_auth::Column::OId, Expr::value(req.o_id))
        .col_expr(sys_col_auth::Column::PId, Expr::value(req.p_id))
        .col_expr(sys_col_auth::Column::DId, Expr::value(req.d_id))
        .col_expr(
            sys_col_auth::Column::Version,
            Expr::value(Expr::col(sys_col_auth::Column::Version).add(1)),
        ) // 版本号 +1
        .filter(sys_col_auth::Column::Id.eq(&id))
        .exec(db)
        .await?;
    Ok(id)
}

/// 软删除列级权限（根据 ID）
pub async fn remove_by_id(db: &DbConn, id: &str, op_user: &str) -> Result<bool> {
    ColAuthEntity::update_many()
        .col_expr(sys_col_auth::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_col_auth::Column::UpdatedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(
            sys_col_auth::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .filter(sys_col_auth::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(true)
}

/// 软删除列级权限（批量 ID）
pub async fn remove(db: &DbConn, req: DeleteReq, op_user: &str) -> Result<bool> {
    ColAuthEntity::update_many()
        .col_expr(sys_col_auth::Column::UpdateBy, Expr::value(op_user))
        .col_expr(
            sys_col_auth::Column::UpdatedAt,
            Expr::value(Local::now().naive_local()),
        )
        .col_expr(
            sys_col_auth::Column::Status,
            Expr::value(DataStatus::Delete.to_string()),
        )
        .filter(sys_col_auth::Column::Id.is_in(req.ids))
        .exec(db)
        .await?;
    Ok(true)
}

/// 硬删除列级权限（根据 ID）
pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<String> {
    match ColAuthEntity::delete_many()
        .filter(sys_col_auth::Column::Id.eq(id))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        _ => Ok("成功删除数据".to_string()),
    }
}

/// 硬删除列级权限（批量ID）
pub async fn delete(db: &DbConn, req: DeleteReq) -> Result<String> {
    match ColAuthEntity::delete_many()
        .filter(sys_col_auth::Column::Id.is_in(req.ids))
        .exec(db)
        .await?
        .rows_affected
    {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// 查询列级权限（根据 ID）
pub async fn find_by_id(db: &DbConn, id: &str) -> Result<ColAuthModel> {
    match ColAuthEntity::find()
        .filter(sys_col_auth::Column::DeletedAt.is_null())
        .filter(sys_col_auth::Column::Id.eq(id))
        .one(db)
        .await?
    {
        None => Err(anyhow!("列级授权不存在")),
        Some(u) => Ok(u),
    }
}

/// 分页查询列表
pub async fn find_all(
    db: &DbConn,
    page_params: PageParams,
    req: ColAuthSearchReq,
) -> Result<PageData<ColAuthModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

    let mut q = ColAuthEntity::find();

    // 查询条件处理
    if let Some(x) = req.mcode {
        if !x.is_empty() {
            q = q.filter(sys_col_auth::Column::Mcode.eq(&x));
        }
    }
    if let Some(x) = req.field {
        if !x.is_empty() {
            q = q.filter(sys_col_auth::Column::Field.contains(&x));
        }
    }

    let paginator = q
        .order_by_asc(sys_col_auth::Column::Ord)
        .paginate(db, page_size);
    let res = paginator.to_page_data(page_num, page_size).await?;
    Ok(res)
}

/// 查询列级权限（根据 菜单）
pub async fn find_by_mcode(db: &DbConn, mcode: &str) -> Result<Vec<ColAuthModel>> {
    let col_s = ColAuthEntity::find()
        .filter(sys_col_auth::Column::DeletedAt.is_null())
        .filter(sys_col_auth::Column::Mcode.eq(mcode))
        .all(db)
        .await?;
    Ok(col_s)
}

/// 查询列级权限（根据 菜单、用户ID）
pub async fn find_by_mcode_uid(db: &DbConn, mcode: &str, uid: &str) -> Result<Vec<ColAuthRes>> {
    find_mcode_uid(db, mcode, uid).await
}

pub async fn find_mcode_uid(db: &DbConn, mcode: &str, u_id: &str) -> Result<Vec<ColAuthRes>> {
    // 先查询一次用户相关部门、角色、岗位、职务
    let roles = srv_user_role::find_by_uid(db, u_id).await?;
    let orgs = srv_user_org::find_by_uid(db, u_id).await?;
    let posts = srv_user_post::find_by_uid(db, u_id).await?;
    let dutys = srv_user_duty::find_by_uid(db, u_id).await?;
    let mut query = Query::select();

    // 所有人权限
    find_by_all(&mut query, mcode).unwrap();
    // 所有包含权限
    find_by_uid(&mut query, mcode, &AuthType::In, u_id).unwrap();
    find_by_org(&mut query, mcode, &AuthType::In, &orgs).unwrap();
    find_by_role(&mut query, mcode, &AuthType::In, &roles).unwrap();
    find_by_post(&mut query, mcode, &AuthType::In, &posts).unwrap();
    find_by_duty(&mut query, mcode, &AuthType::In, &dutys).unwrap();
    find_by_org_role(&mut query, mcode, &AuthType::In, &orgs, &roles).unwrap();
    find_by_org_post(&mut query, mcode, &AuthType::In, &orgs, &posts).unwrap();
    find_by_org_duty(&mut query, mcode, &AuthType::In, &orgs, &dutys).unwrap();
    find_by_role_post(&mut query, mcode, &AuthType::In, &roles, &posts).unwrap();
    find_by_role_duty(&mut query, mcode, &AuthType::In, &roles, &dutys).unwrap();
    find_by_post_duty(&mut query, mcode, &AuthType::In, &posts, &dutys).unwrap();
    find_by_org_role_post(&mut query, mcode, &AuthType::In, &orgs, &roles, &posts).unwrap();
    find_by_org_role_duty(&mut query, mcode, &AuthType::In, &orgs, &roles, &dutys).unwrap();
    find_by_org_role_post_duty(
        &mut query,
        mcode,
        &AuthType::In,
        &orgs,
        &roles,
        &posts,
        &dutys,
    )
    .unwrap();
    // 所有排除权限
    find_by_uid(&mut query, mcode, &AuthType::Ex, u_id).unwrap();
    find_by_org(&mut query, mcode, &AuthType::Ex, &orgs).unwrap();
    find_by_role(&mut query, mcode, &AuthType::Ex, &roles).unwrap();
    find_by_post(&mut query, mcode, &AuthType::Ex, &posts).unwrap();
    find_by_duty(&mut query, mcode, &AuthType::Ex, &dutys).unwrap();
    find_by_org_role(&mut query, mcode, &AuthType::Ex, &orgs, &roles).unwrap();
    find_by_org_post(&mut query, mcode, &AuthType::Ex, &orgs, &posts).unwrap();
    find_by_org_duty(&mut query, mcode, &AuthType::Ex, &orgs, &dutys).unwrap();
    find_by_role_post(&mut query, mcode, &AuthType::Ex, &roles, &posts).unwrap();
    find_by_role_duty(&mut query, mcode, &AuthType::Ex, &roles, &dutys).unwrap();
    find_by_post_duty(&mut query, mcode, &AuthType::Ex, &posts, &dutys).unwrap();
    find_by_org_role_post(&mut query, mcode, &AuthType::Ex, &orgs, &roles, &posts).unwrap();
    find_by_org_role_duty(&mut query, mcode, &AuthType::Ex, &orgs, &roles, &dutys).unwrap();
    find_by_org_role_post_duty(
        &mut query,
        mcode,
        &AuthType::Ex,
        &orgs,
        &roles,
        &posts,
        &dutys,
    )
    .unwrap();

    let builder = db.get_database_backend();
    match JsonValue::find_by_statement(builder.build(&query))
        .all(db)
        .await
    {
        Ok(x) => Ok(x
            .iter()
            .map(|res| serde_json::from_value(res.clone()).unwrap())
            .collect::<Vec<_>>()),
        Err(e) => Err(anyhow!("查询列级授权出错，错误原因：{}", &e)),
    }
}

/// 指定所有人
pub fn find_by_all(query: &mut SelectStatement, mcode: &str) -> Result<()> {
    query
        .columns([
            sys_col_auth::Column::Mcode,
            sys_col_auth::Column::Field,
            sys_col_auth::Column::Regx,
        ])
        .from(ColAuthEntity)
        .and_where(sys_col_auth::Column::DeletedAt.is_null())
        .and_where(sys_col_auth::Column::Atype.eq(AuthType::In.to_int()))
        .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::All.to_int()));
    if !mcode.is_empty() {
        query.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    Ok(())
}

/// 指定用户
pub fn find_by_uid(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    u_id: &str,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::UId.eq(u_id))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::User.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());
    Ok(())
}

/// 指定部门
pub fn find_by_org(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    orgs: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::OId.is_in(orgs))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::Org.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定角色
pub fn find_by_role(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    roles: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::RId.is_in(roles))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::Role.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定岗位
pub fn find_by_post(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    posts: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::PId.is_in(posts))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::Post.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定职务
pub fn find_by_duty(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    dutys: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::DId.is_in(dutys))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::Duty.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定角色、职务
pub fn find_by_role_duty(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    roles: &Vec<String>,
    dutys: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::RId.is_in(roles))
    .and_where(sys_col_auth::Column::DId.is_in(dutys))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::RoleDuty.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定角色、岗位
pub fn find_by_role_post(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    roles: &Vec<String>,
    posts: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::RId.is_in(roles))
    .and_where(sys_col_auth::Column::PId.is_in(posts))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::RolePost.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定部门、角色
pub fn find_by_org_role(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    orgs: &Vec<String>,
    roles: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::OId.is_in(orgs))
    .and_where(sys_col_auth::Column::RId.is_in(roles))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::OrgRole.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定部门、职务
pub fn find_by_org_duty(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    orgs: &Vec<String>,
    dutys: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::DId.is_in(dutys))
    .and_where(sys_col_auth::Column::OId.is_in(orgs))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::OrgDuty.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定部门、岗位
pub fn find_by_org_post(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    orgs: &Vec<String>,
    posts: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::OId.is_in(orgs))
    .and_where(sys_col_auth::Column::PId.is_in(posts))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::OrgPost.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定职务、岗位
pub fn find_by_post_duty(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    posts: &Vec<String>,
    dutys: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::PId.is_in(posts))
    .and_where(sys_col_auth::Column::DId.is_in(dutys))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::PostDuty.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定部门、角色、职务
pub fn find_by_org_role_duty(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    orgs: &Vec<String>,
    roles: &Vec<String>,
    dutys: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::OId.is_in(orgs))
    .and_where(sys_col_auth::Column::RId.is_in(roles))
    .and_where(sys_col_auth::Column::DId.is_in(dutys))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::OrgRoleDuty.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定部门、角色、岗位
pub fn find_by_org_role_post(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    orgs: &Vec<String>,
    roles: &Vec<String>,
    posts: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::OId.is_in(orgs))
    .and_where(sys_col_auth::Column::RId.is_in(roles))
    .and_where(sys_col_auth::Column::PId.is_in(posts))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::OrgRolePost.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}

/// 指定部门、角色、职务、岗位
pub fn find_by_org_role_post_duty(
    query: &mut SelectStatement,
    mcode: &str,
    atype: &AuthType,
    orgs: &Vec<String>,
    roles: &Vec<String>,
    posts: &Vec<String>,
    dutys: &Vec<String>,
) -> Result<()> {
    let mut q = Query::select();
    q.columns([
        sys_col_auth::Column::Mcode,
        sys_col_auth::Column::Field,
        sys_col_auth::Column::Regx,
    ])
    .from(ColAuthEntity)
    .and_where(sys_col_auth::Column::OId.is_in(orgs))
    .and_where(sys_col_auth::Column::RId.is_in(roles))
    .and_where(sys_col_auth::Column::PId.is_in(posts))
    .and_where(sys_col_auth::Column::DId.is_in(dutys))
    .and_where(sys_col_auth::Column::DeletedAt.is_null())
    .and_where(sys_col_auth::Column::Atype.eq(atype.to_int()))
    .and_where(sys_col_auth::Column::Amethod.eq(AuthMethod::OrgRolePostDuty.to_int()));
    if !mcode.is_empty() {
        q.and_where(sys_col_auth::Column::Mcode.eq(mcode));
    }
    query.union(UnionType::All, q.take());

    Ok(())
}
