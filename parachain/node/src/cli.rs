//! Command-line interface for the IkubChain node.

use sc_cli::{
    ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
    NetworkParams, Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::config::PrometheusConfig;
use std::path::PathBuf;

use crate::chain_spec;
use crate::VERSION;

/// Available command-line options for the IkubChain node.
#[derive(Debug, clap::Parser)]
pub struct Cli {
    /// The command to execute.
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[clap(flatten)]
    pub run: RunCmd,
}

/// Available subcommands for the IkubChain node.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Sub-commands concerned with benchmarking.
    #[clap(subcommand)]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}

/// The `run` command used to run a node.
#[derive(Debug, clap::Parser)]
pub struct RunCmd {
    /// The shared parameters
    #[clap(flatten)]
    pub shared_params: SharedParams,

    /// The custom base path, if any
    #[clap(long = "base-path", value_name = "PATH")]
    pub base_path: Option<PathBuf>,

    /// The chain specification to use
    #[clap(long, value_name = "CHAIN_SPEC")]
    pub chain: Option<String>,

    /// Specify the development chain
    #[clap(long)]
    pub dev: bool,

    /// Disable the automatic hardware benchmarks
    #[clap(long)]
    pub no_hardware_benchmarks: bool,

    /// The custom keystore path, if any
    #[clap(long = "keystore-path", value_name = "PATH")]
    pub keystore_path: Option<PathBuf>,

    /// The custom node key type ID
    #[clap(long = "node-key-type", value_name = "TYPE")]
    pub node_key_type: Option<String>,

    /// The custom node key seed
    #[clap(long = "node-key-seed", value_name = "SEED")]
    pub node_key_seed: Option<String>,

    /// The custom node key file
    #[clap(long = "node-key-file", value_name = "FILE")]
    pub node_key_file: Option<PathBuf>,
}

impl Default for RunCmd {
    fn default() -> Self {
        Self {
            shared_params: Default::default(),
            base_path: None,
            chain: None,
            dev: false,
            no_hardware_benchmarks: false,
            keystore_path: None,
            node_key_type: None,
            node_key_seed: None,
            node_key_file: None,
        }
    }
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "IkubChain Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        "IkubChain Node".into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/yourusername/ikub-chain/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2025
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        let spec = match id {
            "dev" => chain_spec::development_config(),
            "local" => chain_spec::local_testnet_config(),
            "ikub" => chain_spec::ikub_chain_config(),
            path => {
                let path = PathBuf::from(path);
                if path.exists() {
                    Ok(chain_spec::ChainSpec::from_json_file(path)?)
                } else {
                    Err(format!("Chain spec not found: {}", path.display()))
                }
            }
        }?;
        Ok(Box::new(spec))
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &VERSION
    }
}

impl CliConfiguration for RunCmd {
    fn shared_params(&self) -> &SharedParams {
        &self.shared_params
    }

    fn import_params(&self) -> Option<&ImportParams> {
        Some(&self.shared_params.import_params()?)
    }

    fn network_params(&self) -> Option<&NetworkParams> {
        Some(&self.shared_params.network_params()?)
    }

    fn keystore_params(&self) -> Option<&KeystoreParams> {
        Some(&self.shared_params.keystore_params()?)
    }

    fn base_path(&self) -> Result<Option<&std::path::Path>> {
        Ok(match &self.base_path {
            Some(path) => Some(path.as_path()),
            None => None,
        })
    }

    fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<std::net::SocketAddr>> {
        self.shared_params.rpc_addr(default_listen_port)
    }

    fn prometheus_config(
        &self,
        default_listen_port: u16,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<PrometheusConfig>> {
        self.shared_params
            .prometheus_config(default_listen_port, chain_spec.network())
    }

    fn chain_id(&self, is_dev: bool) -> Result<String> {
        let chain_id = match (self.chain.as_ref().map(|c| c.as_str()), self.dev, is_dev) {
            (Some(chain), _, _) => chain.to_string(),
            (_, true, _) => "dev".to_string(),
            (_, _, true) => "dev".to_string(),
            _ => "ikub".to_string(),
        };
        Ok(chain_id)
    }

    fn role(&self, _is_dev: bool) -> Result<sc_service::Role> {
        Ok(sc_service::Role::Full)
    }

    fn transaction_pool(&self) -> Result<sc_service::config::TransactionPoolOptions> {
        self.shared_params.transaction_pool()
    }

    fn state_cache_child_ratio(&self) -> Result<Option<usize>> {
        self.shared_params.state_cache_child_ratio()
    }

    fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
        self.shared_params.rpc_methods()
    }

    fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
        self.shared_params.rpc_ws_max_connections()
    }

    fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
        self.shared_params.rpc_cors(is_dev)
    }

    fn default_heap_pages(&self) -> Result<Option<u64>> {
        self.shared_params.default_heap_pages()
    }

    fn force_authoring(&self) -> Result<bool> {
        self.shared_params.force_authoring()
    }

    fn disable_grandpa(&self) -> Result<bool> {
        self.shared_params.disable_grandpa()
    }

    fn dev_key_seed(&self, is_dev: bool) -> Result<Option<String>> {
        self.shared_params.dev_key_seed(is_dev)
    }

    fn telemetry_endpoints(
        &self,
        chain_spec: &Box<dyn ChainSpec>,
    ) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
        self.shared_params.telemetry_endpoints(chain_spec)
    }

    fn node_name(&self) -> Result<String> {
        self.shared_params.node_name()
    }

    fn wasm_method(&self) -> Result<sc_service::WasmExecutionMethod> {
        self.shared_params.wasm_method()
    }

    fn wasm_runtime_overrides(&self) -> Result<Option<PathBuf>> {
        self.shared_params.wasm_runtime_overrides()
    }

    fn tracing_receiver(&self) -> sc_tracing::TracingReceiver {
        self.shared_params.tracing_receiver()
    }

    fn tracing_targets(&self) -> Result<Option<String>> {
        self.shared_params.tracing_targets()
    }

    fn release_spec(&self) -> Result<bool> {
        self.shared_params.release_spec()
    }
}
