//! Command execution for the IkubChain node.

use sc_cli::{Result, CliConfiguration, ExecutionStrategy, WasmExecutionMethod};
use sc_service::{Configuration, ServiceBuilderCommand};
use sc_telemetry::{TelemetryHandle, TelemetryWorkerHandle};
use sc_network::config::NetworkConfiguration;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

use crate::{
    chain_spec,
    service::{new_full, new_light, new_partial, Executor},
    Cli, Subcommand,
};

/// Run the node with the given command-line arguments.
pub fn run() -> Result<()> {
    let cli = crate::cli::Cli::from_args();
    
    match &cli.subcommand {
        Some(Subcommand::Run(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.run_node_until_exit(|config| async move {
                match config.role {
                    sc_service::Role::Light => new_light(config).map_err(sc_cli::Error::Service),
                    _ => new_full(config).map_err(sc_cli::Error::Service),
                }
            })
        }
        _ => {
            // Handle other subcommands
            let runner = cli.create_runner(&cli.run)?;
            runner.run_subcommand(
                |config| {
                    let PartialComponents {
                        client,
                        backend,
                        task_manager,
                        import_queue,
                        ..
                    } = new_partial(&config)?;
                    Ok((client, backend, import_queue, task_manager))
                },
                |config, _| {
                    let (keep_alive, _) = futures::channel::oneshot::channel();
                    let task_manager = sc_service::TaskManager::new(
                        config.tokio_handle.clone(),
                        keep_alive,
                    )?;
                    Ok((config, task_manager))
                },
            )
        }
    }
}

/// Build the configuration for the node.
pub fn build_config(
    cli: &Cli,
    task_executor: &std::sync::Arc<dyn sc_service::TaskExecutor>,
) -> Result<Configuration> {
    let mut config = Configuration::default();
    
    // Set the chain specification
    config.chain_spec = cli.load_spec(&cli.run.shared_params.chain.clone().unwrap_or_default())?;
    
    // Set the base path
    config.base_path = cli.run.shared_params.base_path.clone()
        .or_else(|| Some(directories::BaseDirs::new()?.home_dir().join(".local/share/ikubchain")))
        .transpose()?;
    
    // Set the network configuration
    let network_config = NetworkConfiguration::new(
        "ikubchain-node",
        "ikubchain",
        Default::default(),
        None,
    );
    
    config.network = network_config;
    
    // Set the keystore configuration
    config.keystore = cli.run.keystore_params()?.keystore_config(
        &config.base_path,
        cli.run.shared_params.dev,
        &config.chain_spec,
    )?;
    
    // Set the database configuration
    config.database = cli.run.database_params()?.database_config(
        &config.base_path,
        cli.run.shared_params.dev,
    )?;
    
    // Set the transaction pool configuration
    config.transaction_pool = cli.run.transaction_pool_params()?.transaction_pool_config();
    
    // Set the RPC configuration
    config.rpc_http = cli.run.rpc_http(
        &config.network,
        cli.run.rpc_http_max_payload_in_mb,
        cli.run.rpc_http_max_connections,
    )?;
    
    config.rpc_ws = cli.run.rpc_ws(&config.network, cli.run.rpc_ws_max_connections)?;
    
    // Set the telemetry endpoints
    config.telemetry_endpoints = cli.run.telemetry_params()?.telemetry_endpoints()?;
    
    // Set the execution strategy
    config.wasm_method = cli.run.wasm_method()?;
    config.execution_strategies = cli.run.execution_strategies()?;
    
    // Set the role (full node, light client, etc.)
    config.role = cli.run.role()?;
    
    // Set the telemetry handle
    config.telemetry_handle = if !cli.run.no_telemetry {
        Some(TelemetryHandle::new(TelemetryWorkerHandle::new(
            config.telemetry_endpoints.clone(),
            config.telemetry_external_transport.clone(),
        )?))
    } else {
        None
    };
    
    Ok(config)
}
