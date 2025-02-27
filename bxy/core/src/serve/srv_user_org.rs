use anyhow::Result;
use byz_entity::core::{sys_user_org, UserOrgEntity};
use chrono::Local;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbConn, EntityTrait, QueryFilter, Set, TransactionTrait,
};

use crate::api::request::req_user::UserOrgReq;

/// 用户部门
pub async fn user_org<C>(db: &DbConn, user_orgs: Vec<UserOrgReq>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserOrgEntity::insert_many(
        user_orgs
            .iter()
            .map(|x| sys_user_org::ActiveModel {
                id: Set(scru128::new_string().to_uppercase()),
                u_id: Set(x.u_id.clone()),
                o_id: Set(x.o_id.clone()),
                u_type: Set(x.u_type),
                created_at: Set(Local::now().naive_local()),
            })
            .collect::<Vec<_>>(),
    )
    .exec(db)
    .await?;
    Ok(())
}

/// 删除用户部门（指定 ID）
pub async fn delete_by_id<C>(db: &DbConn, id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserOrgEntity::delete_many()
        .filter(sys_user_org::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(())
}

/// 删除用户部门（指定 用户ID）
pub async fn delete_by_uid<C>(db: &DbConn, u_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserOrgEntity::delete_many()
        .filter(sys_user_org::Column::UId.eq(u_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 批量删除用户部门（多用户）
pub async fn delete_by_uids<C>(db: &DbConn, uid_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserOrgEntity::delete_many()
        .filter(sys_user_org::Column::UId.is_in(uid_ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 删除用户部门（指定部门）
pub async fn delete_by_org<C>(db: &DbConn, org_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserOrgEntity::delete_many()
        .filter(sys_user_org::Column::OId.eq(org_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 批量删除用户部门（多部门）
pub async fn delete_by_orgs<C>(db: &DbConn, org_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserOrgEntity::delete_many()
        .filter(sys_user_org::Column::OId.is_in(org_ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 获取用户部门（根据用户ID）
pub async fn find_by_uid(db: &DbConn, u_id: &str) -> Result<Vec<String>> {
    let s = UserOrgEntity::find()
        .filter(sys_user_org::Column::UId.eq(u_id))
        .all(db)
        .await?;
    let res = s.iter().map(|x| x.o_id.clone()).collect::<Vec<_>>();
    Ok(res)
}
