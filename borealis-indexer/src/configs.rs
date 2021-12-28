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
    /// Custom directory for configurations and state. Defaults to ./.borealis-indexer/
    #[clap(short, long)]
    pub home_dir: Option<std::path::PathBuf>,
    //  /// Configuration file path
    //  #[clap(short, long)]
    //  pub config_path: Option<std::path::PathBuf>,
    /// Subcommands
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

/// CLI subcommands
#[derive(Clap, Debug, Clone)]
pub(crate) enum SubCommand {
    /// Checking connection to NATS
    Check(RunArgs),
    /// Initialize Borealis Indexer configuration
    Init(InitConfigArgs),
    /// Run Borealis Indexer with options and sync blocks from the network
    Run(RunArgs),
}

/// CLI options to run Borealis Indexer
#[derive(Clap, Debug, Clone)]
pub(crate) struct RunArgs {
    /// root CA certificate
    #[clap(long)]
    pub root_cert_path: Option<std::path::PathBuf>,
    /// client certificate
    #[clap(long)]
    pub client_cert_path: Option<std::path::PathBuf>,
    /// client private key
    #[clap(long)]
    pub client_private_key: Option<std::path::PathBuf>,
    /// Path to NATS credentials (JWT/NKEY tokens)
    #[clap(short, long)]
    pub creds_path: Option<std::path::PathBuf>,
    /// Borealis Bus (NATS based MOM/MQ/SOA service bus) protocol://address:port
    /// Example: "nats://borealis.aurora.dev:4222" or "tls://borealis.aurora.dev:4443" for TLS connection
    #[clap(long, default_value = "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222")]
    pub nats_server: String,
    /// Stream messages to subject
    #[clap(long, default_value = "BlockIndex_StreamerMessages")]
    pub subject: String,
    /// Streaming messages format (`CBOR` or `JSON`), suffix for subject name
    #[clap(long, default_value = "CBOR")]
    pub msg_format: MsgFormat,
}

/// Streaming messages format
#[derive(Clap, Debug, Clone, Copy)]
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
#[derive(Clap, Debug, Clone)]
pub(crate) struct InitConfigArgs {
    /// chain/network id (localnet, devnet, testnet, betanet, mainnet)
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

/// Initialize logging
pub(crate) fn init_logging() {
    // Custom filters
    let env_filter = EnvFilter::new(
        "borealis-indexer=info,tokio_reactor=info,near=info,near=error,stats=info,telemetry=info,borealis_indexer=info,indexer=info,near-performance-metrics=info",
    );
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();
}
