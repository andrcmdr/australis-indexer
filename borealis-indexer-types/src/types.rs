use serde::{Deserialize, Serialize};

pub use near_primitives::hash::CryptoHash;
pub use near_primitives::{types, views};

use std::time::SystemTime;

pub const BTC_EPOCH: u64 = 1231006505; // Bitcoin genesis, 2009-01-03T18:15:05Z

/// Based on https://github.com/aurora-is-near/borealis.go/blob/a17d266a7a4e0918db743db474332e8474e90f35/raw_event.go#L18
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RawEvent<PayloadType> {
    pub type_: u16,
    pub sequential_id: u64,
    pub timestamp_s: u32,
    pub timestamp_ms: u16,
    pub unique_id: [u8; 16],
    //  payload can be an object, string or JSON string for further payload serialization and deserialization with the whole RawEvent message to/from CBOR or JSON format with a byte vector representration for a message transmission
    pub payload: PayloadType,
}

impl<PayloadType> std::ops::Deref for RawEvent<PayloadType> {
    type Target = PayloadType;

    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

impl<PayloadType> RawEvent<PayloadType> {
    pub fn new(type_: u16, sequential_id: u64, payload: PayloadType) -> Self {
        let now = SystemTime::UNIX_EPOCH.elapsed().unwrap();
        let timestamp_s = (now.as_secs() - BTC_EPOCH) as u32;
        let timestamp_ms = (now.as_millis() % 1000) as u16;
        let unique_id = rand::random();

        Self {
            type_,
            sequential_id,
            timestamp_s,
            timestamp_ms,
            unique_id,
            payload,
        }
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
