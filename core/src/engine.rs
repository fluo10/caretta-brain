use std::sync::Arc;

use iroh::protocol::Router;
use irpc::util::make_server_endpoint;
#[cfg(feature = "desktop")]
use n0_future::task::AbortOnDropHandle;
use sea_orm::DatabaseConnection;

use crate::{config::{IpcConfig, LogConfig, P2pConfig, StorageConfig}, ipc::{IpcActor, IpcActorContext}};

pub struct Engine{
    iroh_router: Router,
    #[cfg(feature = "desktop")]
    ipc_server: AbortOnDropHandle<()>,
    
}

impl Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }
    async fn shutdown(self) -> Result<(), n0_future::task::JoinError>{
        self.iroh_router.shutdown().await
    }
}

#[derive(Debug, Default)]
pub struct EngineBuilder {
    ipc_config: Option<IpcConfig>,
    log_config: Option<LogConfig>,
    p2p_config: Option<P2pConfig>,
    storage_config: Option<StorageConfig>,
    database_connection: Option<Arc<DatabaseConnection>>,
}

impl EngineBuilder {
    pub fn ipc_config(mut self, config: IpcConfig) -> Self {
        self.ipc_config.insert(config);
        self
    }
    pub fn log_config(mut self, config: LogConfig) -> Self {
        self.log_config.insert(config);
        self
    }  
    pub fn p2p_config(mut self, config: P2pConfig) -> Self {
        self.p2p_config.insert(config);
        self
    }
    pub fn storage(mut self, config: StorageConfig) -> Self {
        self.storage_config.insert(config);
        self
    }
    pub fn load_db(mut self) -> Self {
        todo!()
    }
    pub async fn spawn(self) -> Engine{
        let (server, cert) = make_server_endpoint(self.ipc_config.unwrap().endpoint.clone()).unwrap();
        let actor = IpcActor::spawn(IpcActorContext {
            database_connection: self.database_connection.unwrap().clone(),
            iroh_endpoint: self.p2p_config.unwrap().spawn_iroh_endpoint().await.unwrap()
        });
        let handle = actor.listen(server).unwrap();
        todo!()
    }
}
