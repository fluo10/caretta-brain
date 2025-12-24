use sea_orm_migration::{prelude::*, schema::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            DatabaseBackend::Sqlite => {
                let db = manager.get_connection();
                todo!();
                db.execute_unprepared(
                    "CREATE TABLE invitation_token (
                        id             INTEGER PRIMARY KEY,
                        secret         INTEGER NOT NULL,
                        created_at     TEXT NOT NULL,
                        closed_at      TEXT,
                        status         INTEGER NOT NULL
                    )",
                )
                .await?;
                Ok(())
            }, 
            x => Err(DbErr::Migration(format!("Expected Sqlite, found {}", x.as_str())))
        }
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        unimplemented!();
    }
}
