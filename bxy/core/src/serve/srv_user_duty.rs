use anyhow::Result;
use byz_entity::core::{sys_user_duty, UserDutyEntity};
use chrono::Local;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbConn, EntityTrait, QueryFilter, Set, TransactionTrait,
};

/// 用户职务
pub async fn user_duty<C>(db: &DbConn, u_id: &str, duty_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserDutyEntity::insert_many(
        duty_ids
            .clone()
            .iter()
            .map(|x| sys_user_duty::ActiveModel {
                id: Set(scru128::new_string().to_uppercase()),
                u_id: Set(u_id.to_string()),
                d_id: Set(x.to_string()),
                created_at: Set(Local::now().naive_local()),
            })
            .collect::<Vec<_>>(),
    )
    .exec(db)
    .await?;
    Ok(())
}

/// 删除用户职务（指定 ID）
pub async fn delete_by_id<C>(db: &DbConn, id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserDutyEntity::delete_many()
        .filter(sys_user_duty::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(())
}

/// 删除用户职务（指定 用户ID）
pub async fn delete_by_uid<C>(db: &DbConn, u_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserDutyEntity::delete_many()
        .filter(sys_user_duty::Column::UId.eq(u_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 批量删除用户职务（多用户）
pub async fn delete_by_uids<C>(db: &DbConn, uid_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserDutyEntity::delete_many()
        .filter(sys_user_duty::Column::UId.is_in(uid_ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 删除用户职务（指定职务）
pub async fn delete_by_duty<C>(db: &DbConn, duty_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserDutyEntity::delete_many()
        .filter(sys_user_duty::Column::DId.eq(duty_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 批量删除用户职务（多职务）
pub async fn delete_by_dutys<C>(db: &DbConn, duty_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserDutyEntity::delete_many()
        .filter(sys_user_duty::Column::DId.is_in(duty_ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 获取用户职务（根据 用户ID）
pub async fn find_by_uid(db: &DbConn, u_id: &str) -> Result<Vec<String>> {
    let s = UserDutyEntity::find()
        .filter(sys_user_duty::Column::UId.eq(u_id))
        .all(db)
        .await?;
    let res = s.iter().map(|x| x.d_id.clone()).collect::<Vec<_>>();
    Ok(res)
}
