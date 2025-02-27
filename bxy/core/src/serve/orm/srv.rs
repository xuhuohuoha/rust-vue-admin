use anyhow::anyhow;
use sea_orm::{ConnectionTrait, DbConn, FromQueryResult, JsonValue, PaginatorTrait, Statement};

use crate::model::app_structs::{PageData, PageParams, ToPageData};

/// 通用 Service 接口
pub struct Service {}

pub trait ServiceTrait: Sync {
    // fn save<E: EntityTrait>(
    //     db: &DbConn,
    //     model: E::ActiveModel,
    // ) -> impl std::future::Future<Output = anyhow::Result<E::Model>> + Send;

    // fn insert<E: EntityTrait>(
    //     db: &DbConn,
    //     model: E::ActiveModel,
    // ) -> impl std::future::Future<Output = anyhow::Result<E::Model>> + Send;

    // fn update<E: EntityTrait>(
    //     db: &DbConn,
    //     model: E::ActiveModel,
    // ) -> impl std::future::Future<Output = anyhow::Result<E::Model>> + Send;

    // fn remove<E: EntityTrait>(
    //     db: &DbConn,
    //     model: E::ActiveModel,
    // ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    // fn delete<E: EntityTrait>(
    //     db: &DbConn,
    //     id: E::PrimaryKey,
    // ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    /// 查询单条数据
    ///
    /// # 参数
    /// - db：数据库连接
    /// - sql：原生SQL
    /// - params：动态参数
    fn find_one(
        db: &DbConn,
        sql: &str,
        params: Vec<sea_orm::Value>,
    ) -> impl std::future::Future<Output = anyhow::Result<sea_orm::JsonValue>> + Send;

    /// 查询所有数据
    ///
    /// # 参数
    /// - db：数据库连接
    /// - sql：原生SQL
    /// - params：动态参数
    fn find_all(
        db: &DbConn,
        sql: &str,
        params: Vec<sea_orm::Value>,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<sea_orm::JsonValue>>> + Send;

    /// 查询分页数据
    ///
    /// # 参数
    /// - db：数据库连接
    /// - sql：原生SQL
    /// - page_params：分页参数
    /// - params：动态参数
    fn find_list(
        db: &DbConn,
        sql: &str,
        page_params: PageParams,
        params: Vec<sea_orm::Value>,
    ) -> impl std::future::Future<Output = anyhow::Result<PageData<sea_orm::JsonValue>>> + Send;
}

impl ServiceTrait for Service {
    // async fn save<E: EntityTrait>(
    //     db: &DbConn,
    //     model: E::ActiveModel,
    // ) -> impl std::future::Future<Output = anyhow::Result<E::Model>> + Send {
    //     model.save(db)
    // }

    /// 查询单条数据
    async fn find_one(
        db: &DbConn,
        sql: &str,
        params: Vec<sea_orm::Value>,
    ) -> anyhow::Result<sea_orm::JsonValue> {
        let query_res = JsonValue::find_by_statement(Statement::from_sql_and_values(
            db.get_database_backend(),
            r#sql,
            params,
        ))
        .one(db)
        .await?;
        match query_res {
            Some(x) => Ok(x),
            None => Err(anyhow!("查询结果为空.")),
        }
    }

    /// 查询所有数据
    async fn find_all(
        db: &DbConn,
        sql: &str,
        params: Vec<sea_orm::Value>,
    ) -> anyhow::Result<Vec<sea_orm::JsonValue>> {
        let query_res: Vec<JsonValue> = JsonValue::find_by_statement(
            Statement::from_sql_and_values(db.get_database_backend(), r#sql, params),
        )
        .all(db)
        .await?;
        Ok(query_res)
    }

    /// 查询分页数据
    async fn find_list(
        db: &DbConn,
        sql: &str,
        page_params: PageParams,
        params: Vec<sea_orm::Value>,
    ) -> anyhow::Result<PageData<sea_orm::JsonValue>> {
        let page_num = page_params.page_num.unwrap_or(1);
        let page_size = page_params.page_size.unwrap_or(u32::MAX as u64);

        let params1 = params.clone();

        let query_res = JsonValue::find_by_statement(Statement::from_sql_and_values(
            db.get_database_backend(),
            r#sql,
            params,
        ))
        .paginate(db, page_size)
        .to_page_data(page_num, page_size)
        .await?;
        tracing::info!(
            "find_list {:?}",
            JsonValue::find_by_statement(Statement::from_sql_and_values(
                db.get_database_backend(),
                r#sql,
                params1,
            ))
        );
        Ok(query_res)
    }
}
