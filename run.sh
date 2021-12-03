#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

read -n 1 -s -p "Proceed with indexer run? [press any key to continue] : " choice
echo -e "\n"

./target/debug/aurora-indexer run
# ./target/release/aurora-indexer run
# | jq '{block_height: .block.header.height, block_hash: .block.header.hash, block_header_chunks: .block.chunks, shard_chunk_header: .shards[0].chunk.header, transactions: .shards[0].chunk.transactions, receipts: .shards[0].chunk.receipts, receipt_execution_outcomes: .shards[0].receipt_execution_outcomes, state_changes: .state_changes}'

# ./target/release/aurora-indexer --home-dir ./.near/localnet/ init --chain-id localnet --download-genesis --download-config
# ./target/release/aurora-indexer --home-dir ./.near/localnet/ run

# ./target/release/aurora-indexer --home-dir ./.near/devnet/ init --chain-id devnet --download-genesis --download-config
# ./target/release/aurora-indexer --home-dir ./.near/devnet/ run

# ./target/release/aurora-indexer --home-dir ./.near/testnet/ init --chain-id testnet --download-genesis --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json"
# ./target/release/aurora-indexer --home-dir ./.near/testnet/ run

# ./target/release/aurora-indexer --home-dir ./.near/betanet/ init --chain-id betanet --download-genesis --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/betanet/config.json"
# ./target/release/aurora-indexer --home-dir ./.near/betanet/ run

# ./target/release/aurora-indexer --home-dir ./.near/mainnet/ init --chain-id mainnet --download-genesis --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json"
# ./target/release/aurora-indexer --home-dir ./.near/mainnet/ run

