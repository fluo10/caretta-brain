use std::{sync::Arc, time::Duration};

use crate::{context::{ServiceContext, ServiceContextExt}, ipc::{DevicePingRequest, DevicePingResponse, IpcActorError, IpcApi, IpcApiTrait, IpcError, IpcMessage}, types::DeviceIdentifier};
use irpc::{Client, Service, WithChannels};
use n0_future::StreamExt;
use tracing::info;

pub struct IpcActor {
    recv: tokio::sync::mpsc::Receiver<IpcMessage>,
    context: Arc<dyn AsRef<ServiceContext> + Sync + Send>
}

impl IpcActor {
    pub fn spawn(context: &Arc<dyn AsRef<ServiceContext> + Sync + Send>) -> IpcApi {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let actor = Self {
            recv: rx,
            context: context.clone()
        };
        n0_future::task::spawn(actor.run());
        IpcApi {
            inner: Client::local(tx),
        }
    }
    async fn run(mut self) {
        while let Some(msg) = self.recv.recv().await {
            self.handle(msg).await;
        }
    }
    async fn handle(&mut self, msg: IpcMessage) {
        match msg {
            IpcMessage::DevicePing(ping) => {
                info!("ping {:?}", ping);
                let WithChannels{tx, inner, ..} = ping;
                let target = inner.target;

                if let Some(x) = tx.send(self.device_ping(target).await.map(|rtt| DevicePingResponse { rtt })).await.err() {
                    tracing::error!("{}", x)
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl IpcApiTrait for IpcActor {
    type Error = IpcActorError;
    async fn device_ping(&self, target: DeviceIdentifier) -> Result<Duration, Self::Error>{
                let public_key = target
            .to_public_key(&self.context)
            .await
            .map_err(|e| IpcActorError::Internal(format!("{:?}", e).to_string()))?
            .ok_or(IpcActorError::DeviceNotFound(target.clone()))?;
        let mut stream = self
            .context
            .as_ref()
            .discover(public_key.into_inner())
            .await
            .ok_or(IpcActorError::DeviceNotFound(target.clone()))?;
        if let Some(x) = stream.next().await {
            let discovered = x.map_err(|e| IpcActorError::Internal(format!("{:?}", e).to_string()))?;
            iroh_ping::Ping::new()
                .ping(
                    self.context.as_ref().as_endpoint().unwrap(),
                    discovered.into_endpoint_addr(),
                )
                .await
                .map_err(|e| IpcActorError::Internal(format!("{:?}", e)))
        } else {
            unreachable!()
        }
    }
}