pub use sea_orm_migration::prelude::*;
pub mod db;

mod m20220101_000001_create_table;

/// Cli工具
pub struct Migrator;
/// Cli数据
pub static DATA_DIR: &str = "migration/data/";

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}
