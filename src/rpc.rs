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
	transport::{server::Router, Channel, Server},
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
	calls: HashMap<String, Box<dyn ICall<Request = T>>>,
}

impl<T: MessageSerde> StarfishCallImpl<T> {
	fn new() -> Self {
		Self { calls: Default::default() }
	}

	fn add_handle(mut self, call: impl ICall<Request = T>) -> Self {
		let key = call.default_request().type_url().to_owned();
		self.calls.insert(key, Box::new(call));
		self
	}
}

pub fn new_call<T: MessageSerde>() -> StarfishCallImpl<T> {
	StarfishCallImpl::<T>::new()
}

// fn new_call<T: MessageSerde>(ins: impl ICall<Request = T>) -> impl StarfishCall {
// 	StarfishCallImpl { ins: Box::new(ins) }
// }

#[async_trait::async_trait]
impl<T: MessageSerde> StarfishCall for StarfishCallImpl<T> {
	async fn call(&self, request: Request<Any>) -> Result<Response<StarfishResult>, Status> {
		let request_any_data = request.into_inner();
		let call = self.calls.get(&request_any_data.type_url);
		if call.is_none() {
			return Ok(Response::new(StarfishResult {
				code: Some(StarfishCode { code: 404, msg: "404".into() }),
				data: None,
			}))
		}
		let call = call.unwrap();
		let request_data = request_any_data.unpack_as(call.default_request());
		if let Err(err) = request_data {
			return Ok(Response::new(StarfishResult {
				code: Some(StarfishCode { code: 500, msg: err.to_string() }),
				data: None,
			}))
		}
		let request_data = request_data.unwrap();
		let result = call.call(request_data).await;
		if let Err(err) = result {
			if let Some(code) = err.downcast_ref::<StarfishCode>() {
				return Ok(Response::new(StarfishResult { code: Some(code.to_owned()), data: None }))
			}
			return Ok(Response::new(StarfishResult {
				code: Some(StarfishCode { code: 500, msg: err.to_string() }),
				data: None,
			}))
		}
		let result = result.unwrap();
		Ok(Response::new(result))
	}
}

pub async fn init_server<T: MessageSerde>(
	config: &env::Rpc,
	call: Option<StarfishCallImpl<T>>,
) -> anyhow::Result<()> {
	Server::builder()
		.add_optional_service(match call {
			Some(call) => Some(StarfishCallServer::new(call)),
			None => None,
		})
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
