use crate::env;
use anyhow::anyhow;
use derive_more::Error;
use futures::Future;
use inner::starfish_call_server::{StarfishCall, StarfishCallServer};
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
pub trait ICall: Send + Sync + 'static {
	type Request: MessageSerde;

	async fn call(&self, request: Self::Request) -> anyhow::Result<StarfishResult>;

	fn default_request(&self) -> Self::Request;
}

struct StarfishCallImpl<T: MessageSerde> {
	ins: Box<dyn ICall<Request = T>>,
}

#[async_trait::async_trait]
impl<T: MessageSerde> StarfishCall for StarfishCallImpl<T> {
	async fn call(&self, request: Request<Any>) -> Result<Response<StarfishResult>, Status> {
		let default_request = self.ins.default_request();
		let request_data = request.into_inner();
		let request_data = request_data.unpack_as(default_request);
		if let Err(err) = request_data {
			return Ok(Response::new(StarfishResult {
				code: Some(StarfishCode { code: 2, msg: err.to_string() }),
				data: None,
			}))
		}
		let request_data = request_data.unwrap();
		let result = self.ins.call(request_data).await;
		if let Err(err) = result {
			if let Some(code) = err.downcast_ref::<StarfishCode>() {
				return Ok(Response::new(StarfishResult { code: Some(code.to_owned()), data: None }))
			}
			return Ok(Response::new(StarfishResult {
				code: Some(StarfishCode { code: 1, msg: err.to_string() }),
				data: None,
			}))
		}
		let result = result.unwrap();
		Ok(Response::new(result))
	}
}

pub fn new_call<T: MessageSerde>(ins: impl ICall<Request = T>) -> impl StarfishCall {
	StarfishCallImpl { ins: Box::new(ins) }
}
