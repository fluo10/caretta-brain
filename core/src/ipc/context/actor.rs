use std::sync::Arc;

use iroh::Endpoint;
use sea_orm::DatabaseConnection;

use crate::ipc::IpcActor;

#[derive(Debug)]
pub struct IpcActorContext {
    pub database_connection: Arc<DatabaseConnection>,
    pub iroh_endpoint: Endpoint,
}

impl AsRef<DatabaseConnection> for IpcActorContext {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.database_connection
    }
}

impl AsRef<Endpoint> for IpcActorContext {
    fn as_ref(&self) -> &Endpoint {
        &self.iroh_endpoint
    }
}