use std::{marker::PhantomData, pin::Pin};

use iroh::{
    Endpoint,
    discovery::{ConcurrentDiscovery, Discovery, DiscoveryError, DiscoveryItem},
    protocol::Router,
};
use n0_future::Stream;
use sea_orm::DatabaseConnection;

use crate::config::{LogConfig, P2pConfig, StorageConfig};

/// An extension trait for [`ServiceContext`]
pub trait ServiceContextExt {
    fn as_iroh_router(&self) -> &Router;
    fn as_endpoint(&self) -> &Endpoint {
        self.as_iroh_router().endpoint()
    }
    fn as_discovery(&self) -> &ConcurrentDiscovery {
        self.as_endpoint().discovery()
    }
    async fn discover(
        &self,
        endpoint_id: iroh::EndpointId,
    ) -> Option<
        Pin<
            Box<
                dyn Stream<Item = Result<DiscoveryItem, DiscoveryError>>
                    + std::marker::Send
                    + 'static,
            >,
        >,
    > {
        self.as_discovery().resolve(endpoint_id)
    }
    fn as_database_connection(&self) -> &DatabaseConnection;

}

impl<T> ServiceContextExt for T
where
    T: AsRef<ServiceContext>,
{
    fn as_iroh_router(&self) -> &Router {
        self.as_ref().as_iroh_router()
    }
    fn as_database_connection(&self) -> &DatabaseConnection {
        self.as_ref().as_database_connection()
    }
}

/// A context for background process
#[derive(Debug)]
pub struct ServiceContext {
    pub app_name: &'static str,
    pub storage_config: StorageConfig,
    pub database_connection: DatabaseConnection,
    pub iroh_router: Router,
}
impl ServiceContextExt for ServiceContext {
    fn as_iroh_router(&self) -> &Router {
        &self.iroh_router
    }
    fn as_database_connection(&self) -> &DatabaseConnection {
        &self.database_connection
    }
}