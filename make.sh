#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

if [[ "$1" == "help" || "$1" == "h" || "$1" == "?" ]]; then
    echo
    echo "bash $0 [ ? | h | help | fmt | check | build | genconf | run ]"

elif [[ "$1" == "fmt" ]]; then

    echo "--check works since cargo-fmt 1.4.38"
    cargo fmt -v --all --check ;

    read -n 1 -s -p "Proceed with cargo fmt/check/build? [y|n] : " choice_fmt
    echo -e "\n"

    if [[ $choice_fmt == "y" ]]; then
        cargo fmt -v --all ;
    else
        echo
        echo "Canceled"
    fi

elif [[ "$1" == "check" ]]; then

    cargo check ;

    cargo clippy ;

elif [[ "$1" == "build" ]]; then

    cargo build
    # cargo build --release

elif [[ "$1" == "genconf" ]]; then

    read -n 1 -s -p "Proceed with indexer initial configuration? [y|n] : " choice_init
    echo -e "\n"

    if [[ $choice_init == "y" ]]; then
        ./target/debug/aurora-indexer init
        # ./target/release/aurora-indexer init
    else
        echo
        echo "Canceled"
    fi

elif [[ "$1" == "run" ]]; then

    read -n 1 -s -p "Proceed with indexer run? [y|n] : " choice_run
    echo -e "\n"

    if [[ $choice_run == "y" ]]; then
        ./target/debug/aurora-indexer run
        # ./target/release/aurora-indexer run
        # | jq '{block_height: .block.header.height, block_hash: .block.header.hash, block_header_chunks: .block.chunks, shard_chunk_header: .shards[0].chunk.header, transactions: .shards[0].chunk.transactions, receipts: .shards[0].chunk.receipts, receipt_execution_outcomes: .shards[0].receipt_execution_outcomes, state_changes: .state_changes}'
    else
        echo
        echo "Canceled"
    fi

fi


# Examples of how to init & run indexer with custom configuration parameters

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

