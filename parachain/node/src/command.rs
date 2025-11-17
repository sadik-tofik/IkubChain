//! Command line argument parsing and execution.

use crate::cli::{Cli, Subcommand};
use sc_cli::{
    CliConfiguration, DefaultConfigurationValues, ImportParams, Result, RuntimeVersion, SharedParams,
};
use sc_service::{config::DatabaseSource, Configuration, Role, TaskManager};
use sp_runtime::traits::Block as BlockT;

use crate::service::new_full;

/// Build a `Configuration` object from the provided command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = crate::service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, .. } = crate::service::new_partial(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, .. } = crate::service::new_partial(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = crate::service::new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run(config.database))
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                let PartialComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = crate::service::new_partial(&config)?;
                Ok((cmd.run(client, backend), task_manager))
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| cmd.run::<sp_io::SubstrateHostFunctions, _>(config))
        }
        None => {
            let runner = cli.create_runner(&cli.run)?;
            runner.run_node_until_exit(|config| async move {
                match config.role {
                    Role::Light => service::new_light(config),
                    _ => service::new_full(config),
                }
                .map(|service| service.0)
            })
        }
    }
}

/// Run a node with the given configuration.
pub fn run_node(
    config: Configuration,
    task_manager: &mut TaskManager,
) -> sc_service::error::Result<()> {
    match config.role {
        Role::Light => new_light(config).map(|service| service.0),
        _ => new_full(config).map(|service| service.0),
    }
    .map(|_| ())
}

/// Run a light node with the given configuration.
pub fn new_light(
    config: Configuration,
) -> Result<TaskManager, sc_service::Error> {
    let (client, backend, keystore, mut task_manager, on_demand) =
        sc_service::new_light_parts::<
            crate::service::Block,
            crate::service::RuntimeApi,
            crate::service::Executor,
        >(&config)?;

    let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
        config.transaction_pool.clone(),
        config.prometheus_registry(),
        task_manager.spawn_handle(),
        client.clone(),
        on_demand.clone(),
    ));

    let grandpa_block_import = sc_finality_grandpa::light_block_import(
        client.clone(),
        backend.clone(),
        &(client.clone() as Arc<_>),
        Arc::new(on_demand.checker().clone()) as Arc<_>,
    )?;

    let (grandpa, _) = sc_finality_grandpa::register_grandpa_light(
        grandpa_block_import,
        &(client.clone() as Arc<_>),
        config.prometheus_registry().as_ref(),
    );

    let babe_config = sc_consensus_babe::Config::get_or_compute(&*client)?;
    let (babe_link, _) = sc_consensus_babe::block_import(
        babe_config,
        grandpa,
        client.clone(),
    )?;

    let justification_import = babe_link.clone();
    let import_queue = sc_consensus_babe::import_queue(
        babe_link,
        Box::new(justification_import),
        None,
        Some(Box::new(justification_import)),
        client.clone(),
        client,
        task_manager.spawn_handle(),
        config.prometheus_registry(),
        sp_consensus::NeverCanAuthor,
    )?;

    Ok((task_manager, import_queue, client, transaction_pool, backend, keystore))
}

/// Run a full node with the given configuration.
pub fn new_full(
    config: Configuration,
) -> Result<TaskManager, sc_service::Error> {
    let mut task_manager = sc_service::TaskManager::new(
        config.task_executor.clone(),
        config.prometheus_registry(),
    )?;

    let (client, backend, keystore, mut task_manager, on_demand) =
        sc_service::new_full_parts::<
            crate::service::Block,
            crate::service::RuntimeApi,
            crate::service::Executor,
        >(&config, task_manager.spawn_handle())?;

    let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_handle(),
        client.clone(),
    ));

    let grandpa_config = sc_finality_grandpa::Config {
        // Using the default voting rule here for now
        justification_period: 512,
        name: None,
        observer_enabled: true,
        keystore: Some(keystore.clone()),
        is_authority: config.role.is_authority(),
    };

    let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
        client.clone(),
        &(client.clone() as Arc<_>),
        select_chain.clone(),
    )?;

    let (block_import, babe_link) = sc_consensus_babe::block_import(
        sc_consensus_babe::Config::get_or_compute(&*client)?,
        grandpa_block_import,
        client.clone(),
    )?;

    let justification_import = grandpa_link.justification_import();
    let (import_queue, babe_worker_handle) = sc_consensus_babe::import_queue(
        babe_link.clone(),
        block_import.clone(),
        Some(Box::new(justification_import)),
        None,
        client.clone(),
        client.clone(),
        task_manager.spawn_handle(),
        config.prometheus_registry(),
        sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
    )?;

    Ok((task_manager, import_queue, client, transaction_pool, backend, keystore, babe_worker_handle))
}

/// Build a `Configuration` object from the provided command line arguments
pub fn load_spec(
    id: &str,
) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
    Ok(match id {
        "dev" => Box::new(chain_spec::development_config()),
        "local" => Box::new(chain_spec::local_testnet_config()),
        "ikub" => Box::new(chain_spec::ikub_chain_config()),
        path => {
            let path = std::path::PathBuf::from(path);
            if path.exists() {
                Box::new(chain_spec::ChainSpec::from_json_file(path)?)
            } else {
                return Err(format!("Chain spec not found: {}", path.display()));
            }
        }
    })
}
