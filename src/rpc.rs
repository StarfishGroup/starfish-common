use crate::env;
use anyhow::anyhow;
use futures::Future;
use inner::{
	starfish_client::StarfishClient,
	starfish_server::{Starfish, StarfishServer},
};
pub use inner::{StarfishCode, StarfishResult};
use prost_wkt_types::{Any, MessageSerde};
use std::{net::ToSocketAddrs, pin::Pin};
use tokio::sync::mpsc::{channel, Receiver};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{
	transport::{Channel, Server},
	Request, Response, Status, Streaming,
};

mod inner {
	tonic::include_proto!("starfish");
}

#[async_trait::async_trait]
pub trait CallHandle: Send + Sync + 'static {
	type Request: MessageSerde + Default;

	async fn call(&self, request: Self::Request) -> anyhow::Result<StarfishResult>;

	fn get_key() -> &'static str {
		Self::Request::default().type_url()
	}
}

#[async_trait::async_trait]
pub trait SubscribeHandle: Send + Sync + 'static {
	type Request: MessageSerde + Default;

	async fn call(&self, request: Self::Request) -> anyhow::Result<Receiver<StarfishResult>>;

	fn get_key() -> &'static str {
		Self::Request::default().type_url()
	}
}

#[async_trait::async_trait]
pub trait StreamHandle: Send + Sync + 'static {
	type Request: MessageSerde + Default;

	async fn call(
		&self,
		request: Receiver<Self::Request>,
	) -> anyhow::Result<Receiver<StarfishResult>>;

	fn get_key() -> &'static str {
		Self::Request::default().type_url()
	}
}

struct StarfishImpl {}

#[async_trait::async_trait]
impl Starfish for StarfishImpl {
	type StreamStream = Pin<Box<dyn Stream<Item = Result<StarfishResult, Status>> + Send>>;
	type SubscribeStream = Pin<Box<dyn Stream<Item = Result<StarfishResult, Status>> + Send>>;

	async fn call(&self, request: Request<Any>) -> Result<Response<StarfishResult>, Status> {
		todo!()
	}

	async fn subscribe(
		&self,
		request: Request<Any>,
	) -> Result<Response<Self::SubscribeStream>, Status> {
		todo!()
	}

	async fn stream(
		&self,
		request: Request<Streaming<Any>>,
	) -> Result<Response<Self::SubscribeStream>, Status> {
		todo!()
	}
}
