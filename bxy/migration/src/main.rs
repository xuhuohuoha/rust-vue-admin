use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(byz_migration::Migrator).await;
}
