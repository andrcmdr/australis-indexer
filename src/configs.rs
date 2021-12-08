use clap::{AppSettings, Clap};

use near_indexer::near_primitives::types::Gas;

use tracing_subscriber::EnvFilter;

use core::str::FromStr;

type Error = Box<dyn std::error::Error + 'static>;

/// CLI options (subcommands and flags)
#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "Aurora <hello@aurora.dev>")]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
pub(crate) struct Opts {
    /// Custom directory for configurations and state. Defaults to ./.aurora-indexer/
    #[clap(short, long)]
    pub home_dir: Option<std::path::PathBuf>,
    /// Subcommand
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

/// CLI subcommands
#[derive(Clap, Debug)]
pub(crate) enum SubCommand {
    /// Run Aurora Indexer. Sync from the network.
    Run(RunArgs),
    /// Initialize configurations.
    Init(InitConfigArgs),
}

/// CLI options to run Aurora Indexer
#[derive(Clap, Debug)]
pub(crate) struct RunArgs {
    /// Path to NATS credentials (JWT/NKEY tokens)
    #[clap(short, long)]
    pub creds_path: Option<std::path::PathBuf>,
    /// Aurora Borealis (NATS based MOM/MQ/SOA service bus) protocol://address:port
    /// Example: "nats://borealis.aurora:4222" or "tls://borealis.aurora:4443" for TLS connection
    #[clap(long, default_value = "tls://borealis.aurora:4443")]
    pub nats_server: String,
    /// Streaming messages format
    #[clap(long, default_value = "CBOR")]
    pub msg_format: MsgFormat,
}

/// Streaming messages format
#[derive(Clap, Debug)]
pub(crate) enum MsgFormat {
    Cbor,
    Json,
}

impl FromStr for MsgFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CBOR" | "Cbor" | "cbor" => Ok(MsgFormat::Cbor),
            "JSON" | "Json" | "json" => Ok(MsgFormat::Json),
            _ => Err(
                "Unknown message format: `--msg-fomat` should contain `CBOR` or `JSON`"
                    .to_string()
                    .into(),
            ),
        }
    }
}

/// Override standard config args with CLI options
#[derive(Clap, Debug)]
pub(crate) struct InitConfigArgs {
    /// chain/network id (localnet, testnet, devnet, betanet)
    #[clap(short, long)]
    pub chain_id: Option<String>,
    /// Account ID for the validator key
    #[clap(long)]
    pub account_id: Option<String>,
    /// Specify private key generated from seed (TESTING ONLY)
    #[clap(long)]
    pub test_seed: Option<String>,
    /// Number of shards to initialize the chain with
    #[clap(short, long, default_value = "1")]
    pub num_shards: u64,
    /// Makes block production fast (TESTING ONLY)
    #[clap(short, long)]
    pub fast: bool,
    /// Genesis file to use when initialize testnet (including downloading)
    #[clap(short, long)]
    pub genesis: Option<String>,
    /// Download the verified NEAR genesis file automatically.
    #[clap(long)]
    pub download_genesis: bool,
    /// Specify a custom download URL for the genesis-file.
    #[clap(long)]
    pub download_genesis_url: Option<String>,
    /// Download the verified NEAR config file automatically.
    #[clap(long)]
    pub download_config: bool,
    /// Specify a custom download URL for the config file.
    #[clap(long)]
    pub download_config_url: Option<String>,
    /// Specify the boot nodes to bootstrap the network
    #[clap(long)]
    pub boot_nodes: Option<String>,
    /// Specify a custom max_gas_burnt_view limit.
    #[clap(long)]
    pub max_gas_burnt_view: Option<Gas>,
}

/// Initialize logging
pub(crate) fn init_logging() {
    // Custom filters
    let env_filter = EnvFilter::new(
        "indexer-example=info,tokio_reactor=info,near=info,near=error,stats=info,telemetry=info,indexer_example=info,indexer=info,near-performance-metrics=info",
    );
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();
}

/// Override standard config args with CLI options
impl From<InitConfigArgs> for near_indexer::InitConfigArgs {
    fn from(config_args: InitConfigArgs) -> Self {
        Self {
            chain_id: config_args.chain_id,
            account_id: config_args.account_id,
            test_seed: config_args.test_seed,
            num_shards: config_args.num_shards,
            fast: config_args.fast,
            genesis: config_args.genesis,
            download_genesis: config_args.download_genesis,
            download_genesis_url: config_args.download_genesis_url,
            download_config: config_args.download_config,
            download_config_url: config_args.download_config_url,
            boot_nodes: config_args.boot_nodes,
            max_gas_burnt_view: config_args.max_gas_burnt_view,
        }
    }
}
