//! Command-line interface for the IkubChain node.

use structopt::StructOpt;
use sc_cli::{
    RuntimeVersion, 
    SubstrateCli,
    Subcommand,
    CliConfiguration,
    SharedParams,
    RunCmd,
    ChainSpec,
    DatabaseParams,
    PruningParams,
    KeystoreParams,
    NetworkParams,
    TelemetryParams,
    TransactionPoolParams,
    OffchainWorkerParams,
    NodeKeyParams,
    RpcMethods,
    RPC_DEFAULT_PORT,
    RPC_DEFAULT_LISTEN_ADDRESS,
    RPC_DEFAULT_MAX_CONNECTIONS,
};

use crate::chain_spec;
use crate::VERSION;

/// Available subcommands for the IkubChain node.
#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Key management utilities.
    #[structopt(name = "key", about = "Key management utilities")]
    Key(sc_cli::KeySubcommand),

    /// Build the chain specification.
    #[structopt(name = "build-spec", about = "Build a chain specification")]
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    #[structopt(name = "check-block", about = "Validate blocks")]
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    #[structopt(name = "export-blocks", about = "Export blocks")]
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    #[structopt(name = "export-state", about = "Export the state of a given block into a chain spec")]
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    #[structopt(name = "import-blocks", about = "Import blocks")]
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    #[structopt(name = "purge-chain", about = "Remove the whole chain data")]
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    #[structopt(name = "revert", about = "Revert the chain to a previous state")]
    Revert(sc_cli::RevertCmd),

    /// Run the node.
    #[structopt(name = "run", about = "Run the node")]
    Run(RunCmd),
}

impl Subcommand {
    /// Get the run command if the subcommand is `run`.
    pub fn run(&self) -> Option<&RunCmd> {
        match self {
            Subcommand::Run(cmd) => Some(cmd),
            _ => None,
        }
    }
}

/// The `Cli` struct defines the command-line interface for the IkubChain node.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "ikubchain",
    about = "IkubChain Node",
    version = VERSION.name,
    author = VERSION.author,
    rename_all = "kebab-case"
)]
pub struct Cli {
    /// Enable verbose output.
    #[structopt(short, long, global = true)]
    pub verbose: bool,

    /// The subcommand to run.
    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,

    /// Run the node.
    #[structopt(flatten)]
    pub run: RunCmd,
}

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "IkubChain Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/ikubchain/ikubchain/issues".into()
    }

    fn copyright_start_year() -> i32 {
        2023
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()?),
            "local" => Box::new(chain_spec::local_testnet_config()?),
            "" | "mainnet" => Box::new(chain_spec::mainnet_config()?),
            path => Box::new(chain_spec::ChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }

    fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &ikubchain_runtime::VERSION
    }
}

/// Parse and run the command-line arguments.
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();
    
    match &cli.subcommand {
        Some(Subcommand::Key(cmd)) => cmd.run(&cli),
        Some(Subcommand::BuildSpec(cmd)) => cmd.run::<service::Executor>(&cli),
        Some(Subcommand::CheckBlock(cmd)) => cmd.run(&cli),
        Some(Subcommand::ExportBlocks(cmd)) => cmd.run(&cli),
        Some(Subcommand::ExportState(cmd)) => cmd.run(&cli),
        Some(Subcommand::ImportBlocks(cmd)) => cmd.run(&cli),
        Some(Subcommand::PurgeChain(cmd)) => cmd.run(&cli),
        Some(Subcommand::Revert(cmd)) => cmd.run(&cli),
        None => cli.run.run(),
    }
}
