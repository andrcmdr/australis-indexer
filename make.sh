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

    read -n 1 -s -p "Proceed with cargo fmt? [Enter/y|n] : " choice_fmt
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

elif [[ "$1" == "exec" ]]; then

    read -n 1 -s -p "Proceed with indexer command pasing? [Enter/y|n] : " choice_exec
    echo -e "\n"

    if [[ $choice_exec == "y" || $choice_exec == "" ]]; then

        ./target/debug/borealis-indexer "${@:2}"
        # ./target/release/borealis-indexer "${@:2}"
        # | jq '{block_height: .block.header.height, block_hash: .block.header.hash, block_header_chunks: .block.chunks, shard_chunk_header: .shards[0].chunk.header, transactions: .shards[0].chunk.transactions, receipts: .shards[0].chunk.receipts, receipt_execution_outcomes: .shards[0].receipt_execution_outcomes, state_changes: .state_changes}'

    else
        echo
        echo "Canceled"
    fi

fi

