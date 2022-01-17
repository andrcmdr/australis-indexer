use serde::{Deserialize, Serialize};

use serde_cbor as cbor;
use serde_json;

pub use near_primitives::hash::CryptoHash;
pub use near_primitives::{types, views};

use std::time::SystemTime;

pub const VERSION: u8 = 1;
pub const EVENT_TYPE: u16 = 4096;
pub const BOREALIS_EPOCH: u64 = 1231006505; // Conform to Bitcoin genesis, 2009-01-03T18:15:05Z

/// According to:
/// https://github.com/aurora-is-near/borealis.go/blob/a17d266a7a4e0918db743db474332e8474e90f35/raw_event.go#L18-L26
/// https://github.com/aurora-is-near/borealis.go/blob/a17d266a7a4e0918db743db474332e8474e90f35/events.go#L14-L19
/// https://github.com/aurora-is-near/borealis-events#message-format
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RawEvent<T> {
    pub version: u8,
    pub event_type: u16,
    pub sequential_id: u64,
    pub timestamp_s: u32,
    pub timestamp_ms: u16,
    pub unique_id: [u8; 16],
    //  payload can be an object, string or JSON string for further payload serialization and deserialization with the whole RawEvent message to/from CBOR or JSON format with a byte vector representration for a message transmission
    pub payload: T,
}

impl<T> RawEvent<T> {
    pub fn new(sequential_id: u64, payload: T) -> Self {
        let version = VERSION;
        let event_type = EVENT_TYPE;
        let now = SystemTime::UNIX_EPOCH.elapsed().unwrap();
        let timestamp_s = (now.as_secs() - BOREALIS_EPOCH) as u32;
        let timestamp_ms = (now.as_millis() % 1000) as u16;
        let unique_id = rand::random();

        Self {
            version,
            event_type,
            sequential_id,
            timestamp_s,
            timestamp_ms,
            unique_id,
            payload,
        }
    }
}

impl<T> RawEvent<T>
where
    T: Serialize,
{
    pub fn to_cbor(&self) -> Vec<u8> {
        cbor::to_vec(self).expect("[CBOR bytes vector: RawEvent] Message serialization error")
    }

    pub fn to_json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("[JSON bytes vector: BorealisMessage] Message serialization error")
    }
}

impl<'de, T> RawEvent<T>
where
    T: Deserialize<'de>,
{
    pub fn from_cbor(msg: &'de Vec<u8>) -> Self {
        cbor::from_slice(msg).expect("[CBOR bytes vector: RawEvent] Message deserialization error")
    }

    pub fn from_json_bytes(msg: &'de Vec<u8>) -> Self {
        serde_json::from_slice(msg).expect("[JSON bytes vector: BorealisMessage] Message deserialization error")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamerMessage {
    pub block: views::BlockView,
    pub shards: Vec<IndexerShard>,
    pub state_changes: views::StateChangesView,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexerShard {
    pub shard_id: types::ShardId,
    pub chunk: Option<IndexerChunkView>,
    pub receipt_execution_outcomes: Vec<IndexerExecutionOutcomeWithReceipt>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexerChunkView {
    pub author: types::AccountId,
    pub header: views::ChunkHeaderView,
    pub transactions: Vec<IndexerTransactionWithOutcome>,
    pub receipts: Vec<views::ReceiptView>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexerTransactionWithOutcome {
    pub transaction: views::SignedTransactionView,
    pub outcome: IndexerExecutionOutcomeWithOptionalReceipt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexerExecutionOutcomeWithOptionalReceipt {
    pub execution_outcome: views::ExecutionOutcomeWithIdView,
    pub receipt: Option<views::ReceiptView>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndexerExecutionOutcomeWithReceipt {
    pub execution_outcome: views::ExecutionOutcomeWithIdView,
    pub receipt: views::ReceiptView,
}
