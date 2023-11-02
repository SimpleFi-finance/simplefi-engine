use std::net::SocketAddr;
use clap::Parser;
use metrics_exporter_prometheus::PrometheusHandle;
use tracing::{debug, info};
use crate::{prometheus_exporter, runner::CliContext, args::{RpcServerArgs, utils::parse_socket_address}};

#[derive(Debug, Parser)]
pub struct ServerCommand {
    /// All rpc related arguments
    #[clap(flatten)]
    pub rpc: RpcServerArgs,

    /// Enable Prometheus metrics.
    ///
    /// The metrics will be served at the given interface and port.
    #[arg(long, value_name = "SOCKET", value_parser = parse_socket_address, help_heading = "Metrics")]
    pub metrics: Option<SocketAddr>,
}

impl ServerCommand {
    pub async fn execute(self, _ctx: CliContext) -> eyre::Result<()> {
        // Read config
        debug!("Read server config");
        // let mut config: Config = self.load_config(config_path.clone())?;

        // Database config
        debug!("Database config: {:?}", self);

        let prometheus_handle = self.install_prometheus_recorder()?;
        
        self.start_metrics_endpoint(prometheus_handle /* Arc::clone(&db) */).await?;

        // Start metrics
        // todo!();

        // Start server


        /* // adjust rpc port numbers based on instance number
        self.adjust_instance_ports(); */

        // Start RPC server
        let _rpc_server_handles = self.rpc.start_servers().await?;

        /* // Start RPC servers
        let _rpc_server_handles =
            self.rpc.start_servers(&components, engine_api, jwt_secret, &mut self.ext).await?; */

        // Wait for exit
        info!("Wait for exit");

        /* if self.debug.terminate {
            Ok(())
        } else { */
            // The pipeline has finished downloading blocks up to `--debug.tip` or
            // `--debug.max-block`. Keep other node components alive for further usage.
            futures::future::pending().await
        /* } */
    }

    fn install_prometheus_recorder(&self) -> eyre::Result<PrometheusHandle> {
        prometheus_exporter::install_recorder()
    }

    async fn start_metrics_endpoint(
        &self,
        prometheus_handle: PrometheusHandle
        /* , db: Arc<DatabaseEnv> */) -> eyre::Result<()> {
        if let Some(listen_addr) = self.metrics {
            info!(target: "simp::cli", addr = %listen_addr, "Starting metrics endpoint");

            prometheus_exporter::serve(
                listen_addr, 
                prometheus_handle,
                /* db, */ 
                metrics_process::Collector::default()
            )
            .await?;
        }

        Ok(())
    }

    /* /// Change rpc port numbers based on the instance number.
    fn adjust_instance_ports(&mut self) {
        // auth port is scaled by a factor of instance * 100
        /* self.rpc.auth_port += self.instance * 100 - 100; */
        // http port is scaled by a factor of -instance
        self.rpc.http_port -= self.instance - 1;
        // ws port is scaled by a factor of instance * 2
        /* self.rpc.ws_port += self.instance * 2 - 2; */
    } */
}
