use actix;
use nats;
use nats::jetstream::{
    AckPolicy, Consumer as JetStreamConsumer, ConsumerConfig, DeliverPolicy, DiscardPolicy, ReplayPolicy,
    RetentionPolicy, StorageType, StreamConfig, StreamInfo, DateTime, ConsumerInfo, AccountInfo
};
use chrono::{DateTime as ChronoDateTime, Utc};
// use chrono::TimeZone;
// use near_indexer::StreamerMessage;
use borealis_types::prelude::{BorealisMessage, StreamerMessage};
use serde_cbor as cbor;
use serde_json;
use serde::de::DeserializeOwned;
use tracing::info;
use tracing_subscriber::EnvFilter;

use core::str::FromStr;

pub type Error = Box<dyn std::error::Error + 'static>;

/// CLI options to run Borealis Consumer
#[derive(Debug, Clone)]
pub struct Context {
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
//  default_value = "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222"
    pub nats_server: String,
    /// Consumer work mode (standard `Subscriber` or `JetStream` subscriber)
//  default_value = "JetStream"
    pub work_mode: WorkMode,
    /// Consumer subject, for subscription and to take messages from
//  default_value = "BlockIndex_StreamerMessages"
    pub subject: String,
    /// Consuming messages format (`CBOR` or `JSON`), suffix for subject name
//  default_value = "CBOR"
    pub msg_format: MsgFormat,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            root_cert_path: Some(std::path::PathBuf::from("./.nats/seed/root-ca.crt")),
            client_cert_path: None,
            client_private_key: None,
            creds_path: Some(std::path::PathBuf::from("./.nats/seed/nats.creds")),
            nats_server: "tls://eastcoast.nats.backend.aurora.dev:4222,tls://westcoast.nats.backend.aurora.dev:4222".to_string(),
            work_mode: WorkMode::Jetstream,
            subject: "BlockIndex_StreamerMessages_mainnet".to_string(),
            msg_format: MsgFormat::Cbor,
        }
    }
}

/// Consumer work mode
#[derive(Debug, Clone, Copy)]
pub enum WorkMode {
    Subscriber,
    Jetstream,
}

impl FromStr for WorkMode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.to_lowercase();
        match input.as_str() {
            "subscriber" => Ok(WorkMode::Subscriber),
            "jetstream" => Ok(WorkMode::Jetstream),
            _ => Err("Unknown consumer work mode: `--work-mode` should be `Subscriber` or `JetStream`".to_string().into()),
        }
    }
}

/// Consuming messages format (should be upper case, 'cause it's a suffix for `subject` name, and NATS subject is case sensitive)
#[derive(Debug, Clone, Copy)]
pub enum MsgFormat {
    Cbor,
    Json,
}

impl FromStr for MsgFormat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.to_lowercase();
        match input.as_str() {
            "cbor" => Ok(MsgFormat::Cbor),
            "json" => Ok(MsgFormat::Json),
            _ => Err("Unknown message format: `--msg-fomat` should contain `CBOR` or `JSON`".to_string().into()),
        }
    }
}

impl ToString for MsgFormat {
    fn to_string(&self) -> String {
        match self {
            MsgFormat::Cbor => String::from("CBOR"),
            MsgFormat::Json => String::from("JSON"),
        }
    }
}

/// Verbosity level for messages dump to log and stdout:
/// WithBlockHashHeight - output only block height & hash
/// WithStreamerMessageDump - full dump of `StreamerMessage`
/// WithStreamerMessageParse - full dump with full parse of `StreamerMessage`
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum VerbosityLevel {
    WithBlockHashHeight,
    WithStreamerMessageDump,
    WithStreamerMessageParse,
}

impl FromStr for VerbosityLevel {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.to_lowercase();
        match input.as_str() {
            "0" | "withblockhashheight" => Ok(VerbosityLevel::WithBlockHashHeight),
            "1" | "withstreamermessagedump" => Ok(VerbosityLevel::WithStreamerMessageDump),
            "2" | "withstreamermessageparse" => Ok(VerbosityLevel::WithStreamerMessageParse),
            _ => Err("Unknown output verbosity level: `--verbose` should be `WithBlockHashHeight` (`0`), `WithStreamerMessageDump` (`1`) or `WithStreamerMessageParse` (`2`)".to_string().into()),
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

/// Consumer's methods for Borealis NATS Bus
// #[async_trait]
pub trait Consumer {
    fn nats_connect(self) -> nats::Connection;
    fn nats_check_connection(nats_connection: &nats::Connection);
}

// #[async_trait]
// impl Consumer for Context {}

/// Create connection to Borealis NATS Bus
pub fn nats_connect(context: Context) -> nats::Connection {
    let creds_path = context
        .creds_path
        .unwrap_or(std::path::PathBuf::from("./.nats/seed/nats.creds"));

    let options = match (
        context.root_cert_path,
        context.client_cert_path,
        context.client_private_key,
    ) {
        (Some(root_cert_path), None, None) => {
            nats::Options::with_credentials(creds_path)
                .with_name("Borealis Indexer [TLS, Server Auth]")
                .tls_required(true)
                .add_root_certificate(root_cert_path)
                .reconnect_buffer_size(1024 * 1024 * 1024)
                .max_reconnects(100000)
                .reconnect_callback(|| info!(target: "borealis_consumer", "connection has been reestablished"))
                .reconnect_delay_callback(|reconnect_try| {
                    let reconnect_attempt = {
                        if reconnect_try == 0 {
                            1 as usize
                        } else {
                            reconnect_try
                        }
                    };
                    let delay = core::time::Duration::from_millis(std::cmp::min(
                        (reconnect_attempt * rand::Rng::gen_range(&mut rand::thread_rng(), 100..1000))
                            as u64,
                        1000,
                    ));
                    info!(
                        target: "borealis_consumer",
                        "reconnection attempt #{} within delay of {:?} ...",
                        reconnect_attempt, delay
                    );
                    delay
                })
                .disconnect_callback(|| info!(target: "borealis_consumer", "connection has been lost")) // todo: re-run message consumer
                .close_callback(|| info!(target: "borealis_consumer", "connection has been closed")) // todo: re-run message consumer
        }
        (Some(root_cert_path), Some(client_cert_path), Some(client_private_key)) => {
            nats::Options::with_credentials(creds_path)
                .with_name("Borealis Indexer [TLS, Server Auth, Client Auth]")
                .tls_required(true)
                .add_root_certificate(root_cert_path)
                .client_cert(client_cert_path, client_private_key)
                .reconnect_buffer_size(1024 * 1024 * 1024)
                .max_reconnects(100000)
                .reconnect_callback(|| info!(target: "borealis_consumer", "connection has been reestablished"))
                .reconnect_delay_callback(|reconnect_try| {
                    let reconnect_attempt = {
                        if reconnect_try == 0 {
                            1 as usize
                        } else {
                            reconnect_try
                        }
                    };
                    let delay = core::time::Duration::from_millis(std::cmp::min(
                        (reconnect_attempt * rand::Rng::gen_range(&mut rand::thread_rng(), 100..1000))
                            as u64,
                        1000,
                    ));
                    info!(
                        target: "borealis_consumer",
                        "reconnection attempt #{} within delay of {:?} ...",
                        reconnect_attempt, delay
                    );
                    delay
                })
                .disconnect_callback(|| info!(target: "borealis_consumer", "connection has been lost")) // todo: re-run message consumer
                .close_callback(|| info!(target: "borealis_consumer", "connection has been closed")) // todo: re-run message consumer
        }
        _ => {
            nats::Options::with_credentials(creds_path)
                .with_name("Borealis Indexer [NATS, without TLS]")
                .reconnect_buffer_size(1024 * 1024 * 1024)
                .max_reconnects(100000)
                .reconnect_callback(|| info!(target: "borealis_consumer", "connection has been reestablished"))
                .reconnect_delay_callback(|reconnect_try| {
                    let reconnect_attempt = {
                        if reconnect_try == 0 {
                            1 as usize
                        } else {
                            reconnect_try
                        }
                    };
                    let delay = core::time::Duration::from_millis(std::cmp::min(
                        (reconnect_attempt * rand::Rng::gen_range(&mut rand::thread_rng(), 100..1000))
                            as u64,
                        1000,
                    ));
                    info!(
                        target: "borealis_consumer",
                        "reconnection attempt #{} within delay of {:?} ...",
                        reconnect_attempt, delay
                    );
                    delay
                })
                .disconnect_callback(|| info!(target: "borealis_consumer", "connection has been lost")) // todo: re-run message consumer
                .close_callback(|| info!(target: "borealis_consumer", "connection has been closed")) // todo: re-run message consumer
        }
    };

    let nats_connection = options
        .connect(context.nats_server.as_str())
        .expect("NATS connection error or wrong credentials");

    nats_connection
}

/// Check connection to Borealis NATS Bus
pub fn nats_check_connection(nats_connection: &nats::Connection) {
//  info!(target: "borealis_consumer", "NATS Connection: {:?}", nats_connection);
    info!(target: "borealis_consumer", "round trip time (rtt) between this client and the current NATS server: {:?}", nats_connection.rtt());
    info!(target: "borealis_consumer", "this client IP address, as known by the current NATS server: {:?}", nats_connection.client_ip());
    info!(target: "borealis_consumer", "this client ID, as known by the current NATS server: {:?}", nats_connection.client_id());
    info!(target: "borealis_consumer", "maximum payload size the current NATS server will accept: {:?}", nats_connection.max_payload());
}

// Initialization for JetStream consumers
pub fn init(context: &Context) {
    let nats_connection = nats_connect(context.to_owned());
    let stream_info = jetstream_create_stream(&nats_connection, format!("{}_{}", context.subject, context.msg_format.to_string()), Some(vec![format!("{}_{}", context.subject, context.msg_format.to_string())]));
    let consumer = jetstream_create_consumer_from_args(context, &nats_connection);

    info!(
        target: "borealis_consumer",
        "Initialized:\nStream:\n{:?}\nConsumer:\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        stream_info,
        consumer.nc,
        consumer.stream,
        consumer.cfg,
        consumer.push_subscriber,
        consumer.timeout
    );
}

// Create JetStream with initial stream name and consisting of exact subject names
pub fn jetstream_create_stream(nats_connection: &nats::Connection, stream_name: String, subject_names: Option<Vec<String>>) -> StreamInfo {
    // JetStreams cannot be created from NATS Client side due to restrictions on NATS server side, but this ability is still available in library for client side consumers
    let stream_info = nats_connection.create_stream(StreamConfig {
        name: stream_name,
        discard: DiscardPolicy::Old,
        subjects: subject_names,
        duplicate_window: 86400,
        retention: RetentionPolicy::Limits,
        storage: StorageType::File,
        ..Default::default()
    }).expect("IO error, something went wrong while creating a new stream, maybe stream already exist");

    info!(
        target: "borealis_consumer",
        "Initialized:\nStream:\n{:?}",
        stream_info
    );

    stream_info
}

// Create JetStream consumer with custom parameters (see documentation descrption for the meaning of exact ConsumerConfig parameters)
pub fn jetstream_create_consumer(nats_connection: &nats::Connection, stream_name: String, deliver_subject: Option<String>, durable_name: Option<String>, filter_subject: String) -> JetStreamConsumer {
    let consumer = JetStreamConsumer::create_or_open(nats_connection.to_owned(), stream_name.as_str(), ConsumerConfig {
        deliver_subject,
        durable_name,
        deliver_policy: DeliverPolicy::Last,
        ack_policy: AckPolicy::Explicit,
        filter_subject,
        replay_policy: ReplayPolicy::Instant,
        ..Default::default()
    }).expect("IO error, something went wrong while creating a new consumer or returning an existent consumer");

    info!(
        target: "borealis_consumer",
        "Initialized:\nConsumer:\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        consumer.nc,
        consumer.stream,
        consumer.cfg,
        consumer.push_subscriber,
        consumer.timeout
    );

    consumer
}

pub fn jetstream_create_consumer_from_start_seq(nats_connection: &nats::Connection, stream_name: String, deliver_subject: Option<String>, durable_name: Option<String>, filter_subject: String, start_seq: Option<i64>) -> JetStreamConsumer {
    let consumer = JetStreamConsumer::create_or_open(nats_connection.to_owned(), stream_name.as_str(), ConsumerConfig {
        deliver_subject,
        durable_name,
        deliver_policy: DeliverPolicy::ByStartSeq,
        ack_policy: AckPolicy::Explicit,
        filter_subject,
        replay_policy: ReplayPolicy::Instant,
        opt_start_seq: match start_seq {
            Some(seq) => seq,
            None => Default::default(),
        },
        ..Default::default()
    }).expect("IO error, something went wrong while creating a new consumer or returning an existent consumer");

    info!(
        target: "borealis_consumer",
        "Initialized:\nConsumer:\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        consumer.nc,
        consumer.stream,
        consumer.cfg,
        consumer.push_subscriber,
        consumer.timeout
    );

    consumer
}

pub fn jetstream_create_consumer_from_start_time(nats_connection: &nats::Connection, stream_name: String, deliver_subject: Option<String>, durable_name: Option<String>, filter_subject: String, start_time: Option<ChronoDateTime<Utc>>) -> JetStreamConsumer {
    let consumer = JetStreamConsumer::create_or_open(nats_connection.to_owned(), stream_name.as_str(), ConsumerConfig {
        deliver_subject,
        durable_name,
        deliver_policy: DeliverPolicy::ByStartTime,
        ack_policy: AckPolicy::Explicit,
        filter_subject,
        replay_policy: ReplayPolicy::Instant,
        opt_start_time: match start_time {
            Some(time) => Some(DateTime(time)),
            None => Default::default(),
        },
        ..Default::default()
    }).expect("IO error, something went wrong while creating a new consumer or returning an existent consumer");

    info!(
        target: "borealis_consumer",
        "Initialized:\nConsumer:\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        consumer.nc,
        consumer.stream,
        consumer.cfg,
        consumer.push_subscriber,
        consumer.timeout
    );

    consumer
}

// Create JetStream consumer with Context parameters (see documentation descrption for the meaning of exact Context parameters)
pub fn jetstream_create_consumer_from_args(context: &Context, nats_connection: &nats::Connection) -> JetStreamConsumer {
    let consumer = JetStreamConsumer::create_or_open(nats_connection.to_owned(), format!("{}_{}", context.subject, context.msg_format.to_string()).as_str(), ConsumerConfig {
        deliver_subject: Some(format!("{}_{}", context.subject, context.msg_format.to_string())),
        durable_name: Some(format!("Borealis_Consumer_{}_{}", context.subject, context.msg_format.to_string())),
        deliver_policy: DeliverPolicy::Last,
        ack_policy: AckPolicy::Explicit,
        filter_subject: format!("{}_{}", context.subject, context.msg_format.to_string()),
        replay_policy: ReplayPolicy::Instant,
        ..Default::default()
    }).expect("IO error, something went wrong while creating a new consumer or returning an existent consumer");

    info!(
        target: "borealis_consumer",
        "Initialized:\nConsumer:\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
        consumer.nc,
        consumer.stream,
        consumer.cfg,
        consumer.push_subscriber,
        consumer.timeout
    );

    consumer
}

pub fn get_stream_names(nats_connection: &nats::Connection) {
    nats_connection.stream_names().for_each(| stream_name | {
        match stream_name {
            Ok(stream_name) => {
                info!(
                    target: "borealis_consumer",
                    "Stream name: {}",
                    stream_name
                );
            },
            Err(error) => {
                info!(
                    target: "borealis_consumer",
                    "Error during retrieving a stream name: {}",
                    error
                );
            },
        }
    });
}

pub fn get_streams_list(nats_connection: &nats::Connection) {
    nats_connection.list_streams().for_each(| stream_info | {
        match stream_info {
            Ok(stream_info) => {
                info!(
                    target: "borealis_consumer",
                    "Stream information: {:?}",
                    stream_info
                );
            },
            Err(error) => {
                info!(
                    target: "borealis_consumer",
                    "Error during retrieving a stream information: {}",
                    error
                );
            },
        }
    });
}

pub fn get_stream_info(nats_connection: &nats::Connection, stream_name: String) -> std::io::Result<StreamInfo> {
    match nats_connection.stream_info(stream_name.as_str()) {
        Ok(stream_info) => {
            info!(
                target: "borealis_consumer",
                "Stream {} information: {:?}",
                stream_name,
                stream_info
            );
            Ok(stream_info)
        },
        Err(error) => {
            info!(
                target: "borealis_consumer",
                "Error during retrieving a stream {} information: {}",
                stream_name,
                error
            );
            Err(error)
        },
    }
}

pub fn get_consumers_list(nats_connection: &nats::Connection, stream_name: String) {
    match nats_connection.list_consumers(stream_name.as_str()) {
        Ok(consumers_list) => {
            consumers_list.for_each(| consumer_info | {
                if let Ok(consumer_info) = consumer_info {
                    info!(
                        target: "borealis_consumer",
                        "Consumer information for stream {}: {:?}",
                        stream_name,
                        consumer_info
                    );
                } else if let Err(error) = consumer_info {
                    info!(
                        target: "borealis_consumer",
                        "Error during retrieving a consumer information for stream {}: {}",
                        stream_name,
                        error
                    );
                };
            });
        },
        Err(error) => {
            info!(
                target: "borealis_consumer",
                "Error during retrieving a consumer list for stream {}: {}",
                stream_name,
                error
            );
        },
    }
}

pub fn get_consumer_info(nats_connection: &nats::Connection, stream_name: String, consumer_name: String) -> std::io::Result<ConsumerInfo> {
    match nats_connection.consumer_info(stream_name.as_str(), consumer_name.as_str()) {
        Ok(consumer_info) => {
            info!(
                target: "borealis_consumer",
                "Consumer {} information for stream {}: {:?}",
                consumer_name,
                stream_name,
                consumer_info
            );
            Ok(consumer_info)
        },
        Err(error) => {
            info!(
                target: "borealis_consumer",
                "Error during retrieving a consumer {} information for stream {}: {}",
                consumer_name,
                stream_name,
                error
            );
            Err(error)
        },
    }
}

pub fn get_jetstream_account_info(nats_connection: &nats::Connection) -> std::io::Result<AccountInfo> {
    match nats_connection.account_info() {
        Ok(account_info) => {
            info!(
                target: "borealis_consumer",
                "JetStream account information: {:?}",
                account_info
            );
            Ok(account_info)
        },
        Err(error) => {
            info!(
                target: "borealis_consumer",
                "Error during retrieving a JetStream account information: {}",
                error
            );
            Err(error)
        },
    }
}


/*
nats_connect(context) -> Connection
nats_check_connection(context):
let nc = nats::connect("demo.nats.io")?;
let nats_connection = nats_connect(context);
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.rtt
println!("server rtt: {:?}", nc.rtt());
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.client_ip
println!("ip: {:?}", nc.client_ip());
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.client_id
println!("id: {:?}", nc.client_id());
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.max_payload
println!("max payload: {:?}", nc.max_payload());

consumer_init(context(stream, subject, msg_format))
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.stream_names
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.list_streams
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.stream_info
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.list_consumers
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.consumer_info
https://docs.rs/nats/0.16.0/nats/struct.Connection.html#method.account_info
jetstream_create_stream(stream_name, subjects[subject, msg_format])
jetstream_create_consumer(stream, subject, msg_format)
jetstream_create_consumer_from_start_seq(stream, subject, msg_format, start_seq)
jetstream_create_consumer_from_start_time(stream, subject, msg_format, start_time)
jetstream_create_consumer_from_args(stream, subject, msg_format)
jetstream_consumer_create_or_open(stream, subject, msg_format) -> nats::jetstream::Consumer
consumer_subscribe(subject, msg_format) -> subscription
consumer_run(work_mode, subject, msg_format)

message_consumer(subscription, msg_format)
message_jetstream_consumer(consumer, msg_format)
message_decode(msg, msg_format) -> RawEvent, Headers, StreamerMessage

message_dump/message_print(StreamerMessage)
message_log(StreamerMessage)
*/

