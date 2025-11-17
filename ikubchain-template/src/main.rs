//! IkubChain Node main entry point.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

use sc_cli::{VersionInfo, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use ikubchain_template_runtime::{self, opaque::Block, RuntimeApi};
use log::info;

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

/// The version info for the IkubChain node.
pub const VERSION: VersionInfo = VersionInfo {
    name: "IkubChain Node",
    author: "IkubChain Team",
    description: "IkubChain Node Implementation",
    support_url: "https://github.com/ikubchain/ikubchain/issues",
    copyright_start_year: 2023,
    executable_name: "ikubchain",
    executable_description: "IkubChain Node",
    implementation_name: "ikubchain-node",
    spec_name: "ikubchain",
    spec_version: 1,
    impl_version: 1,
    authoring_version: 1,
    transaction_version: 1,
    state_version: 1,
};

/// The chain specification type.
pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

/// The extensions for the chain spec.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sc_chain_spec::ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// The relay chain of the parachain.
    pub relay_chain: String,
    /// The parachain ID.
    pub para_id: u32,
}

/// Parse and run command line arguments
fn run() -> sc_cli::Result<()> {
    let cli = cli::Cli::from_args();

    match &cli.subcommand {
        Some(subcommand) => {
            let runner = cli.create_runner(subcommand)?;
            runner.run_subcommand(subcommand, |config| {
                let PartialComponents { client, backend, task_manager, import_queue, .. } =
                    service::new_partial(&config)?;
                Ok((client, backend, import_queue, task_manager))
            })
        }
        None => {
            let runner = cli.create_runner(&cli.run.normalize())?;
            runner.run_node_until_exit(|config| async move {
                service::new_full(config).map_err(sc_cli::Error::Service)
            })
        }
    }
}

fn main() -> sc_cli::Result<()> {
    command::run()
}
