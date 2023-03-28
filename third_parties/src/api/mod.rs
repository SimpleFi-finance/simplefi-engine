/* use std::net::SocketAddr;
use tonic::{transport::Server, codegen::Service};

pub struct APIBuilder {
    server_builder: Server,
    pub addr: SocketAddr,
}

impl APIBuilder {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            server_builder: Server::builder(),
            addr,
        }
    }

    pub async fn server(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting gRPC server on {}", self.addr);

        self.server_builder.serve(self.addr).await?;
        Ok(())
    }

    pub fn add_service<S, R, B>(mut self, service: S) -> Self
    where
        S: tonic::transport::NamedService<
            ReqBody = tonic::codegen::http::Request<B>,
            ResBody = tonic::codegen::http::Response<R>,
        > + Send + Sync + 'static,
        R: tonic::codegen::Body + Send + Sync + 'static,
        B: tonic::codegen::Body + Send + Sync + 'static,
    {
        self.server_builder = self.server_builder.add_service(service);
        self
    }
}
 */
