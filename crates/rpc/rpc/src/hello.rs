use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use tokio::time::sleep;
use std::time::Duration;
use simp_rpc_api::HelloApiServer;
use tracing::{trace, debug};

pub struct HelloApi {   }

impl HelloApi {
    pub fn new() -> Self {
        HelloApi {  }
    }
}

#[async_trait]
impl HelloApiServer for HelloApi {
    fn say_hello(&self, name: String) -> RpcResult<String> {
        trace!("hello:say_hello called");
        debug!("hello:say_hello called");

        Ok(format!("Hello, {}!", name))
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    async fn wait_hello(&self, name: String) -> RpcResult<String> {
        trace!("hello:say_hello called");
        debug!("hello:wait_hello waiting");

        // delay 2 seconds
        sleep(Duration::from_secs(2)).await;
        

        Ok(format!("Thanks for wait for me, {}!", name))
    }

    /* async fn say_hello(&self, name: String) -> RpcResult<String> {
        Ok(format!("Hello, {}!", name))
    } */
}
