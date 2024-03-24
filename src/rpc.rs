mod rpc {
    tonic::include_proto!("starfish");
}
// use futures::prelude::*;
// use prost_types::Any;
// use std::pin::Pin;
// use tonic::{Request, Response, Status, Streaming};

// #[derive(Default)]
// struct RpcServer;

// #[tonic::async_trait]
// impl rpc::api_server::Api for RpcServer {
//     type SubscribeStream = Pin<Box<dyn Stream<Item = Result<rpc::Result, Status>> + Send>>;
//     type ChannelStream = Pin<Box<dyn Stream<Item = Result<rpc::Result, Status>> + Send>>;

//     async fn call(&self, request: Request<Any>) -> Result<Response<rpc::Result>, Status> {
//         let a = request.into_inner();
//         Ok(Response::new(rpc::Result {
//             code: Some(rpc::Code {
//                 code: 0,
//                 msg: a.type_url,
//             }),
//             data: None,
//         }))
//     }

//     async fn subscribe(
//         &self,
//         request: Request<Any>,
//     ) -> Result<Response<Self::SubscribeStream>, Status> {
//         todo!()
//     }

//     async fn channel(
//         &self,
//         request: Request<Streaming<Any>>,
//     ) -> Result<Response<Self::ChannelStream>, Status> {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::{rpc::api_server::ApiServer, rpc::Code};
    use prost_wkt_types::*;
    use tonic::transport::Server;

    // #[tokio::test]
    // async fn test_server() -> Result<(), Box<dyn std::error::Error>> {
    //     let addr = "127.0.0.1:12345".parse().unwrap();
    //     let rpc = RpcServer::default();
    //     Server::builder()
    //         .add_service(ApiServer::new(rpc))
    //         .serve(addr)
    //         .await?;
    //     Ok(())
    // }

    #[tokio::test]
    async fn test_client() -> Result<(), Box<dyn std::error::Error>> {
        let code = Code {
            code: 1,
            msg: "666".to_owned(),
        };
        let a = code.type_url();
        println!("{}", a);
        let a = Any::try_pack(code)?;
        println!("{:?}", a);
        // let client = ApiClient::connect("127.0.0.1:12345").await?;

        // let c = Code::default();
        // let result = client.call(Request::new(Any::fr)).await?;
        Ok(())
    }
}
