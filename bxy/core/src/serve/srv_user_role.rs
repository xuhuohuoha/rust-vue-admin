use anyhow::Result;
use byz_entity::core::{sys_user_role, UserRoleEntity};
use chrono::Local;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbConn, EntityTrait, QueryFilter, Set, TransactionTrait,
};

/// 用户角色
pub async fn user_role<C>(db: &DbConn, u_id: &str, role_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserRoleEntity::insert_many(
        role_ids
            .clone()
            .iter()
            .map(|x| sys_user_role::ActiveModel {
                id: Set(scru128::new_string().to_uppercase()),
                u_id: Set(u_id.to_string()),
                r_id: Set(x.to_string()),
                created_at: Set(Local::now().naive_local()),
            })
            .collect::<Vec<_>>(),
    )
    .exec(db)
    .await?;
    Ok(())
}

/// 删除用户角色（指定 ID）
pub async fn delete_by_id<C>(db: &DbConn, id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserRoleEntity::delete_many()
        .filter(sys_user_role::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(())
}

/// 删除用户角色（指定 用户ID）
pub async fn delete_by_uid<C>(db: &DbConn, u_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserRoleEntity::delete_many()
        .filter(sys_user_role::Column::UId.eq(u_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 批量删除用户角色（多用户）
pub async fn delete_by_uids<C>(db: &DbConn, uid_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserRoleEntity::delete_many()
        .filter(sys_user_role::Column::UId.is_in(uid_ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 删除用户角色（指定角色）
pub async fn delete_by_role<C>(db: &DbConn, role_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserRoleEntity::delete_many()
        .filter(sys_user_role::Column::RId.eq(role_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 批量删除用户角色（多角色）
pub async fn delete_by_roles<C>(db: &DbConn, role_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserRoleEntity::delete_many()
        .filter(sys_user_role::Column::RId.is_in(role_ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 获取用户角色（根据 用户ID）
pub async fn find_by_uid(db: &DbConn, u_id: &str) -> Result<Vec<String>> {
    let s = UserRoleEntity::find()
        .filter(sys_user_role::Column::UId.eq(u_id))
        .all(db)
        .await?;
    let res = s.iter().map(|x| x.r_id.clone()).collect::<Vec<_>>();
    Ok(res)
}
