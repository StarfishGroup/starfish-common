use crate::env;
use anyhow::anyhow;
use derive_more::Error;
use futures::Future;
use inner::starfish_call_server::{StarfishCall, StarfishCallServer};
pub use inner::{StarfishCode, StarfishResult};
use prost_wkt_types::{Any, MessageSerde};
use std::{collections::HashMap, net::ToSocketAddrs, pin::Pin};
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
	type Request: MessageSerde;
	async fn call(&self, request: Self::Request) -> anyhow::Result<StarfishResult>;
	fn default_request(&self) -> Self::Request;
}

pub struct StarfishCallImpl<T: MessageSerde> {
	call_impl: Box<dyn CallHandle<Request = T>>,
}

#[async_trait::async_trait]
impl<T: MessageSerde> StarfishCall for StarfishCallImpl<T> {
	async fn call(&self, request: Request<Any>) -> Result<Response<StarfishResult>, Status> {
		let default_request = self.call_impl.default_request();
		let request_data = request.into_inner();
		let request_data = request_data
			.unpack_as(default_request)
			.map_err(|err| Status::unknown(err.to_string()))?;
		let result = self
			.call_impl
			.call(request_data)
			.await
			.map_err(|err| Status::unknown(err.to_string()))?;
		Ok(Response::new(result))
	}
}

// #[async_trait::async_trait]
// pub trait SubscribeHandle: Send + Sync + 'static {
// 	type Request: MessageSerde;
// 	async fn call(&self, request: Self::Request) -> anyhow::Result<Receiver<StarfishResult>>;
// 	fn get_type(&self) -> Self::Request;
// }

// #[async_trait::async_trait]
// pub trait StreamHandle: Send + Sync + 'static {
// 	type Request: MessageSerde;
// 	async fn call(
// 		&self,
// 		request: Receiver<Self::Request>,
// 	) -> anyhow::Result<Receiver<StarfishResult>>;
// 	fn get_type(&self) -> Self::Request;
// }

// struct StarfishImpl {
// 	call_handles: HashMap<String, Box<dyn CallHandle>>,
// }

// impl StarfishImpl {
// 	pub fn new() -> Self {
// 		Self { call_handles: Default::default() }
// 	}

// 	pub fn register_call(&mut self, call: Box<dyn CallHandle<Request = Box<dyn MessageSerde>>>) {
// 		self.call_handles.insert("".into(), call);
// 	}
// }

// #[async_trait::async_trait]
// impl Starfish for StarfishImpl {
// 	type StreamStream = Pin<Box<dyn Stream<Item = Result<StarfishResult, Status>> + Send>>;
// 	type SubscribeStream = Pin<Box<dyn Stream<Item = Result<StarfishResult, Status>> + Send>>;

// 	async fn call(&self, request: Request<Any>) -> Result<Response<StarfishResult>, Status> {
// 		let request_data = request.into_inner();
// 		let call = self.call_handles.get(&request_data.type_url);
// 		if call.is_none() {
// 			return Err(Status::unimplemented("not support"));
// 		}
// 		let call = call.unwrap();
// 		let mut t = call.get_type();
// 		request_data.unpack_as(t);
// 		call.call(request_data);
// 		todo!()
// 	}

// 	async fn subscribe(
// 		&self,
// 		request: Request<Any>,
// 	) -> Result<Response<Self::SubscribeStream>, Status> {
// 		todo!()
// 	}

// 	async fn stream(
// 		&self,
// 		request: Request<Streaming<Any>>,
// 	) -> Result<Response<Self::SubscribeStream>, Status> {
// 		todo!()
// 	}
// }
