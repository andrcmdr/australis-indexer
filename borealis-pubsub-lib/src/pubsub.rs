/// CLI options to run Borealis Consumer
#[derive(Debug, Clone)]
pub(crate) struct RunArgs {
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
    /// Consumer work mode (standard `Subscriber` or `JetStream` subscriber)
    #[clap(long, default_value = "JetStream")]
    pub work_mode: WorkMode,
    /// Consumer subject, for subscription and to take messages from
    #[clap(long, default_value = "BlockIndex_StreamerMessages")]
    pub subject: String,
    /// Consuming messages format (`CBOR` or `JSON`), suffix for subject name
    #[clap(long, default_value = "CBOR")]
    pub msg_format: MsgFormat,
}

impl Default for RunArgs {
    fn default() -> Self {
        Self
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


Consumer(RunArgs)

impl Default::default() for RunArgs

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

consumer_init(run_args(stream, subject, msg_format))
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.stream_names
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.list_streams
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.stream_info
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.list_consumers
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.consumer_info
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.account_info
create_jet_stream(stream_name, subjects[subject, msg_format])
create_jet_stream_consumer(stream, subject, msg_format)
create_jet_stream_consumer_with_start_seq(stream, subject, msg_format, start_seq)
create_jet_stream_consumer_with_start_time(stream, subject, msg_format, start_time)

consumer_run(work_mode, subject, msg_format)
consumer_subscribe(subject, msg_format) -> subscription
jet_stream_consumer_create_or_open(stream, subject, msg_format) -> nats::jetstream::Consumer

message_consumer(subscription, msg_format)
message_jetstream_consumer(consumer, msg_format)
message_decode(msg, msg_format) -> RawEvent, Headers, StreamerMessage

message_dump/message_print(StreamerMessage)
message_log(StreamerMessage)

