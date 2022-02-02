use clap::{AppSettings, Clap};

use tracing_subscriber::EnvFilter;

use core::str::FromStr;

type Error = Box<dyn std::error::Error + 'static>;

/// CLI options (subcommands and flags)
#[derive(Clap, Debug)]
#[clap(version = "0.1.0", author = "Aurora <hello@aurora.dev>")]
#[clap(setting = AppSettings::SubcommandRequiredElseHelp)]
pub(crate) struct Opts {
    /// Verbosity level for extensive output to stdout or log
    #[clap(short, long)]
    pub verbose: Option<bool>,
    /// Subcommands
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

/// CLI subcommands
#[derive(Clap, Debug, Clone)]
pub(crate) enum SubCommand {
    /// Checking connection to NATS
    Check(RunArgs),
    /// Initialize NATS JetStream consumer and stream configurations
    Init(RunArgs),
    /// Run Borealis Consumer wirh options
    Run(RunArgs),
}

/// CLI options to run Borealis Consumer
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
    #[clap(
        long,
        default_value = "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222"
    )]
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

/// Consumer work mode
#[derive(Clap, Debug, Clone, Copy)]
pub(crate) enum WorkMode {
    Subscriber,
    Jetstream,
}

impl FromStr for WorkMode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Subscriber" | "subscriber" => Ok(WorkMode::Subscriber),
            "JetStream" | "Jetstream" | "jetstream" => Ok(WorkMode::Jetstream),
            _ => Err(
                "Unknown consumer work mode: `--work-mode` should be `Subscriber` or `JetStream`"
                    .to_string()
                    .into(),
            ),
        }
    }
}

/// Consuming messages format (should be upper case, 'cause it's a suffix for `subject` name, and NATS subject is case sensitive)
#[derive(Clap, Debug, Clone, Copy)]
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
            _ => Err(
                "Unknown message format: `--msg-fomat` should contain `CBOR` or `JSON`"
                    .to_string()
                    .into(),
            ),
        }
    }
}

/// Initialize logging
pub(crate) fn init_logging() {
    // Custom filters
    let env_filter = EnvFilter::new(
        "borealis-consumer=info,tokio_reactor=info,near=info,near=error,stats=info,telemetry=info,borealis_consumer=info,indexer=info,near-performance-metrics=info",
    );
    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();
}
