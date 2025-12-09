use std::sync::Arc;

use sea_orm::DatabaseConnection;

pub trait AsDatabaseConnection {
    fn as_database_connection(&self) -> &DatabaseConnection;
}

impl AsDatabaseConnection for DatabaseConnection {
    fn as_database_connection(&self) -> &DatabaseConnection {
        &self
    }
}

impl AsDatabaseConnection for Arc<DatabaseConnection> {
    fn as_database_connection(&self) -> &DatabaseConnection {
        self.as_ref()
    }
}