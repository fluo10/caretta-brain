use std::{net::SocketAddr, time::Duration};

use irpc::{Client, rpc::RemoteService};
use n0_future::task::{self, AbortOnDropHandle};
use quinn::Endpoint;

use crate::{ipc::{DevicePingRequest, DevicePingResponse, IpcApiTrait, IpcError, IpcProtocol, IpcResult}, types::DeviceIdentifier};

type IpcApiResult<T> = Result<T, IpcError>;
pub struct IpcApi {
    pub inner: Client<IpcProtocol>,
}

impl IpcApi {
    pub fn connect (endpoint: Endpoint, addr: SocketAddr) -> Result<Self, IpcError> {
        Ok(IpcApi {
            inner: Client::quinn(endpoint, addr)
        })
    } 
    pub fn listen(&self, endpoint: Endpoint) -> Result<AbortOnDropHandle<()>, IpcError> {
        let local = self
            .inner
            .as_local()
            .expect("cannot listen on remote API");
        let join_handle = task::spawn(irpc::rpc::listen(
            endpoint,
            IpcProtocol::remote_handler(local),
        ));
        Ok(AbortOnDropHandle::new(join_handle))
    }

}

#[async_trait::async_trait]
impl IpcApiTrait for IpcApi {
    type Error = IpcError;
    async fn device_ping(&self, target: DeviceIdentifier) -> IpcApiResult<Duration>  {
        Ok(self.inner.rpc(DevicePingRequest { target }).await??.rtt)
    }
}