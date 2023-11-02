use jsonrpsee::{core::RpcResult, proc_macros::rpc};

/// Hello namespace rpc interface that gives access to several non-standard RPC methods.
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "hello"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "hello"))]
#[async_trait::async_trait]
pub trait HelloApi {
    #[method(name = "sayHello")]
    fn say_hello(&self, name: String) -> RpcResult<String>;

    #[method(name = "waitHello")]
    async fn wait_hello(&self, name: String) -> RpcResult<String>;
}