use clap::Args;
use futures::TryFutureExt;
use tracing::{info, debug};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use simp_rpc_builder::{constants, RpcServerConfig, ServerBuilder, RpcModuleBuilder, TransportRpcModuleConfig, RpcModuleConfig, RpcModuleSelection};
use crate::cli::{config::SimpRpcConfig, components::{SimpRpcServerHandles, SimpRpcComponents}};

/// Default max number of subscriptions per connection.
pub(crate) const RPC_DEFAULT_MAX_SUBS_PER_CONN: u32 = 1024;
/// Default max request size in MB.
pub(crate) const RPC_DEFAULT_MAX_REQUEST_SIZE_MB: u32 = 15;
/// Default max response size in MB.
///
/// This is only relevant for very large trace responses.
pub(crate) const RPC_DEFAULT_MAX_RESPONSE_SIZE_MB: u32 = 115;
/// Default number of incoming connections.
pub(crate) const RPC_DEFAULT_MAX_CONNECTIONS: u32 = 500;

/// Parameters for configuring the rpc more granularity via CLI
#[derive(Debug, Clone, Args)]
#[command(next_help_heading = "RPC")]
pub struct RpcServerArgs {
    /// Enable the HTTP-RPC server
    #[arg(long, default_value_if("dev", "true", "true"))]
    pub http: bool,

    /// Http server address to listen on
    #[arg(long = "http.addr", default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub http_addr: IpAddr,

    /// Http server port to listen on
    #[arg(long = "http.port", default_value_t = constants::DEFAULT_HTTP_RPC_PORT)]
    pub http_port: u16,

    /* 
    /// Rpc Modules to be configured for the HTTP server
    #[arg(long = "http.api", value_parser = RpcModuleSelectionValueParser::default())]
    pub http_api: Option<RpcModuleSelection>, 
    
    /// Http Corsdomain to allow request from
    #[arg(long = "http.corsdomain")]
    pub http_corsdomain: Option<String>,
    */

    /// Enable the WS-RPC server
    #[arg(long)]
    pub ws: bool,

    /// Ws server address to listen on
    #[arg(long = "ws.addr", default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST))]
    pub ws_addr: IpAddr,

    /// Ws server port to listen on
    #[arg(long = "ws.port", default_value_t = constants::DEFAULT_WS_RPC_PORT)]
    pub ws_port: u16,
/* 
    /// Origins from which to accept WebSocket requests
    #[arg(long = "ws.origins", name = "ws.origins")]
    pub ws_allowed_origins: Option<String>,

    /// Rpc Modules to be configured for the WS server
    #[arg(long = "ws.api", value_parser = RpcModuleSelectionValueParser::default())]
    pub ws_api: Option<RpcModuleSelection>, */

    /// Set the maximum RPC request payload size for both HTTP and WS in megabytes.
    #[arg(long, default_value_t = RPC_DEFAULT_MAX_REQUEST_SIZE_MB)]
    pub rpc_max_request_size: u32,

    /// Set the maximum RPC response payload size for both HTTP and WS in megabytes.
    #[arg(long, visible_alias = "--rpc.returndata.limit", default_value_t = RPC_DEFAULT_MAX_RESPONSE_SIZE_MB)]
    pub rpc_max_response_size: u32,

    /// Set the the maximum concurrent subscriptions per connection.
    #[arg(long, default_value_t = RPC_DEFAULT_MAX_SUBS_PER_CONN)]
    pub rpc_max_subscriptions_per_connection: u32,

    /// Maximum number of RPC server connections.
    #[arg(long, value_name = "COUNT", default_value_t = RPC_DEFAULT_MAX_CONNECTIONS)]
    pub rpc_max_connections: u32,
}

impl RpcServerArgs {
    pub async fn start_servers(&self) -> eyre::Result<SimpRpcServerHandles> {
        debug!("Start server in RpcServerArgs");

        let module_config = self.transport_rpc_module_config();

        let (mut modules, mut registry) = RpcModuleBuilder::default()
            .build_with_auth_server(module_config);

        let server_config = self.rpc_server_config();

        debug!("***Server config: {:?}", server_config);

        debug!("----  modules: {:?}", modules);

        let launch_rpc = modules.clone().start_server(server_config).map_ok(|handle| {           
            debug!("**** HANDLE: {:?}", handle);

            if let Some(addr) = handle.http_local_addr() {
                info!(target: "simp::cli", url=%addr, "RPC HTTP server started");
            }
            if let Some(addr) = handle.ws_local_addr() {
                info!(target: "simp::cli", url=%addr, "RPC WS server started");
            }
            handle
        });

        let rpc = launch_rpc.await?;

        let handles = SimpRpcServerHandles { rpc };

        let _rpc_components = SimpRpcComponents { registry: &mut registry, modules: &mut modules };

        /* conf.on_rpc_server_started(self, components, rpc_components, handles.clone())?; */

        Ok(handles)
    }

    fn transport_rpc_module_config(&self) -> TransportRpcModuleConfig {
        let mut config = TransportRpcModuleConfig::default()
            .with_config(RpcModuleConfig::new());

        if self.http {
            config = config.with_http(
                RpcModuleSelection::standard_modules(),
                /* self.http_api
                    .clone()
                    .unwrap_or_else(|| RpcModuleSelection::standard_modules().into()), */
            );
        }

        if self.ws {
            config = config.with_ws(
                RpcModuleSelection::standard_modules(),
                /* self.ws_api
                    .clone()
                    .unwrap_or_else(|| RpcModuleSelection::standard_modules().into()), */
            );
        }

        config
    }
}

impl SimpRpcConfig for RpcServerArgs {
    fn http_ws_server_builder(&self) -> simp_rpc_builder::ServerBuilder {
        ServerBuilder::new()
            .max_connections(self.rpc_max_connections)
            .max_request_body_size(self.rpc_max_request_size_bytes())
            .max_response_body_size(self.rpc_max_response_size_bytes())
            .max_subscriptions_per_connection(self.rpc_max_subscriptions_per_connection)
    }

    fn rpc_server_config(&self) -> RpcServerConfig {
        let mut config = RpcServerConfig::default();

        debug!("RpcServerConfig: {:?}", config);

        if self.http {
            let socket_address = SocketAddr::new(self.http_addr, self.http_port);

            debug!("Socket Address: {:?}", socket_address);

            config = config
                .with_http_address(socket_address)
                .with_http(self.http_ws_server_builder())
        }

        debug!("RpcServerConfig if http: {:?}", config);

        if self.ws {
            let socket_address = SocketAddr::new(self.ws_addr, self.ws_port);
            config = config.with_ws_address(socket_address).with_ws(self.http_ws_server_builder());
        }

        config 
    }

    fn rpc_max_request_size_bytes(&self) -> u32 {
        self.rpc_max_request_size * 1024 * 1024
    }

    fn rpc_max_response_size_bytes(&self) -> u32 {
        self.rpc_max_response_size * 1024 * 1024
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::net::SocketAddrV4;

    /// A helper type to parse Args more easily
    #[derive(Parser)]
    struct CommandParser<T: Args> {
        #[clap(flatten)]
        args: T,
    }

    #[test]
    fn test_rpc_server_config() {
        let args = CommandParser::<RpcServerArgs>::parse_from([
            "simp",
            /* "--http.api",
            "eth,admin,debug", */
            "--http",
            "--ws",
            "--ws.addr",
            "127.0.0.1",
            "--ws.port",
            "8888",
        ])
        .args;
        let config = args.rpc_server_config();
        assert_eq!(
            config.http_address().unwrap(),
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::LOCALHOST,
                constants::DEFAULT_HTTP_RPC_PORT
            ))
        );
        assert_eq!(
            config.ws_address().unwrap(),
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8888))
        );
    }
}