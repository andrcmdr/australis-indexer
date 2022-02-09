# **Borealis Indexer**
Borealis Indexer is an events listener for NEAR's on-chain events, based on NEAR Indexer Framework.

Provides publishing (as producer) of CBOR (or JSON, for debugging) encoded messages about on-chain events with streaming messages to the Borealis Bus, NATS based service-oriented bus (MOM/MQ), for other services (as consumers/subscribers).

Provides NATS based Pub/Sub interface for requesting and fetching data about any block, transaction, etc. from NEAR's block-chain. `[WIP]`

Can be configured and running as NEAR's archival node with access to all blocks from Genesis.

## **Build and run Borealis Indexer using make.sh shell helper and Cargo:**
```
bash ./make.sh [ help/h/? | fmt | check | build | build release | submodules | submodules update | genconf/init | exec | exec_logging | exec_logging_cliout ]
```

```
bash ./make.sh fmt

bash ./make.sh check

bash ./make.sh submodules update

bash ./make.sh submodules

bash ./make.sh build release

bash ./make.sh build
```

## **Usage help for Borealis Indexer (after buidling executables):**
```
./target/debug/borealis-indexer help
./target/debug/borealis-indexer [ check | init | run ] --help
```

## **Examples of how to init & run Indexer with custom configuration parameters):**

#### **Passing of all further parameters to indexer executable also possible through `make.sh` helper.**

#### **Indexer initial configuration:**
```
bash ./make.sh [ init | genconf ] [ localnet | devnet | testnet | betanet | mainnet ] ...
```

#### **Is equivalent to:**
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/[ localnet | devnet | testnet | betanet | mainnet ]/ init --chain-id [ localnet | devnet | testnet | betanet | mainnet ] ...
```

#### **Passing subcommands to indexer:**
```
bash ./make.sh exec ...
```

#### **It's an equivalent to:**
```
./target/debug/borealis-indexer ...
```

#### **Run Indexer with passing subcommands, wrapping Indexer's output and redirection output into log file:**
```
bash ./make.sh exec_logging ...
```

#### **Run Indexer with passing subcommands, wrapping Indexer's output, redirection output into log file and duplicating output to `stdout`:**
```
bash ./make.sh exec_logging_cliout ...
```

## **Fully qualified usage examples for Borealis Indexer.**

### **LocalNet:**
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/localnet/ init --chain-id localnet

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/localnet/ run --root-cert-path ./.nats/seed/root-ca.crt [--client-cert-path ./.nats/seed/client.crt --client-private-key ./.nats/seed/client.key] --creds-path ./.nats/seed/nats.creds --nats-server ["nats://westcoast.nats.backend.aurora.dev:4222,nats://eastcoast.nats.backend.aurora.dev:4222" | "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222"] --subject "BlockIndex_StreamerMessages_localnet" --msg-format ["CBOR" | "JSON"]
```

### **DevNet:**
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/devnet/ init --chain-id devnet

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/devnet/ run --root-cert-path ./.nats/seed/root-ca.crt [--client-cert-path ./.nats/seed/client.crt --client-private-key ./.nats/seed/client.key] --creds-path ./.nats/seed/nats.creds --nats-server ["nats://westcoast.nats.backend.aurora.dev:4222,nats://eastcoast.nats.backend.aurora.dev:4222" | "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222"] --subject "BlockIndex_StreamerMessages_devnet" --msg-format ["CBOR" | "JSON"]
```

### **TestNet:**
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/testnet/ init --chain-id testnet [--boot-nodes "ed25519:4k9csx6zMiXy4waUvRMPTkEtAS2RFKLVScocR5HwN53P@34.73.25.182:24567,ed25519:4keFArc3M4SE1debUQWi3F1jiuFZSWThgVuA2Ja2p3Jv@34.94.158.10:24567,ed25519:D2t1KTLJuwKDhbcD9tMXcXaydMNykA99Cedz7SkJkdj2@35.234.138.23:24567,ed25519:CAzhtaUPrxCuwJoFzceebiThD9wBofzqqEMCiupZ4M3E@34.94.177.51:24567"] [--download-genesis | --download-genesis-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/genesis.json"] [--download-config | --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json"]

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/testnet/ run --root-cert-path ./.nats/seed/root-ca.crt [--client-cert-path ./.nats/seed/client.crt --client-private-key ./.nats/seed/client.key] --creds-path ./.nats/seed/nats.creds --nats-server ["nats://westcoast.nats.backend.aurora.dev:4222,nats://eastcoast.nats.backend.aurora.dev:4222" | "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222"] --subject "BlockIndex_StreamerMessages_testnet" --msg-format ["CBOR" | "JSON"]
```

### **BetaNet:**
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/betanet/ init --chain-id betanet [--boot-nodes "ed25519:7PGseFbWxvYVgZ89K1uTJKYoKetWs7BJtbyXDzfbAcqX@34.94.17.55:24567,ed25519:6DSjZ8mvsRZDvFqFxo8tCKePG96omXW7eVYVSySmDk8e@35.196.178.178:24567"] [--download-genesis | --download-genesis-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/betanet/genesis.json"] [--download-config | --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/betanet/config.json"]

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/betanet/ run --root-cert-path ./.nats/seed/root-ca.crt [--client-cert-path ./.nats/seed/client.crt --client-private-key ./.nats/seed/client.key] --creds-path ./.nats/seed/nats.creds --nats-server ["nats://westcoast.nats.backend.aurora.dev:4222,nats://eastcoast.nats.backend.aurora.dev:4222" | "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222"] --subject "BlockIndex_StreamerMessages_betanet" --msg-format ["CBOR" | "JSON"]
```

### **MainNet:**
```
./target/debug/borealis-indexer --home-dir ./.borealis-indexer/mainnet/ init --chain-id mainnet [--boot-nodes "ed25519:86EtEy7epneKyrcJwSWP7zsisTkfDRH5CFVszt4qiQYw@35.195.32.249:24567,ed25519:BFB78VTDBBfCY4jCP99zWxhXUcFAZqR22oSx2KEr8UM1@35.229.222.235:24567,ed25519:Cw1YyiX9cybvz3yZcbYdG7oDV6D7Eihdfc8eM1e1KKoh@35.195.27.104:24567,ed25519:33g3PZRdDvzdRpRpFRZLyscJdbMxUA3j3Rf2ktSYwwF8@34.94.132.112:24567,ed25519:CDQFcD9bHUWdc31rDfRi4ZrJczxg8derCzybcac142tK@35.196.209.192:24567"] [--download-genesis | --download-genesis-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/genesis.json"] [--download-config | --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json"]

./target/debug/borealis-indexer --home-dir ./.borealis-indexer/mainnet/ run --root-cert-path ./.nats/seed/root-ca.crt [--client-cert-path ./.nats/seed/client.crt --client-private-key ./.nats/seed/client.key] --creds-path ./.nats/seed/nats.creds --nats-server ["nats://westcoast.nats.backend.aurora.dev:4222,nats://eastcoast.nats.backend.aurora.dev:4222" | "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222"] --subject "BlockIndex_StreamerMessages_mainnet" --msg-format ["CBOR" | "JSON"]
```

## **Usage help for Borealis Consumer Client (after buidling executables):**
```
./target/debug/borealis-consumer help
./target/debug/borealis-consumer [ check | init | run ] --help
```

## **Fully qualified usage examples for Borealis Consumer Client:**
```
./target/debug/borealis-consumer [ check | init | run ] --root-cert-path ./.nats/seed/root-ca.crt [--client-cert-path ./.nats/seed/client.crt --client-private-key ./.nats/seed/client.key] --creds-path ./.nats/seed/nats.creds --nats-server ["nats://westcoast.nats.backend.aurora.dev:4222,nats://eastcoast.nats.backend.aurora.dev:4222" | "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222"] --work-mode ["subscriber" | "jetstream"] --subject "BlockIndex_StreamerMessages_mainnet" --msg-format ["CBOR" | "JSON"]
```
