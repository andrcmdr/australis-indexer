use actix;
use borealis_types::prelude::BorealisMessage;
use clap::Clap;
use configs::{init_logging, AwaitSynced, MsgFormat, Opts, RunArgs, SubCommand, SyncMode, VerbosityLevel};
use nats;
use near_indexer;
use serde_cbor as cbor;
use serde_json;
use tokio::sync::mpsc;
use tracing::info;

pub mod configs;

async fn message_producer(
    mut events_stream: mpsc::Receiver<near_indexer::StreamerMessage>,
    nc: nats::Connection,
    subject: String,
    msg_format: MsgFormat,
    verbosity_level: Option<VerbosityLevel>,
) {
    info!(
        target: "borealis_indexer",
        "Message producer loop started: listening for new messages\n"
    );
    while let Some(streamer_message) = events_stream.recv().await {
        /*
            Example of `StreamerMessage` with all data fields (filled with synthetic data, as an example):

            Note that `outcomes` for a given transaction won't be included into the same block.
            Execution outcomes are included into the blocks after the transaction or receipt
            are recorded on a chain; in most cases, it is the next block after the one that has
            the transaction or receipt.

            StreamerMessage {
                block: BlockView {
                    author: "test.near",
                    header: BlockHeaderView {
                        height: 63596,
                        epoch_id: `Bk7pvZWUTfHRRZtfgTDjnQ6y5cV8yG2h3orCqJvUbiym`,
                        next_epoch_id: `3JuBZ4Gz5Eauf7PzQegfqSEDyvws3eKJYPbfGHAYmeR5`,
                        hash: `5X37niQWWcihDGQjsvDMHYKLCurNJyQLxCeLgneDb8mk`,
                        prev_hash: `2vJNJca72pBiq2eETq2xvuoc6caKDaUkdRgtdefyutbA`,
                        prev_state_root: `GkdxSBf4Kfq8V16N4Kqn3YdcThG1f5KG1KLBmXpMzP1k`,
                        chunk_receipts_root: `9ETNjrt6MkwTgSVMMbpukfxRshSD1avBUUa4R4NuqwHv`,
                        chunk_headers_root: `C7dVr9KdXYKt31yF2BkeAu115fpo79zYTqeU3FzqbFak`,
                        chunk_tx_root: `7tkzFg8RHBmMw1ncRJZCCZAizgq4rwCftTKYLce8RU8t`,
                        outcome_root: `7tkzFg8RHBmMw1ncRJZCCZAizgq4rwCftTKYLce8RU8t`,
                        chunks_included: 1,
                        challenges_root: `11111111111111111111111111111111`,
                        timestamp: 1618558205803345000,
                        timestamp_nanosec: 1618558205803345000,
                        random_value: `3cAa93XmoLaKAJQgWz3K7SiKwnA3uaxi8MGgLM78HTNS`,
                        validator_proposals: [],
                        chunk_mask: [
                            true,
                        ],
                        gas_price: 1000000000,
                        rent_paid: 0,
                        validator_reward: 0,
                        total_supply: 2050206401403887985811862247311434,
                        challenges_result: [],
                        last_final_block: `DCkMmXYHqibzcMjgFjRXJP7eckAMLrA4ijggSApMNwKu`,
                        last_ds_final_block: `2vJNJca72pBiq2eETq2xvuoc6caKDaUkdRgtdefyutbA`,
                        next_bp_hash: `4DJWnxRbUhRrsXK6EBkx4nFeXHKgJWqteDnJ7Hv4MZ6M`,
                        block_merkle_root: `Bvn5K89fJ3uPNsj3324Ls9TXAGUVteHPpfKwKqL1La6W`,
                        approvals: [
                            Some(
                                ed25519:F816hgJod7nPfD2qQz5yhaKDMn1JXmvzj2iXegsJpsmPNnYYZpKYJXgyuVTVJ4TKQbcJ2Q3USCGZF6fX2TcwBBv,
                            ),
                        ],
                        signature: ed25519:239NbE4BuJaxneQA3AEsPrsGY7v3wBgaezbgg56HER69zPrBoc3a4fbyVWPXeoKE3LvgGma1g6pSHk9QHkmETCZY,
                        latest_protocol_version: 43,
                    },
                    chunks: [
                        ChunkHeaderView {
                            chunk_hash: `2M2oeNFBbUUnHfkU1UuBr8EKBCLMH9xr2vfsGRpyiBmA`,
                            prev_block_hash: `2vJNJca72pBiq2eETq2xvuoc6caKDaUkdRgtdefyutbA`,
                            outcome_root: `11111111111111111111111111111111`,
                            prev_state_root: `3gZPPijaumgMRCvMuuZZM1Ab2LoHTSfYigMKwLqZ67m6`,
                            encoded_merkle_root: `79Bt7ivt9Qhp3c6dJYnueaTyPVweYxZRpQHASRRAiyuy`,
                            encoded_length: 8,
                            height_created: 63596,
                            height_included: 63596,
                            shard_id: 0,
                            gas_used: 0,
                            gas_limit: 1000000000000000,
                            rent_paid: 0,
                            validator_reward: 0,
                            balance_burnt: 0,
                            outgoing_receipts_root: `H4Rd6SGeEBTbxkitsCdzfu9xL9HtZ2eHoPCQXUeZ6bW4`,
                            tx_root: `11111111111111111111111111111111`,
                            validator_proposals: [],
                            signature: ed25519:2vWNayBzEoW5DRc7gTdhxdLbkKuK6ACQ78p3JGpKSAZZCarnLroeoALPAFwpr9ZNPxBqdVYh9QLBe7WHZebsS17Z,
                        },
                    ],
                },
                shards: [
                    IndexerShard {
                        shard_id: 0,
                        chunk: Some(
                            IndexerChunkView {
                                author: "test.near",
                                header: ChunkHeaderView {
                                    chunk_hash: `2M2oeNFBbUUnHfkU1UuBr8EKBCLMH9xr2vfsGRpyiBmA`,
                                    prev_block_hash: `2vJNJca72pBiq2eETq2xvuoc6caKDaUkdRgtdefyutbA`,
                                    outcome_root: `11111111111111111111111111111111`,
                                    prev_state_root: `3gZPPijaumgMRCvMuuZZM1Ab2LoHTSfYigMKwLqZ67m6`,
                                    encoded_merkle_root: `79Bt7ivt9Qhp3c6dJYnueaTyPVweYxZRpQHASRRAiyuy`,
                                    encoded_length: 8,
                                    height_created: 63596,
                                    height_included: 0,
                                    shard_id: 0,
                                    gas_used: 0,
                                    gas_limit: 1000000000000000,
                                    rent_paid: 0,
                                    validator_reward: 0,
                                    balance_burnt: 0,
                                    outgoing_receipts_root: `H4Rd6SGeEBTbxkitsCdzfu9xL9HtZ2eHoPCQXUeZ6bW4`,
                                    tx_root: `11111111111111111111111111111111`,
                                    validator_proposals: [],
                                    signature: ed25519:2vWNayBzEoW5DRc7gTdhxdLbkKuK6ACQ78p3JGpKSAZZCarnLroeoALPAFwpr9ZNPxBqdVYh9QLBe7WHZebsS17Z,
                                },
                                transactions: [
                                    IndexerTransactionWithOutcome {
                                        transaction: SignedTransactionView {
                                            signer_id: "test.near",
                                            public_key: ed25519:8NA7mh6TAWzy2qz68bHp62QHTEQ6nJLfiYeKDRwEbU3X,
                                            nonce: 1,
                                            receiver_id: "some.test.near",
                                            actions: [
                                                CreateAccount,
                                                Transfer {
                                                    deposit: 40000000000000000000000000,
                                                },
                                                AddKey {
                                                    public_key: ed25519:2syGhqwJ8ba2nUGmP9tkZn9m1DYZPYYobpufiERVnug8,
                                                    access_key: AccessKeyView {
                                                        nonce: 0,
                                                        permission: FullAccess,
                                                    },
                                                },
                                            ],
                                            signature: ed25519:Qniuu7exnr6xbe6gKafV5vDhuwM1jt9Bn7sCTF6cHfPpYWVJ4Q6kq8RAxKSeLoxbCreVp1XzMMJmXt8YcUqmMYw,
                                            hash: `8dNv9S8rAFwso9fLwfDQXmw5yv5zscDjQpta96pMF6Bi`,
                                        },
                                        outcome: IndexerExecutionOutcomeWithReceipt {
                                            execution_outcome: ExecutionOutcomeWithIdView {
                                                proof: [],
                                                block_hash: `G9v6Fsv94xaa7BRY2N5PFF5PJwT7ec6DPzQK73Yf3CZ6`,
                                                id: `8dNv9S8rAFwso9fLwfDQXmw5yv5zscDjQpta96pMF6Bi`,
                                                outcome: ExecutionOutcomeView {
                                                    logs: [],
                                                    receipt_ids: [
                                                        `CbWu7WYYbYbn3kThs5gcxANrxy7AKLcMcBLxLw8Zq1Fz`,
                                                    ],
                                                    gas_burnt: 424555062500,
                                                    tokens_burnt: 424555062500000000000,
                                                    executor_id: "test.near",
                                                    status: SuccessReceiptId(CbWu7WYYbYbn3kThs5gcxANrxy7AKLcMcBLxLw8Zq1Fz),
                                                },
                                            },
                                            receipt: None,
                                        },
                                    },
                                ],
                                receipts: [
                                    ReceiptView {
                                        predecessor_id: "test.near",
                                        receiver_id: "some.test.near",
                                        receipt_id: `CbWu7WYYbYbn3kThs5gcxANrxy7AKLcMcBLxLw8Zq1Fz`,
                                        receipt: Action {
                                            signer_id: "test.near",
                                            signer_public_key: ed25519:8NA7mh6TAWzy2qz68bHp62QHTEQ6nJLfiYeKDRwEbU3X,
                                            gas_price: 1030000000,
                                            output_data_receivers: [],
                                            input_data_ids: [],
                                            actions: [
                                                CreateAccount,
                                                Transfer {
                                                    deposit: 40000000000000000000000000,
                                                },
                                                AddKey {
                                                    public_key: ed25519:2syGhqwJ8ba2nUGmP9tkZn9m1DYZPYYobpufiERVnug8,
                                                    access_key: AccessKeyView {
                                                        nonce: 0,
                                                        permission: FullAccess,
                                                    },
                                                },
                                            ],
                                        },
                                    },
                                ],
                            },
                        ),
                        receipt_execution_outcomes: [
                            IndexerExecutionOutcomeWithReceipt {
                                execution_outcome: ExecutionOutcomeWithIdView {
                                    proof: [],
                                    block_hash: `BXPB6DQGmBrjARvcgYwS8qKLkyto6dk9NfawGSmfjE9Q`,
                                    id: `CbWu7WYYbYbn3kThs5gcxANrxy7AKLcMcBLxLw8Zq1Fz`,
                                    outcome: ExecutionOutcomeView {
                                        logs: [],
                                        receipt_ids: [
                                            `8vJ1QWM4pffRDnW3c5CxFFV5cMx8wiqxsAqmZTitHvfh`,
                                        ],
                                        gas_burnt: 424555062500,
                                        tokens_burnt: 424555062500000000000,
                                        executor_id: "some.test.near",
                                        status: SuccessValue(``),
                                    },
                                },
                                receipt: ReceiptView {
                                    predecessor_id: "test.near",
                                    receiver_id: "some.test.near",
                                    receipt_id: `CbWu7WYYbYbn3kThs5gcxANrxy7AKLcMcBLxLw8Zq1Fz`,
                                    receipt: Action {
                                        signer_id: "test.near",
                                        signer_public_key: ed25519:8NA7mh6TAWzy2qz68bHp62QHTEQ6nJLfiYeKDRwEbU3X,
                                        gas_price: 1030000000,
                                        output_data_receivers: [],
                                        input_data_ids: [],
                                        actions: [
                                            CreateAccount,
                                            Transfer {
                                                deposit: 40000000000000000000000000,
                                            },
                                            AddKey {
                                                public_key: ed25519:2syGhqwJ8ba2nUGmP9tkZn9m1DYZPYYobpufiERVnug8,
                                                access_key: AccessKeyView {
                                                    nonce: 0,
                                                    permission: FullAccess,
                                                },
                                            },
                                        ],
                                    },
                                },
                            },
                        ],
                    },
                ],
                state_changes: [
                    StateChangeWithCauseView {
                        cause: ValidatorAccountsUpdate,
                        value: AccountUpdate {
                            account_id: "test.near",
                            account: AccountView {
                                amount: 1000000000000000000000000000000000,
                                locked: 50000000000000000000000000000000,
                                code_hash: `11111111111111111111111111111111`,
                                storage_usage: 182,
                                storage_paid_at: 0,
                            },
                        },
                    },
                ],
            }
        */

        info!(
            target: "borealis_indexer",
            "Message producer loop executed: message received\n"
        );

        // Stream message to NATS
        match msg_format {
            MsgFormat::CBOR => {
                nc.publish(
                    format!("{}_{:?}", subject, msg_format).as_str(),
                    BorealisMessage::new(
                        streamer_message.block.header.height,
                        &streamer_message,
                    ).to_cbor(),
                )
                .expect("[CBOR bytes vector] Message passing error");
            }
            MsgFormat::JSON => {
                nc.publish(
                    format!("{}_{:?}", subject, msg_format).as_str(),
                    BorealisMessage::new(
                        streamer_message.block.header.height,
                        &streamer_message,
                    ).to_json_bytes(),
                )
                .expect("[JSON bytes vector] Message passing error");
            }
        }

        // Data handling from `StreamerMessage` data structure. For custom filtering purposes.
        // Same as: jq '{block_height: .block.header.height, block_hash: .block.header.hash, block_header_chunk: .block.chunks[0], shard_chunk_header: .shards[0].chunk.header, transactions: .shards[0].chunk.transactions, receipts: .shards[0].chunk.receipts, receipt_execution_outcomes: .shards[0].receipt_execution_outcomes, state_changes: .state_changes}'

        info!(
            target: "borealis_indexer",
            "block_height: #{}, block_hash: {}\n",
            &streamer_message.block.header.height,
            &streamer_message.block.header.hash
        );

        if let Some(_verbosity_level) = verbosity_level {
            println!(
                "block_height: #{}, block_hash: {}\n",
                &streamer_message.block.header.height, &streamer_message.block.header.hash
            );
        };

        if let Some(VerbosityLevel::WithStreamerMessageDump) | Some(VerbosityLevel::WithStreamerMessageParse) = verbosity_level {
            println!(
                "streamer_message: {}\n",
                serde_json::to_string_pretty(&streamer_message).unwrap()
            );
            println!(
                "streamer_message: {}\n",
                serde_json::to_string(&streamer_message).unwrap()
            );
        };

        if let Some(VerbosityLevel::WithStreamerMessageParse) = verbosity_level {
            println!(
                "streamer_message: {}\n",
                serde_json::to_value(&streamer_message).unwrap()
            );
            println!(
                "streamer_message: {:?}\n",
                cbor::to_vec(&streamer_message).unwrap()
            );

            println!(
                "block_header: {}\n",
                serde_json::to_value(&streamer_message.block.header).unwrap()
            );
            println!(
                "block_header: {:?}\n",
                cbor::to_vec(&streamer_message.block.header).unwrap()
            );

            println!(
                "block_header_chunks#: {}\n",
                streamer_message.block.chunks.len()
            );
            streamer_message.block.chunks.iter().for_each(|chunk| {
                println!(
                    "block_header_chunk: {}\n",
                    serde_json::to_value(&chunk).unwrap()
                );
                println!("block_header_chunk: {:?}\n", cbor::to_vec(&chunk).unwrap());
            });

            println!("shards#: {}\n", streamer_message.shards.len());
            streamer_message.shards.iter().for_each(|shard| {
                if let Some(chunk) = &shard.chunk {
                    println!(
                        "shard_chunk_header: {}\n",
                        serde_json::to_value(&chunk.header).unwrap()
                    );
                    println!(
                        "shard_chunk_header: {:?}\n",
                        cbor::to_vec(&chunk.header).unwrap()
                    );

                    println!("shard_chunk_transactions#: {}\n", chunk.transactions.len());
                    println!(
                        "shard_chunk_transactions: {}\n",
                        serde_json::to_value(&chunk.transactions).unwrap()
                    );
                    println!(
                        "shard_chunk_transactions: {:?}\n",
                        cbor::to_vec(&chunk.transactions).unwrap()
                    );

                    println!("shard_chunk_receipts#: {}\n", chunk.receipts.len());
                    println!(
                        "shard_chunk_receipts: {}\n",
                        serde_json::to_value(&chunk.receipts).unwrap()
                    );
                    println!(
                        "shard_chunk_receipts: {:?}\n",
                        cbor::to_vec(&chunk.receipts).unwrap()
                    );
                } else {
                    println!("shard_chunk_header: None\n");

                    println!("shard_chunk_transactions#: None\n");
                    println!("shard_chunk_transactions: None\n");

                    println!("shard_chunk_receipts#: None\n");
                    println!("shard_chunk_receipts: None\n");
                };

                println!(
                    "shard_receipt_execution_outcomes#: {}\n",
                    shard.receipt_execution_outcomes.len()
                );
                println!(
                    "shard_receipt_execution_outcomes: {}\n",
                    serde_json::to_value(&shard.receipt_execution_outcomes).unwrap()
                );
                println!(
                    "shard_receipt_execution_outcomes: {:?}\n",
                    cbor::to_vec(&shard.receipt_execution_outcomes).unwrap()
                );
            });

            println!("StateChanges#: {}\n", streamer_message.state_changes.len());
            streamer_message
                .state_changes
                .iter()
                .for_each(|state_change| {
                    println!(
                        "StateChange: {}\n",
                        serde_json::to_value(&state_change).unwrap()
                    );
                    println!("StateChange: {:?}\n", cbor::to_vec(&state_change).unwrap());
                });
        };
    }
}

/// Create connection to Borealis NATS Bus
fn nats_connect(connect_args: RunArgs) -> nats::Connection {
    let creds_path = connect_args
        .creds_path
        .unwrap_or(std::path::PathBuf::from("./.nats/seed/nats.creds"));

    let options = match (
        connect_args.root_cert_path,
        connect_args.client_cert_path,
        connect_args.client_private_key,
    ) {
        (Some(root_cert_path), None, None) => {
            nats::Options::with_credentials(creds_path)
                .with_name("Borealis Indexer [TLS, Server Auth]")
                .tls_required(true)
                .add_root_certificate(root_cert_path)
                .reconnect_buffer_size(1024 * 1024 * 1024)
                .max_reconnects(100000)
                .reconnect_callback(|| info!(target: "borealis_indexer", "connection has been reestablished"))
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
                        target: "borealis_indexer",
                        "reconnection attempt #{} within delay of {:?} ...",
                        reconnect_attempt, delay
                    );
                    delay
                })
                .disconnect_callback(|| info!(target: "borealis_indexer", "connection has been lost")) // todo: re-run message producer
                .close_callback(|| info!(target: "borealis_indexer", "connection has been closed")) // todo: re-run message producer
        }
        (Some(root_cert_path), Some(client_cert_path), Some(client_private_key)) => {
            nats::Options::with_credentials(creds_path)
                .with_name("Borealis Indexer [TLS, Server Auth, Client Auth]")
                .tls_required(true)
                .add_root_certificate(root_cert_path)
                .client_cert(client_cert_path, client_private_key)
                .reconnect_buffer_size(1024 * 1024 * 1024)
                .max_reconnects(100000)
                .reconnect_callback(|| info!(target: "borealis_indexer", "connection has been reestablished"))
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
                        target: "borealis_indexer",
                        "reconnection attempt #{} within delay of {:?} ...",
                        reconnect_attempt, delay
                    );
                    delay
                })
                .disconnect_callback(|| info!(target: "borealis_indexer", "connection has been lost")) // todo: re-run message producer
                .close_callback(|| info!(target: "borealis_indexer", "connection has been closed")) // todo: re-run message producer
        }
        _ => {
            nats::Options::with_credentials(creds_path)
                .with_name("Borealis Indexer [NATS, without TLS]")
                .reconnect_buffer_size(1024 * 1024 * 1024)
                .max_reconnects(100000)
                .reconnect_callback(|| info!(target: "borealis_indexer", "connection has been reestablished"))
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
                        target: "borealis_indexer",
                        "reconnection attempt #{} within delay of {:?} ...",
                        reconnect_attempt, delay
                    );
                    delay
                })
                .disconnect_callback(|| info!(target: "borealis_indexer", "connection has been lost")) // todo: re-run message producer
                .close_callback(|| info!(target: "borealis_indexer", "connection has been closed")) // todo: re-run message producer
        }
    };

    let nats_connection = options
        .connect(connect_args.nats_server.as_str())
        .expect("NATS connection error or wrong credentials");

    nats_connection
}

/// Check connection to Borealis NATS Bus
fn nats_check_connection(nats_connection: nats::Connection) {
//  info!(target: "borealis_indexer", "NATS Connection: {:?}", nats_connection);
    info!(target: "borealis_indexer", "round trip time (rtt) between this client and the current NATS server: {:?}", nats_connection.rtt());
    info!(target: "borealis_indexer", "this client IP address, as known by the current NATS server: {:?}", nats_connection.client_ip());
    info!(target: "borealis_indexer", "this client ID, as known by the current NATS server: {:?}", nats_connection.client_id());
    info!(target: "borealis_indexer", "maximum payload size the current NATS server will accept: {:?}", nats_connection.max_payload());
}

fn main() {
    // Search for the root certificates to perform HTTPS/TLS calls
    // for downloading genesis and config files
    openssl_probe::init_ssl_cert_env_vars();

    // Initialize logging
    init_logging();

    // Parse CLI options
    let opts: Opts = Opts::parse();

    // let home_dir = opts.home_dir.unwrap_or(std::path::PathBuf::from(near_indexer::get_default_home()));
    let home_dir = opts
        .home_dir
        .unwrap_or(std::path::PathBuf::from("./.borealis-indexer"));

    match opts.subcmd {
        SubCommand::Check(run_args) => {
            let nats_connection = nats_connect(run_args);
            nats_check_connection(nats_connection.to_owned());
        }
        SubCommand::Init(config_args) => {
            near_indexer::indexer_init_configs(&home_dir, config_args.into())
                .expect("Error while creating Indexer's initial configuration files");
        }
        SubCommand::Run(run_args) => {
            let indexer_config = near_indexer::IndexerConfig {
                home_dir,
                // recover and continue message streaming from latest synced block (real-time), or from interruption, or from exact block height
                sync_mode: match run_args.sync_mode {
                    SyncMode::LatestSynced => near_indexer::SyncModeEnum::LatestSynced,
                    SyncMode::FromInterruption => near_indexer::SyncModeEnum::FromInterruption,
                    SyncMode::BlockHeight => near_indexer::SyncModeEnum::BlockHeight(run_args.block_height.unwrap_or(0)),
                },
                // waiting for full sync or stream messages while syncing
                await_for_node_synced: match run_args.await_synced {
                    AwaitSynced::WaitForFullSync => near_indexer::AwaitForNodeSyncedEnum::WaitForFullSync,
                    AwaitSynced::StreamWhileSyncing => near_indexer::AwaitForNodeSyncedEnum::StreamWhileSyncing,
                },
            };

            let nats_connection = nats_connect(run_args.to_owned());

            let system = actix::System::new();
            system.block_on(async move {
                let indexer = near_indexer::Indexer::new(indexer_config)
                    .expect("Error while creating Indexer instance");
                let events_stream = indexer.streamer();
                actix::spawn(message_producer(
                    events_stream,
                    nats_connection,
                    run_args.subject,
                    run_args.msg_format,
                    opts.verbose,
                ));
            });
            system.run().unwrap();
        }
    }
}
