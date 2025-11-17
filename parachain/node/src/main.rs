//! IkubChain Node

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

use sc_cli::{RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use ikub_chain_runtime::{
    self, opaque::Block, RuntimeApi,
};

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

/// The version info used for the IkubChain node.
pub const VERSION: RuntimeVersion = ikub_chain_runtime::VERSION;

/// The chain specification option. The standalone chain and the parachain have different
/// spec. The `load_spec` method will return one of them.
#[derive(Debug, clap::ValueEnum)]
enum ChainSpec {
    /// Whatever the current runtime is, with just Alice as an auth.
    Development,
    /// Whatever the current runtime is, with simple Alice/Bob auths.
    LocalTestnet,
    /// The IkubChain mainnet.
    IkubChain,
}

impl std::str::FromStr for ChainSpec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Development),
            "local" => Ok(Self::LocalTestnet),
            "ikub" => Ok(Self::IkubChain),
            _ => Err("Invalid chain spec".into()),
        }
    }
}

/// The `run` function is the main entry point for the node.
fn run() -> sc_cli::Result<()> {
    let cli = cli::Cli::from_args();

    match &cli.subcommand {
        Some(subcommand) => {
            command::run_subcommand(cli, subcommand)
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        // Simple test to verify the version is set
        assert_eq!(VERSION.spec_name, "ikubchain");
    }
}
