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
}

impl Default for RunArgs {
    fn default() -> Self {
        Self
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


Producer(RunArgs, InitConfigArgs)

impl Default::default() for RunArgs
impl Default::default() for InitConfigArgs

impl From/Into<cli::InitConfigArgs> for lib::InitConfigArgs
impl From/Into<cli::InitConfigArgs> for near_indexer::InitConfigArgs
impl From/Into<lib::InitConfigArgs> for near_indexer::InitConfigArgs
impl From/Into<cli::RunArgs> for lib::RunArgs


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
message_encode(msg, msg_format) -> msg_fmt
message_publish(msg_fmt, subject, msg_format)


message_dump/message_print(StreamerMessage)
message_log(StreamerMessage)

