#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

if [[ "$1" == "help" || "$1" == "h" || "$1" == "?" || "$1" == "" ]]; then
    echo
    echo -e "bash $0 [ help/h/? | fmt | check | build | genconf/init | run ]\n"

elif [[ "$1" == "fmt" ]]; then

    echo "--check works since cargo-fmt 1.4.38"
    cargo fmt -v --all --check ;

    read -n 1 -s -p "Proceed with cargo fmt/check/build? [Enter/y|n] : " choice_fmt
    echo -e "\n"

    if [[ $choice_fmt == "y" || $choice_fmt == "" ]]; then
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

elif [[ "$1" == "genconf" || "$1" == "init" ]]; then

    read -n 1 -s -p "Proceed with indexer initial configuration? [Enter/y|n] : " choice_init
    echo -e "\n"

    if [[ $choice_init == "y" || $choice_init == "" ]]; then

        ./target/debug/borealis-indexer init "${@:2}"
        # ./target/release/borealis-indexer init "${@:2}"

        sed -r -s -i"" "s/^(\s*?)\"tracked\_shards\"\:\s\[\]\,/\1\"tracked\_shards\"\:\ \[0\]\,/gI" ./.borealis-indexer/config.json

    else
        echo
        echo "Canceled"
    fi

elif [[ "$1" == "run" ]]; then

    read -n 1 -s -p "Proceed with indexer run? [Enter/y|n] : " choice_run
    echo -e "\n"

    if [[ $choice_run == "y" || $choice_run == "" ]]; then

        ./target/debug/borealis-indexer run  "${@:2}"
        # ./target/release/borealis-indexer run  "${@:2}"
        # | jq '{block_height: .block.header.height, block_hash: .block.header.hash, block_header_chunks: .block.chunks, shard_chunk_header: .shards[0].chunk.header, transactions: .shards[0].chunk.transactions, receipts: .shards[0].chunk.receipts, receipt_execution_outcomes: .shards[0].receipt_execution_outcomes, state_changes: .state_changes}'

    else
        echo
        echo "Canceled"
    fi

fi


# Examples of how to init & run indexer with custom configuration parameters:

# bash ./make.sh init ...
# bash ./make.sh run ...
# or
# ./target/debug/borealis-indexer --home-dir "some_dir" init ...
# ./target/debug/borealis-indexer --home-dir "some_dir" run ...

# ./target/debug/borealis-indexer --home-dir ./.near/localnet/ init --chain-id localnet
# ./target/debug/borealis-indexer --home-dir ./.near/localnet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")

# ./target/debug/borealis-indexer --home-dir ./.near/devnet/ init --chain-id devnet
# ./target/debug/borealis-indexer --home-dir ./.near/devnet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")

# ./target/debug/borealis-indexer --home-dir ./.near/testnet/ init --chain-id testnet --boot-nodes "" --download-genesis --download-genesis-url "" --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json"
# ./target/debug/borealis-indexer --home-dir ./.near/testnet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")

# ./target/debug/borealis-indexer --home-dir ./.near/betanet/ init --chain-id betanet --boot-nodes "" --download-genesis --download-genesis-url "" --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/betanet/config.json"
# ./target/debug/borealis-indexer --home-dir ./.near/betanet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")

# ./target/debug/borealis-indexer --home-dir ./.near/mainnet/ init --chain-id mainnet --boot-nodes "" --download-genesis --download-genesis-url "" --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json"
# ./target/debug/borealis-indexer --home-dir ./.near/mainnet/ run --creds-path "./.nats/seed/nats.creds" --nats-server ("nats://demo.nats.io:4222" | "tls://demo.nats.io:4443") --subject "BlockIndex_StreamerMessages" --msg-format ("CBOR" | "JSON")

