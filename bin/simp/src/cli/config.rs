use simp_rpc_builder::{ServerBuilder, RpcServerConfig};

/// A trait that provides configured RPC server.
///
/// This provides all basic config values for the RPC server and is implemented by the
/// [RpcServerArgs](crate::args::RpcServerArgs) type.
pub trait SimpRpcConfig {
    /// Returns the max request size in bytes.
    fn rpc_max_request_size_bytes(&self) -> u32;

    /// Returns the max response size in bytes.
    fn rpc_max_response_size_bytes(&self) -> u32;

    /// Returns the default server builder for http/ws
    fn http_ws_server_builder(&self) -> ServerBuilder;

    /// Creates the [RpcServerConfig] from cli args.
    fn rpc_server_config(&self) -> RpcServerConfig;
}
