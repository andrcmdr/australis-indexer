# Borealis Indexer
Borealis Indexer is an events listener for NEAR's on-chain events, based on NEAR Indexer Framework.

Provides publishing (as producer) of CBOR (or JSON, for debugging) encoded messages about on-chain events with streaming messages to the Borealis Bus, NATS based service-oriented bus (MOM/MQ), for other services (as consumers/subscribers).

Provides NATS based Pub/Sub interface for requesting and fetching data about any block, transaction, etc. from NEAR's block-chain. `[WIP]`

Can be configured and running as NEAR's archival node with access to all blocks from Genesis.

**Build and run Borealis Indexer using Cargo:**
```
bash ./make.sh [ help/h/? | fmt | check | build | genconf/init | exec ]
```

**Usage help for Borealis Indexer (after buidling executables):**
```
./target/debug/borealis-indexer help
./target/debug/borealis-indexer [ check | init | run ] --help
```

**Examples of how to init & run indexer with custom configuration parameters):**

Passing of all further parameters to indexer executable also possible through `make.sh` helper.

Indexer initial configuration:
```
bash ./make.sh [ init | genconf ] ...
```

Is equivalent to:
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer init ...
```

Passing any subcommand to indexer:
```
bash ./make.sh exec ...
```

It's an equivalent to:
```
./target/debug/borealis-indexer ...
```

**Fully qualified usage examples for Borealis Indexer.**

LocalNet:
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/localnet/ init --chain-id localnet

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/localnet/ run --root-cert-path ./.nats/seed/ca.crt (--client-cert-path ./.nats/seed/client.key --client-private-key ./.nats/seed/client.crt) --creds-path ./.nats/seed/nats.creds --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")
```

DevNet:
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/devnet/ init --chain-id devnet

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/devnet/ run --root-cert-path ./.nats/seed/ca.crt (--client-cert-path ./.nats/seed/client.key --client-private-key ./.nats/seed/client.crt) --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")
```

TestNet:
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/testnet/ init --chain-id testnet --boot-nodes "..." (--download-genesis --download-genesis-url "..." --download-config) --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json"

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/testnet/ run --root-cert-path ./.nats/seed/ca.crt (--client-cert-path ./.nats/seed/client.key --client-private-key ./.nats/seed/client.crt) --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")
```

BetaNet:
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/betanet/ init --chain-id betanet --boot-nodes "..." (--download-genesis --download-genesis-url "..." --download-config) --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/betanet/config.json"

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/betanet/ run --root-cert-path ./.nats/seed/ca.crt (--client-cert-path ./.nats/seed/client.key --client-private-key ./.nats/seed/client.crt) --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")
```

MainNet:
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/mainnet/ init --chain-id mainnet --boot-nodes "..." (--download-genesis --download-genesis-url "..." --download-config) --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json"

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/mainnet/ run --root-cert-path ./.nats/seed/ca.crt (--client-cert-path ./.nats/seed/client.key --client-private-key ./.nats/seed/client.crt) --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")
```

**Usage help for Borealis Consumer Client (after buidling executables):**
```
./target/debug/borealis-consumer help
./target/debug/borealis-consumer [ check | init | run ] --help
```

**Fully qualified usage examples for Borealis Consumer Client:**
```
./target/debug/borealis-consumer [ check | init | run ] --root-cert-path ./.nats/seed/ca.crt (--client-cert-path ./.nats/seed/client.key --client-private-key ./.nats/seed/client.crt) --creds-path ./.nats/seed/nats.creds --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --work-mode ("subscriber" | "jetstream") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")
```
