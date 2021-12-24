#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

echo -e "\n";

mkdir -v -p ./log/

echo -e "\n";

time /bin/time -v env RUST_BACKTRACE=1 ./borealis-indexer --home-dir ./.borealis-indexer/mainnet/ init --chain-id mainnet --download-genesis --download-config >> ./log/borealis-indexer.debug.log 2>&1 \
&& sed -r -s -i"" "s/^(\s*?)\"tracked\_shards\"\:\s\[\]\,/\1\"tracked\_shards\"\:\ \[0\]\,/gI" "./.borealis-indexer/mainnet/config.json" \
& disown;

echo -e "\n";

time /bin/time -v env RUST_BACKTRACE=1 ./borealis-indexer --home-dir ./.borealis-indexer/testnet/ init --chain-id testnet --download-genesis --download-config >> ./log/borealis-indexer.debug.log 2>&1 \
&& sed -r -s -i"" "s/^(\s*?)\"tracked\_shards\"\:\s\[\]\,/\1\"tracked\_shards\"\:\ \[0\]\,/gI" "./.borealis-indexer/testnet/config.json" \
& disown;

echo -e "\n";

time /bin/time -v env RUST_BACKTRACE=1 ./borealis-indexer --home-dir ./.borealis-indexer/betanet/ init --chain-id betanet --download-genesis --download-config >> ./log/borealis-indexer.debug.log 2>&1 \
&& sed -r -s -i"" "s/^(\s*?)\"tracked\_shards\"\:\s\[\]\,/\1\"tracked\_shards\"\:\ \[0\]\,/gI" "./.borealis-indexer/betanet/config.json" \
& disown;

echo -e "\n";

time /bin/time -v env RUST_BACKTRACE=1 ./borealis-indexer --home-dir ./.borealis-indexer/devnet/ init --chain-id devnet >> ./log/borealis-indexer.debug.log 2>&1 \
&& sed -r -s -i"" "s/^(\s*?)\"tracked\_shards\"\:\s\[\]\,/\1\"tracked\_shards\"\:\ \[0\]\,/gI" "./.borealis-indexer/devnet/config.json" \
& disown;

echo -e "\n";

time /bin/time -v env RUST_BACKTRACE=1 ./borealis-indexer --home-dir ./.borealis-indexer/localnet/ init --chain-id localnet >> ./log/borealis-indexer.debug.log 2>&1 \
&& sed -r -s -i"" "s/^(\s*?)\"tracked\_shards\"\:\s\[\]\,/\1\"tracked\_shards\"\:\ \[0\]\,/gI" "./.borealis-indexer/localnet/config.json" \
& disown;

echo -e "\n";

