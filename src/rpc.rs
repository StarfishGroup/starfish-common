mod rpc {
    tonic::include_proto!("starfish");
}
use anyhow::anyhow;
use prost_wkt_types::Any;
use std::{borrow::Cow, net::ToSocketAddrs, pin::Pin};
use tokio::sync::mpsc::{channel, Receiver};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{
    transport::{Channel, Server},
    Request, Response, Status, Streaming,
};

use crate::env;

use self::rpc::api_client::ApiClient;

#[async_trait::async_trait]
pub trait ServerHandle {
    async fn call(&self, request: Any) -> anyhow::Result<rpc::Result>;
    async fn subscribe(&self, request: Any) -> anyhow::Result<Receiver<rpc::Result>>;
    async fn channel(&self, request: Receiver<Any>) -> anyhow::Result<Receiver<rpc::Result>>;
}

struct RpcServer {
    handle: Box<dyn ServerHandle + Send + Sync>,
}

impl RpcServer {
    pub fn new(handle: Box<dyn ServerHandle + Send + Sync>) -> Self {
        Self { handle }
    }
}

#[async_trait::async_trait]
impl rpc::api_server::Api for RpcServer {
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<rpc::Result, Status>> + Send>>;
    type ChannelStream = Pin<Box<dyn Stream<Item = Result<rpc::Result, Status>> + Send>>;

    async fn call(&self, request: Request<Any>) -> Result<Response<rpc::Result>, Status> {
        let result = self
            .handle
            .call(request.into_inner())
            .await
            .map_err(|err| Status::new(tonic::Code::Internal, err.to_string()))?;
        Ok(Response::new(result))
    }

    async fn subscribe(
        &self,
        request: Request<Any>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let mut result = self
            .handle
            .subscribe(request.into_inner())
            .await
            .map_err(|err| Status::new(tonic::Code::Internal, err.to_string()))?;
        let (tx, rx) = channel(1024);
        tokio::spawn(async move {
            while let Some(item) = result.recv().await {
                if tx.send(Ok(item)).await.is_err() {
                    break;
                }
            }
            tx.closed().await;
        });
        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::SubscribeStream
        ))
    }

    async fn channel(
        &self,
        request: Request<Streaming<Any>>,
    ) -> Result<Response<Self::ChannelStream>, Status> {
        let (tx, rx) = channel(1024);
        let mut request = request.into_inner();
        let _tx = tx.clone();
        tokio::spawn(async move {
            while let Some(item) = request.next().await {
                if let Ok(item) = item {
                    if _tx.send(item).await.is_err() {
                        break;
                    }
                }
            }
            _tx.closed().await;
        });
        let mut result = self
            .handle
            .channel(rx)
            .await
            .map_err(|err| Status::new(tonic::Code::Internal, err.to_string()))?;
        let (tx, rx) = channel(1024);
        tokio::spawn(async move {
            while let Some(item) = result.recv().await {
                if tx.send(Ok(item)).await.is_err() {
                    break;
                }
            }
            tx.closed().await;
        });
        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::SubscribeStream
        ))
    }
}

pub async fn init_server(
    config: &env::Rpc,
    handle: Box<dyn ServerHandle + Send + Sync>,
) -> anyhow::Result<()> {
    let rpc_server = RpcServer::new(handle);

    Server::builder()
        .add_service(rpc::api_server::ApiServer::new(rpc_server))
        .serve(
            config
                .bind
                .to_socket_addrs()?
                .next()
                .ok_or(anyhow!("parse address failed : {}", config.bind))?,
        )
        .await?;

    Ok(())
}

pub async fn init_client(url: String) -> anyhow::Result<ApiClient<Channel>> {
    let client = ApiClient::connect(url).await?;
    Ok(client)
}
