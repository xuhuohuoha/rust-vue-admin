use anyhow::Result;
use byz_entity::core::{sys_user_post, UserPostEntity};
use chrono::Local;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbConn, EntityTrait, QueryFilter, Set, TransactionTrait,
};

/// 用户岗位
pub async fn user_post<C>(db: &DbConn, u_id: &str, post_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserPostEntity::insert_many(
        post_ids
            .clone()
            .iter()
            .map(|x| sys_user_post::ActiveModel {
                id: Set(scru128::new_string().to_uppercase()),
                u_id: Set(u_id.to_string()),
                p_id: Set(x.to_string()),
                created_at: Set(Local::now().naive_local()),
            })
            .collect::<Vec<_>>(),
    )
    .exec(db)
    .await?;
    Ok(())
}

/// 删除用户岗位（指定 ID）
pub async fn delete_by_id<C>(db: &DbConn, id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserPostEntity::delete_many()
        .filter(sys_user_post::Column::Id.eq(id))
        .exec(db)
        .await?;
    Ok(())
}

/// 删除用户岗位（指定 用户ID）
pub async fn delete_by_uid<C>(db: &DbConn, u_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserPostEntity::delete_many()
        .filter(sys_user_post::Column::UId.eq(u_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 批量删除用户岗位（多用户）
pub async fn delete_by_uids<C>(db: &DbConn, uid_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserPostEntity::delete_many()
        .filter(sys_user_post::Column::UId.is_in(uid_ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 删除用户岗位（指定岗位）
pub async fn delete_by_post<C>(db: &DbConn, post_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserPostEntity::delete_many()
        .filter(sys_user_post::Column::PId.eq(post_id))
        .exec(db)
        .await?;
    Ok(())
}

/// 批量删除用户岗位（多岗位）
pub async fn delete_by_posts<C>(db: &DbConn, post_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    UserPostEntity::delete_many()
        .filter(sys_user_post::Column::PId.is_in(post_ids))
        .exec(db)
        .await?;
    Ok(())
}

/// 获取用户岗位（根据 用户ID）
pub async fn find_by_uid(db: &DbConn, u_id: &str) -> Result<Vec<String>> {
    let s = UserPostEntity::find()
        .filter(sys_user_post::Column::UId.eq(u_id))
        .all(db)
        .await?;
    let res = s.iter().map(|x| x.p_id.clone()).collect::<Vec<_>>();
    Ok(res)
}
