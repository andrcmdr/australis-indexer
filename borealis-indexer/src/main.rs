use borealis_types::prelude::BorealisMessage;
use clap::Clap;
use configs::{
    init_logging, AwaitSynced, MsgFormat, Opts, RunArgs, SubCommand, SyncMode, VerbosityLevel, Error,
};
use nats;
use near_indexer;
use serde_cbor as cbor;
use serde_json;
use tokio::sync::{mpsc, broadcast};
use tokio::runtime;
use core::sync::atomic::{AtomicUsize, Ordering};
use tracing::info;

pub mod configs;

async fn message_producer(
    mut events_stream: mpsc::Receiver<near_indexer::StreamerMessage>,
    mut actual_connection_rx: broadcast::Receiver<NATSConnection>,
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

        let nats_connection = {
            match actual_connection_rx.recv().await {
                Ok(nats_connection) => Ok(nats_connection),
                Err(_error) => actual_connection_rx.recv().await,
            }
        }.unwrap().connection.unwrap();

        // Stream message to NATS
        match msg_format {
            MsgFormat::Cbor => {
                nats_connection.publish(
                    format!("{}_{}", subject, msg_format.to_string()).as_str(),
                    BorealisMessage::new(streamer_message.block.header.height, &streamer_message)
                        .to_cbor(),
                )
                .expect("[CBOR bytes vector] Message passing error");
            }
            MsgFormat::Json => {
                nats_connection.publish(
                    format!("{}_{}", subject, msg_format.to_string()).as_str(),
                    BorealisMessage::new(streamer_message.block.header.height, &streamer_message)
                        .to_json_bytes(),
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

        if let Some(VerbosityLevel::WithStreamerMessageDump)
        | Some(VerbosityLevel::WithStreamerMessageParse) = verbosity_level
        {
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

#[derive(Debug, Clone)]
enum ConnectionEvent {
    ConnectionReestablished,
    ConnectionLost,
    ConnectionClosed,
}

#[derive(Debug, Clone)]
struct NATSConnection {
    connection: Option<nats::Connection>,
}

impl NATSConnection
    where
        Self: Send + Sync + 'static,
{
    fn new() -> NATSConnection {
        NATSConnection {
            connection: None,
        }
    }

    /// Create options for connection to Borealis NATS Bus
    fn options(self, connect_args: RunArgs, connection_event_tx: mpsc::Sender<ConnectionEvent>) -> nats::Options {
        let connection_reestablished_event = connection_event_tx.clone();
        let connection_lost_event = connection_event_tx.clone();
        let connection_closed_event = connection_event_tx.clone();
        
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
                    .reconnect_delay_callback(|reconnect_try| {
                        let reconnect_attempt = {
                            if reconnect_try == 0 {
                                1 as usize
                            } else {
                                reconnect_try
                            }
                        };
                        let delay = core::time::Duration::from_millis(std::cmp::min(
                            (reconnect_attempt
                                * rand::Rng::gen_range(&mut rand::thread_rng(), 100..1000))
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
                    .reconnect_callback( move || {
                        info!(target: "borealis_indexer", "connection has been reestablished");
                        connection_reestablished_event.blocking_send(ConnectionEvent::ConnectionReestablished).unwrap();
                    })
                    .disconnect_callback( move || {
                        info!(target: "borealis_indexer", "connection has been lost");
                        connection_lost_event.blocking_send(ConnectionEvent::ConnectionLost).unwrap();
                    })
                    .close_callback(move || {
                        info!(target: "borealis_indexer", "connection has been closed");
                        connection_closed_event.blocking_send(ConnectionEvent::ConnectionClosed).unwrap();
                    })
            }
            (Some(root_cert_path), Some(client_cert_path), Some(client_private_key)) => {
                nats::Options::with_credentials(creds_path)
                    .with_name("Borealis Indexer [TLS, Server Auth, Client Auth]")
                    .tls_required(true)
                    .add_root_certificate(root_cert_path)
                    .client_cert(client_cert_path, client_private_key)
                    .reconnect_buffer_size(1024 * 1024 * 1024)
                    .max_reconnects(100000)
                    .reconnect_delay_callback(|reconnect_try| {
                        let reconnect_attempt = {
                            if reconnect_try == 0 {
                                1 as usize
                            } else {
                                reconnect_try
                            }
                        };
                        let delay = core::time::Duration::from_millis(std::cmp::min(
                            (reconnect_attempt
                                * rand::Rng::gen_range(&mut rand::thread_rng(), 100..1000))
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
                    .reconnect_callback( move || {
                        info!(target: "borealis_indexer", "connection has been reestablished");
                        connection_reestablished_event.blocking_send(ConnectionEvent::ConnectionReestablished).unwrap();
                    })
                    .disconnect_callback( move || {
                        info!(target: "borealis_indexer", "connection has been lost");
                        connection_lost_event.blocking_send(ConnectionEvent::ConnectionLost).unwrap();
                    })
                    .close_callback(move || {
                        info!(target: "borealis_indexer", "connection has been closed");
                        connection_closed_event.blocking_send(ConnectionEvent::ConnectionClosed).unwrap();
                    })
           }
            _ => {
                nats::Options::with_credentials(creds_path)
                    .with_name("Borealis Indexer [NATS, without TLS]")
                    .reconnect_buffer_size(1024 * 1024 * 1024)
                    .max_reconnects(100000)
                    .reconnect_delay_callback(|reconnect_try| {
                        let reconnect_attempt = {
                            if reconnect_try == 0 {
                                1 as usize
                            } else {
                                reconnect_try
                            }
                        };
                        let delay = core::time::Duration::from_millis(std::cmp::min(
                            (reconnect_attempt
                                * rand::Rng::gen_range(&mut rand::thread_rng(), 100..1000))
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
                    .reconnect_callback( move || {
                        info!(target: "borealis_indexer", "connection has been reestablished");
                        connection_reestablished_event.blocking_send(ConnectionEvent::ConnectionReestablished).unwrap();
                    })
                    .disconnect_callback( move || {
                        info!(target: "borealis_indexer", "connection has been lost");
                        connection_lost_event.blocking_send(ConnectionEvent::ConnectionLost).unwrap();
                    })
                    .close_callback(move || {
                        info!(target: "borealis_indexer", "connection has been closed");
                        connection_closed_event.blocking_send(ConnectionEvent::ConnectionClosed).unwrap();
                    })
           }
        };
        options
    }

    /// Create connection to Borealis NATS Bus
    fn connect(connect_args: RunArgs, connection_event_tx: mpsc::Sender<ConnectionEvent>, actual_connection_tx: broadcast::Sender<NATSConnection>) -> Result<Self, Error> {
        let nats_connection_initial = NATSConnection::new();
        let connection_options = nats_connection_initial.options(connect_args.to_owned(), connection_event_tx);

        if let Ok(nats_connection) = connection_options.connect(connect_args.nats_server.as_str()) {
            actual_connection_tx.send(Self { connection: Some(nats_connection.clone()) }).unwrap();
            return Ok(Self { connection: Some(nats_connection) })
        } else {
            Err("NATS connection error or wrong credentials".to_string().into())
        }
    }

    /// Use already existed connection to Borealis NATS Bus or recreate new connection to prevent connection issues
    fn try_connect(self, connect_args: RunArgs, connection_event_tx: mpsc::Sender<ConnectionEvent>, actual_connection_tx: broadcast::Sender<NATSConnection>) -> Result<Self, Error> {
        let nats_connection_initial = NATSConnection::new();
        let connection_options = nats_connection_initial.options(connect_args.to_owned(), connection_event_tx);

        if let Ok(rtt_duration) = self.connection.as_ref().unwrap().rtt() {
            // info!(target: "borealis_indexer", "NATS Connection: {:?}", connection.unwrap());
            info!(target: "borealis_indexer", "round trip time (rtt) between this client and the current NATS server: {:?}", rtt_duration);
            actual_connection_tx.send(self.clone()).unwrap();
            return Ok(self)
        } else if let Ok(nats_connection) = connection_options.connect(connect_args.nats_server.as_str()) {
            actual_connection_tx.send(Self { connection: Some(nats_connection.clone()) }).unwrap();
            return Ok(Self { connection: Some(nats_connection) })
        } else {
            Err("NATS connection error or wrong credentials".to_string().into())
        }
    }

    /// Check connection to Borealis NATS Bus
    fn nats_check_connection(self) {
        let nats_connection = self.connection.unwrap();
        // info!(target: "borealis_indexer", "NATS Connection: {:?}", connection.unwrap());
        info!(target: "borealis_indexer", "round trip time (rtt) between this client and the current NATS server: {:?}", nats_connection.rtt());
        info!(target: "borealis_indexer", "this client IP address, as known by the current NATS server: {:?}", nats_connection.client_ip());
        info!(target: "borealis_indexer", "this client ID, as known by the current NATS server: {:?}", nats_connection.client_id());
        info!(target: "borealis_indexer", "maximum payload size the current NATS server will accept: {:?}", nats_connection.max_payload());
    }
}

fn main() -> Result<(), Error> {
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

    let events_processing = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn( || {
            static THREAD_ID: AtomicUsize = AtomicUsize::new(0);
            let thread_id = THREAD_ID.fetch_add(1, Ordering::SeqCst);
            format!("connection-events-processing-{}", thread_id)
         })
        .on_thread_start( || {
            info!(target: "borealis_indexer", "NATS connection events processing runtime: thread starting");
        })
        .on_thread_stop( || {
            info!(target: "borealis_indexer", "NATS connection events processing runtime: thread stopping");
        })
        .on_thread_park( || {
            info!(target: "borealis_indexer", "NATS connection events processing runtime: thread parking and going idle");
        })
        .on_thread_unpark( || {
            info!(target: "borealis_indexer", "NATS connection events processing runtime: thread unparked and starts executing tasks");
        })
        .build()?;

    let messages_processing = runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn( || {
            static THREAD_ID: AtomicUsize = AtomicUsize::new(0);
            let thread_id = THREAD_ID.fetch_add(1, Ordering::SeqCst);
            format!("streamer-messages-processing-{}", thread_id)
         })
        .on_thread_start( || {
            info!(target: "borealis_indexer", "Streamer Messages processing runtime: thread starting");
        })
        .on_thread_stop( || {
            info!(target: "borealis_indexer", "Streamer Messages processing runtime: thread stopping");
        })
        .on_thread_park( || {
            info!(target: "borealis_indexer", "Streamer Messages processing runtime: thread parking and going idle");
        })
        .on_thread_unpark( || {
            info!(target: "borealis_indexer", "Streamer Messages processing runtime: thread unparked and starts executing tasks");
        })
        .build()?;

    let (connection_event_tx, mut connection_event_rx) = mpsc::channel::<ConnectionEvent>(1);
    let (actual_connection_tx, mut actual_connection_rx) = broadcast::channel::<NATSConnection>(1);

    let connection_event_sender = connection_event_tx.clone();
    let actual_connection_sender = actual_connection_tx.clone();
    let actual_connection_receiver = actual_connection_tx.subscribe();

    // let events_processing_handle = events_processing.handle();
    // events_processing_handle.enter();
    // tokio::spawn(future);

    if let SubCommand::Check(run_args) | SubCommand::Run(run_args) = opts.subcmd.clone() {
        events_processing.spawn(async move {
            while let Some(event) = connection_event_rx.recv().await {
                match event {
                    ConnectionEvent::ConnectionReestablished => {
                        info!(target: "borealis_indexer", "connection has been reestablished, thus checking connection is active...");
                        let nats_connection = {
                            match actual_connection_rx.recv().await {
                                Ok(nats_connection) => Ok(nats_connection),
                                Err(_error) => actual_connection_rx.recv().await,
                            }
                        }.unwrap();
                        let _nats_connection_actual = nats_connection.try_connect(run_args.to_owned(), connection_event_tx.clone(), actual_connection_tx.clone()).unwrap();
                    },
                    ConnectionEvent::ConnectionLost => {
                        info!(target: "borealis_indexer", "connection has been lost, thus retrieving connection...");
                        let nats_connection = {
                            match actual_connection_rx.recv().await {
                                Ok(nats_connection) => Ok(nats_connection),
                                Err(_error) => actual_connection_rx.recv().await,
                            }
                        }.unwrap();
                        let _nats_connection_actual = nats_connection.try_connect(run_args.to_owned(), connection_event_tx.clone(), actual_connection_tx.clone()).unwrap();
                    },
                    ConnectionEvent::ConnectionClosed => {
                        info!(target: "borealis_indexer", "connection has been closed, thus retrieving connection...");
                        let nats_connection = {
                            match actual_connection_rx.recv().await {
                                Ok(nats_connection) => Ok(nats_connection),
                                Err(_error) => actual_connection_rx.recv().await,
                            }
                        }.unwrap();
                        let _nats_connection_actual = nats_connection.try_connect(run_args.to_owned(), connection_event_tx.clone(), actual_connection_tx.clone()).unwrap();
                    },
                }
            }
        });
    };

    match opts.subcmd {
        SubCommand::Check(run_args) => {
            let nats_connection = NATSConnection::connect(run_args.to_owned(), connection_event_sender.clone(),  actual_connection_sender.clone()).unwrap();
            let nats_connection_actual = nats_connection.try_connect(run_args.to_owned(), connection_event_sender.clone(), actual_connection_sender.clone()).unwrap();
            nats_connection_actual.nats_check_connection();
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
                    SyncMode::BlockHeight => {
                        near_indexer::SyncModeEnum::BlockHeight(run_args.block_height.unwrap_or(0))
                    }
                },
                // waiting for full sync or stream messages while syncing
                await_for_node_synced: match run_args.await_synced {
                    AwaitSynced::WaitForFullSync => {
                        near_indexer::AwaitForNodeSyncedEnum::WaitForFullSync
                    }
                    AwaitSynced::StreamWhileSyncing => {
                        near_indexer::AwaitForNodeSyncedEnum::StreamWhileSyncing
                    }
                },
            };

            let nats_connection = NATSConnection::connect(run_args.to_owned(), connection_event_sender.clone(),  actual_connection_sender.clone()).unwrap();
            let nats_connection_actual = nats_connection.try_connect(run_args.to_owned(), connection_event_sender.clone(), actual_connection_sender.clone()).unwrap();
            nats_connection_actual.to_owned().nats_check_connection();

            messages_processing.spawn(async move {
                let indexer = near_indexer::Indexer::new(indexer_config)
                    .expect("Error while creating Indexer instance");

                let events_stream = indexer.streamer();

                tokio::spawn(async move { message_producer(
                    events_stream,
                    actual_connection_receiver,
                    run_args.subject,
                    run_args.msg_format,
                    opts.verbose,
                )});
            });
        }
    }
    // Graceful shutdown for all tasks (futures, green threads) currently executed on existed run-time thread-pools
    info!(target: "borealis_indexer", "Shutdown process within 10 seconds...");
    messages_processing.shutdown_timeout(core::time::Duration::from_secs(10));
    events_processing.shutdown_timeout(core::time::Duration::from_secs(10));
    Ok(())
}
