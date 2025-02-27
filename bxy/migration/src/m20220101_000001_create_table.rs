use crate::db::{create_one_table as Cot, drop_one_table as Dot, init_data};
use byz_entity::core::*;

pub use sea_orm::{ConnectionTrait, Schema};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_table(manager).await?;
        // create_index(manager).await?;
        init_data(manager, Migration.name()).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_table(manager).await
    }
}

/// 创建表格
async fn create_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    println!("开始创建表格----------");
    let db = manager.get_connection();
    // create_table(manager).await
    let builder = manager.get_database_backend();
    let schema = Schema::new(builder);

    Cot(db, builder, &schema, sys_adtion::Entity).await?;
    Cot(db, builder, &schema, sys_adtion_ex::Entity).await?;
    Cot(db, builder, &schema, sys_app::Entity).await?;
    Cot(db, builder, &schema, sys_app_auth::Entity).await?;
    Cot(db, builder, &schema, sys_col_auth::Entity).await?;
    Cot(db, builder, &schema, sys_datasource::Entity).await?;
    Cot(db, builder, &schema, sys_dict::Entity).await?;
    Cot(db, builder, &schema, sys_dict_data::Entity).await?;
    Cot(db, builder, &schema, sys_dql::Entity).await?;
    Cot(db, builder, &schema, sys_duty::Entity).await?;
    Cot(db, builder, &schema, sys_job::Entity).await?;
    Cot(db, builder, &schema, sys_job_log::Entity).await?;
    Cot(db, builder, &schema, sys_login_log::Entity).await?;
    Cot(db, builder, &schema, sys_master_detail::Entity).await?;
    Cot(db, builder, &schema, sys_menu::Entity).await?;
    Cot(db, builder, &schema, sys_online::Entity).await?;
    Cot(db, builder, &schema, sys_oper_log::Entity).await?;
    Cot(db, builder, &schema, sys_org::Entity).await?;
    Cot(db, builder, &schema, sys_post::Entity).await?;
    Cot(db, builder, &schema, sys_role::Entity).await?;
    Cot(db, builder, &schema, sys_row_auth::Entity).await?;
    Cot(db, builder, &schema, sys_tree::Entity).await?;
    Cot(db, builder, &schema, sys_update_log::Entity).await?;
    Cot(db, builder, &schema, sys_user::Entity).await?;
    Cot(db, builder, &schema, sys_user_api::Entity).await?;
    Cot(db, builder, &schema, sys_user_auth::Entity).await?;
    Cot(db, builder, &schema, sys_user_duty::Entity).await?;
    Cot(db, builder, &schema, sys_user_org::Entity).await?;
    Cot(db, builder, &schema, sys_user_post::Entity).await?;
    Cot(db, builder, &schema, sys_user_role::Entity).await?;

    Ok(())
}

//  创建索引
// async fn create_index(m: &SchemaManager<'_>) -> Result<(), DbErr> {
//     println!("开始创建索引----------");
//     Ok(())
// }

// 删除表格
async fn drop_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    println!("开始删除表格----------");

    Dot(manager, sys_adtion::Entity).await?;
    Dot(manager, sys_adtion_ex::Entity).await?;
    Dot(manager, sys_app::Entity).await?;
    Dot(manager, sys_app_auth::Entity).await?;
    Dot(manager, sys_col_auth::Entity).await?;
    Dot(manager, sys_datasource::Entity).await?;
    Dot(manager, sys_dict::Entity).await?;
    Dot(manager, sys_dict_data::Entity).await?;
    Dot(manager, sys_dql::Entity).await?;
    Dot(manager, sys_duty::Entity).await?;
    Dot(manager, sys_job::Entity).await?;
    Dot(manager, sys_job_log::Entity).await?;
    Dot(manager, sys_login_log::Entity).await?;
    Dot(manager, sys_master_detail::Entity).await?;
    Dot(manager, sys_menu::Entity).await?;
    Dot(manager, sys_online::Entity).await?;
    Dot(manager, sys_oper_log::Entity).await?;
    Dot(manager, sys_org::Entity).await?;
    Dot(manager, sys_post::Entity).await?;
    Dot(manager, sys_role::Entity).await?;
    Dot(manager, sys_row_auth::Entity).await?;
    Dot(manager, sys_tree::Entity).await?;
    Dot(manager, sys_update_log::Entity).await?;
    Dot(manager, sys_user::Entity).await?;
    Dot(manager, sys_user_api::Entity).await?;
    Dot(manager, sys_user_auth::Entity).await?;
    Dot(manager, sys_user_duty::Entity).await?;
    Dot(manager, sys_user_org::Entity).await?;
    Dot(manager, sys_user_post::Entity).await?;
    Dot(manager, sys_user_role::Entity).await?;

    Ok(())
}
