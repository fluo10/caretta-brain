use std::time::Duration;

use crate::{ipc::{DevicePingRequest, DevicePingResponse}, types::DeviceIdentifier};

#[async_trait::async_trait]
pub trait IpcApiTrait{
    type Error;
    async fn device_ping(&self, target: DeviceIdentifier) -> Result<Duration, Self::Error>;
}