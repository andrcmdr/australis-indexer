# Borealis Indexer
Borealis Indexer is an events listener for NEAR's on-chain events, based on NEAR Indexer Framework.

Provides publishing (as producer) of CBOR (or JSON, for debugging) encoded messages about on-chain events with streaming messages to the Borealis Bus, NATS based service-oriented bus (MOM/MQ), for other services (as consumers/subscribers).

Provides NATS based Pub/Sub interface for requesting and fetching data about any block, transaction, etc. from NEAR's block-chain. `[WIP]`

Can be configured and running as NEAR's archival node with access to all blocks from Genesis.

Make and run Borealis Indexer:
```
bash ./make.sh help
```
