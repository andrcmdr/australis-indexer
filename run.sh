#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

read -n 1 -s -p "Proceed with indexer run? [press any key to continue] : " choice

./target/debug/aurora-indexer run
# ./target/release/aurora-indexer run
# | jq '{block_height: .block.header.height, block_hash: .block.header.hash, chunks: .block.chunks, shard_chunk_header: .shards[0].chunk.header, transactions: .shards[0].chunk.transactions, receipts: .shards[0].chunk.receipts, receipt_execution_outcomes: .shards[0].receipt_execution_outcomes, state_changes: .state_changes}'
