use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8000".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .concurrency_limit_per_connection(64) // Optimize for higher concurrency
        .http2_keepalive_interval(Some(std::time::Duration::from_secs(30))) // Persistent connection tuning
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

// use tokio::sync::Semaphore;
// use tonic::codegen::InterceptedService;
// use tonic::{transport::Server, Request, Response, Status};

// pub mod greeter {
//     tonic::include_proto!("greeter"); // Match the proto package name
// }

// use greeter::{
//     greeter_server::{Greeter, GreeterServer},
//     HelloReply, HelloRequest,
// };

// #[derive(Debug, Default)]
// pub struct GreeterService {
//     // Semaphore to limit concurrency
//     semaphore: tokio::sync::Semaphore,
// }

// #[tonic::async_trait]
// impl Greeter for GreeterService {
//     async fn say_hello(
//         &self,
//         request: Request<HelloRequest>,
//     ) -> Result<Response<HelloReply>, Status> {
//         let permit = self.semaphore.acquire().await?;
//         let name = request.into_inner().name;

//         // Simulate async processing
//         tokio::time::sleep(std::time::Duration::from_millis(100)).await;

//         let reply = HelloReply {
//             message: format!("Hello, {}!", name),
//         };

//         drop(permit); // Release semaphore
//         Ok(Response::new(reply))
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "127.0.0.1:50051".parse()?;
//     let greeter = GreeterService {
//         semaphore: Semaphore::new(64), // Limit concurrency to 64
//     };

//     println!("GreeterServer listening on {}", addr);

//     Server::builder()
//         // .concurrency_limit_per_connection(64) // Optimize for higher concurrency
//         // .http2_keepalive_interval(std::time::Duration::as_secs(30)) // Persistent connection tuning
//         .compression(tonic::codec::CompressionEncoding::Gzip) // Enable compression
//         .add_service(GreeterServer::new(greeter))
//         .serve(addr)
//         .await?;

//     Ok(())
// }
