use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(caretta_agent_migration::Migrator).await;
}
