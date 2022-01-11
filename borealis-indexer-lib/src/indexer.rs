use actix;
use borealis_indexer_types::prelude::BorealisMessage;
use nats;
use near_indexer;
use serde_cbor as cbor;
use serde_json;
use tokio::sync::mpsc;
use tracing::info;

use near_indexer::near_primitives::types::Gas;

use tracing_subscriber::EnvFilter;

use core::str::FromStr;

type Error = Box<dyn std::error::Error + 'static>;

/// CLI options to run Borealis Indexer
#[derive(Debug, Clone)]
pub struct RunArgs {
    /// root CA certificate
    pub root_cert_path: Option<std::path::PathBuf>,
    /// client certificate
    pub client_cert_path: Option<std::path::PathBuf>,
    /// client private key
    pub client_private_key: Option<std::path::PathBuf>,
    /// Path to NATS credentials (JWT/NKEY tokens)
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
    #[clap(long, default_value = "FromInterruption")]
    pub sync_mode: SyncMode,
    pub block_height: Option<u64>,
}

impl Default for RunArgs {
    fn default() -> Self {
        Self
    }
}

/// Streaming messages format
#[derive(Debug, Clone, Copy)]
pub(crate) enum MsgFormat {
    CBOR,
    JSON,
}

impl FromStr for MsgFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CBOR" | "Cbor" | "cbor" => Ok(MsgFormat::CBOR),
            "JSON" | "Json" | "json" => Ok(MsgFormat::JSON),
            _ => Err("Unknown message format: `--msg-fomat` should contain `CBOR` or `JSON`".to_string().into()),
        }
    }
}

/// Definition of a syncing mode for NEAR Indexer
#[derive(Debug, Clone, Copy)]
pub(crate) enum SyncMode {
    /// Real-time syncing, always taking the latest finalized block to stream
    LatestSynced,
    /// Starts syncing from the block NEAR Indexer was interrupted last time
    FromInterruption,
    /// Specific block height to start syncing from, RunArgs.block_height should follow after it
    BlockHeight,
}

impl FromStr for SyncMode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LatestSynced" | "Latestsynced" | "latestsynced" => Ok(SyncMode::LatestSynced),
            "FromInterruption" | "Frominterruption" | "frominterruption" => Ok(SyncMode::FromInterruption),
            "BlockHeight" | "Blockheight" | "blockheight" => Ok(SyncMode::BlockHeight),
            _ => Err("Unknown indexer synchronization mode: `--sync-mode` should be `LatestSynced`, `FromInterruption` or `BlockHeight` with --block-height explicit pointing".to_string().into()),
        }
    }
}

/// Override standard config args with CLI options
#[derive(Debug, Clone)]
pub struct InitConfigArgs {
    /// chain/network id (localnet, devnet, testnet, betanet, mainnet)
    pub chain_id: Option<String>,
    /// Account ID for the validator key
    pub account_id: Option<String>,
    /// Specify private key generated from seed (TESTING ONLY)
    pub test_seed: Option<String>,
    /// Number of shards to initialize the chain with
    #[clap(short, long, default_value = "1")]
    pub num_shards: u64,
    /// Makes block production fast (TESTING ONLY)
    pub fast: bool,
    /// Genesis file to use when initialize testnet (including downloading)
    pub genesis: Option<String>,
    /// Download the verified NEAR genesis file automatically.
    pub download_genesis: bool,
    /// Specify a custom download URL for the genesis-file.
    pub download_genesis_url: Option<String>,
    /// Download the verified NEAR config file automatically.
    pub download_config: bool,
    /// Specify a custom download URL for the config file.
    pub download_config_url: Option<String>,
    /// Specify the boot nodes to bootstrap the network
    pub boot_nodes: Option<String>,
    /// Specify a custom max_gas_burnt_view limit.
    pub max_gas_burnt_view: Option<Gas>,
}

impl Default for InitConfigArgs {
    fn default() -> Self {
        Self
    }
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
pub fn init_logging() {
    // Custom filters
    let env_filter = EnvFilter::new(
        "borealis-indexer=info,tokio_reactor=info,near=info,near=error,stats=info,telemetry=info,borealis_indexer=info,indexer=info,near-performance-metrics=info",
    );
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();
}

/*
impl From/Into<cli::InitConfigArgs> for lib::InitConfigArgs
impl From/Into<cli::InitConfigArgs> for near_indexer::InitConfigArgs
impl From/Into<lib::InitConfigArgs> for near_indexer::InitConfigArgs
impl From/Into<cli::RunArgs> for lib::RunArgs
*/

pub trait Producer {
    fn nats_connect(&self) -> nats::Connection;
    fn nats_check_connection(&self);
    fn message_encode(&self, msg_seq_id: u64, payload: &T) -> BorealisMessage;
    fn run(&self);
}

pub trait Indexer {
    fn init(&self);
}

impl Indexer for InitConfigArgs {
/// Initialize Indexer's configurations
    fn init(&self, home_path: Option<std::path::PathBuf>) {
        // let home_dir = home_path.unwrap_or(std::path::PathBuf::from(near_indexer::get_default_home()));
        let home_dir = home_path
            .unwrap_or(std::path::PathBuf::from("./.borealis-indexer"));
        
        near_indexer::indexer_init_configs(&home_dir, self.into());
    }
}

impl Producer for RunArgs {
/// Create connection to Borealis NATS Bus
    fn nats_connect(&self) -> nats::Connection {
        let creds_path = self
            .creds_path
            .unwrap_or(std::path::PathBuf::from("./.nats/seed/nats.creds"));
    
        let options =
            match (
                self.root_cert_path,
                self.client_cert_path,
                self.client_private_key,
            ) {
                (Some(root_cert_path), None, None) => {
                    nats::Options::with_credentials(creds_path)
                        .with_name("Borealis Indexer [TLS]")
                        .tls_required(true)
                        .add_root_certificate(root_cert_path)
                        .reconnect_buffer_size(1024 * 1024 * 1024)
                        .max_reconnects(1000)
                        .reconnect_callback(|| println!("connection has been reestablished"))
                        .reconnect_delay_callback(|reconnect_attempt| {
                            let delay = core::time::Duration::from_millis(std::cmp::min(
                                (reconnect_attempt * rand::Rng::gen_range(&mut rand::thread_rng(), 50..100))
                                    as u64,
                                1000,
                            ));
                            println!("reconnection attempt #{} within delay of {:?} ms...", reconnect_attempt, delay);
                            delay
                        })
                        .disconnect_callback(|| println!("connection has been lost")) // todo: re-run message producer
                        .close_callback(|| println!("connection has been closed")) // todo: re-run message producer
                },
                (Some(root_cert_path), Some(client_cert_path), Some(client_private_key)) => {
                    nats::Options::with_credentials(creds_path)
                        .with_name("Borealis Indexer [TLS, Client Auth]")
                        .tls_required(true)
                        .add_root_certificate(root_cert_path)
                        .client_cert(client_cert_path, client_private_key)
                        .reconnect_buffer_size(1024 * 1024 * 1024)
                        .max_reconnects(1000)
                        .reconnect_callback(|| println!("connection has been reestablished"))
                        .reconnect_delay_callback(|reconnect_attempt| {
                            let delay = core::time::Duration::from_millis(std::cmp::min(
                                (reconnect_attempt * rand::Rng::gen_range(&mut rand::thread_rng(), 50..100))
                                    as u64,
                                1000,
                            ));
                            println!("reconnection attempt #{} within delay of {:?} ms...", reconnect_attempt, delay);
                            delay
                        })
                        .disconnect_callback(|| println!("connection has been lost")) // todo: re-run message producer
                        .close_callback(|| println!("connection has been closed")) // todo: re-run message producer
                },
                _ => {
                    nats::Options::with_credentials(creds_path)
                        .with_name("Borealis Indexer [NATS, w/o TLS]")
                        .reconnect_buffer_size(1024 * 1024 * 1024)
                        .max_reconnects(1000)
                        .reconnect_callback(|| println!("connection has been reestablished"))
                        .reconnect_delay_callback(|reconnect_attempt| {
                            let delay = core::time::Duration::from_millis(std::cmp::min(
                                (reconnect_attempt * rand::Rng::gen_range(&mut rand::thread_rng(), 50..100))
                                    as u64,
                                1000,
                            ));
                            println!("reconnection attempt #{} within delay of {:?} ms...", reconnect_attempt, delay);
                            delay
                        })
                        .disconnect_callback(|| println!("connection has been lost")) // todo: re-run message producer
                        .close_callback(|| println!("connection has been closed")) // todo: re-run message producer
                },
            };
    
        let nats_connection = options
            .connect(self.nats_server)
            .expect("NATS connection error or wrong credentials");
    
        nats_connection
    }

/// Check connection to Borealis NATS Bus
    fn nats_check_connection(&self) {
        let nats_connection = self.nats_connect();
        println!("NATS Connection: {:?}", nats_connection);
        println!("round trip time (rtt) between this client and the current NATS server: {:?}", nats_connection.rtt());
        println!("this client IP address, as known by the current NATS server: {:?}", nats_connection.client_ip());
        println!("this client ID, as known by the current NATS server: {:?}", nats_connection.client_id());
        println!("maximum payload size the current NATS server will accept: {:?}", nats_connection.max_payload());
    }

/// Create Borealis Message with payload
    fn message_encode(&self, msg_seq_id: u64, payload: &T) -> BorealisMessage {
        match self.msg_format {
            MsgFormat::CBOR => {
                BorealisMessage::new(
                    msg_seq_id,
                    payload,
                ).to_cbor()
            }
            MsgFormat::JSON => {
                BorealisMessage::new(
                    msg_seq_id,
                    payload,
                ).to_json_bytes()
            }
        }
    }

    fn run(&self, home_path: Option<std::path::PathBuf>) {
        // let home_dir = home_path.unwrap_or(std::path::PathBuf::from(near_indexer::get_default_home()));
        let home_dir = home_path
            .unwrap_or(std::path::PathBuf::from("./.borealis-indexer"));

        let indexer_config = near_indexer::IndexerConfig {
            home_dir,
            // recover and continue message streaming from latest synced block (real-time), or from interruption, or from exact block height
            sync_mode: match self.sync_mode {
                SyncMode::FromInterruption => near_indexer::SyncModeEnum::FromInterruption,
                SyncMode::LatestSynced => near_indexer::SyncModeEnum::LatestSynced,
                SyncMode::BlockHeight => near_indexer::SyncModeEnum::BlockHeight(self.block_height.unwrap_or(0)),
            },
            // stream messages while syncing
            await_for_node_synced: near_indexer::AwaitForNodeSyncedEnum::StreamWhileSyncing,
            // await_for_node_synced: near_indexer::AwaitForNodeSyncedEnum::WaitForFullSync,
        };

        let nats_connection = self.nats_connect();

        let system = actix::System::new();
        system.block_on(async move {
            let indexer = near_indexer::Indexer::new(indexer_config);
            let events_stream = indexer.streamer();
            actix::spawn(self.block_events_listener(
                events_stream,
                nats_connection,
                self.subject,
                self.msg_format,
            ).await);
            actix::System::current().stop();
        });
        system.run().unwrap();
    }
}


nats_connect(run_args) -> Connection
nats_check_connection(run_args):
let nc = nats::connect("demo.nats.io")?;
let nats_connection = nats_connect(run_args);
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.rtt
println!("server rtt: {:?}", nc.rtt());
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.client_ip
println!("ip: {:?}", nc.client_ip());
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.client_id
println!("id: {:?}", nc.client_id());
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.max_payload
println!("max payload: {:?}", nc.max_payload());


producer_init(config_args)
producer_run(run_args, run_args.sync_mode)

message_producer(events_stream = indexer.streamer(), nats_connection, subject, msg_format)
| block_events_listener()
|| handle_message()
message_encode(msg, msg_format) -> msg_fmt
message_publish(msg_fmt, subject, msg_format)


message_dump/message_print(StreamerMessage)
message_log(StreamerMessage)

