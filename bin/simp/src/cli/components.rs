use simp_rpc_builder::{RpcServerHandle, SimpModuleRegistry, TransportRpcModules};

/// The trait that is implemented for the Node command.
pub trait SimpNodeComponents {}

/// Helper container to encapsulate [SimpModuleRegistry] and [TransportRpcModules].
#[derive(Debug)]
#[allow(clippy::type_complexity)]
pub struct SimpRpcComponents<'a> {
    /// A Helper type the holds instances of the configured modules.
    ///
    /// This provides easy access to rpc handlers
    pub registry: &'a mut SimpModuleRegistry<>,
    /// Holds installed modules per transport type.
    ///
    /// This can be used to merge additional modules into the configured transports (http, ws). See [TransportRpcModules::merge_configured]
    pub modules: &'a mut TransportRpcModules,
}
 
/// Contains the handles to the spawned RPC servers.
///
/// This can be used to access the endpoints of the servers.
///
/// # Example
///
/// ```rust
/// use simp::cli::components::SimpRpcServerHandles;
/// use simp::rpc::api::EthApiClient;
/// # async fn t(handles: SimpRpcServerHandles) {
///    let client = handles.rpc.http_client().expect("http server not started");
///    let block_number = client.block_number().await.unwrap();
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SimpRpcServerHandles {
    /// The regular RPC server handle.
    pub rpc: RpcServerHandle,
}
