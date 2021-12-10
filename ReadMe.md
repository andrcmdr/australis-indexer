# Borealis Indexer
Borealis Indexer is an events listener for NEAR's on-chain events, based on NEAR Indexer Framework.

Provides publishing (as producer) of CBOR (or JSON, for debugging) encoded messages about on-chain events with streaming messages to the Borealis Bus, NATS based service-oriented bus (MOM/MQ), for other services (as consumers/subscribers).

Provides NATS based Pub/Sub interface for requesting and fetching data about any block, transaction, etc. from NEAR's block-chain. `[WIP]`

Can be configured and running as NEAR's archival node with access to all blocks from Genesis.

**Make and run Borealis Indexer:**
```
bash ./make.sh [ help/h/? | fmt | check | build | genconf/init | run ]
```

**Usage help for Borealis Indexer and Borealis Consumer (after buidling executables):**
```
./target/debug/borealis-indexer help
./target/debug/borealis-indexer [init | run] --help

./target/debug/borealis-consumer help
./target/debug/borealis-consumer [init | run] --help
```

**Examples of how to init & run indexer with custom configuration parameters):**

Passing of all further parameters to indexer executable also possible through `make.sh` helper:
```
bash ./make.sh init ...
bash ./make.sh run ...
```
It's an equivalent to:
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer init ...
./target/debug/borealis-indexer --home-dir ./.borealis-indexer run ...
```

**Fully qualified usage examples for Borealis Indexer:**
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/localnet/ init --chain-id localnet

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/localnet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")


./target/debug/borealis-indexer --home-dir ./.borealis-indexer/devnet/ init --chain-id devnet

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/devnet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")


./target/debug/borealis-indexer --home-dir ./.borealis-indexer/testnet/ init --chain-id testnet --boot-nodes "" --download-genesis --download-genesis-url "" --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json"

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/testnet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")


./target/debug/borealis-indexer --home-dir ./.borealis-indexer/betanet/ init --chain-id betanet --boot-nodes "" --download-genesis --download-genesis-url "" --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/betanet/config.json"

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/betanet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")


./target/debug/borealis-indexer --home-dir ./.borealis-indexer/mainnet/ init --chain-id mainnet --boot-nodes "" --download-genesis --download-genesis-url "" --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json"

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/mainnet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")
```
