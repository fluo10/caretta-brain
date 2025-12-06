use std::{marker::PhantomData, sync::LazyLock};

use crate::{config::StorageConfig, context::ServiceContext};
use tokio::sync::OnceCell;

const TEST_APP_NAME: &str = "caretta-brain-test";

pub static SERVICE_CONTEXT: OnceCell<ServiceContext> = OnceCell::const_new();
pub async fn service_conext() -> &'static ServiceContext {
    SERVICE_CONTEXT
        .get_or_init(|| async move {
            let test_dir = tempfile::Builder::new()
                .prefix(TEST_APP_NAME)
                .tempdir()
                .unwrap()
                .keep();
            let data_dir = test_dir.join("data");
            let cache_dir = test_dir.join("cache");
            let storage_config = StorageConfig {
                data_dir,
                cache_dir,
            };
            let database_connection = storage_config.to_database_connection();
            let iroh_router = None;
            ServiceContext {
                app_name: TEST_APP_NAME,
                storage_config,
                iroh_router,
                database_connection,
            }
        })
        .await
}

