#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

echo -e "\n";

# time /bin/time -v env RUST_BACKTRACE=full ./borealis-indexer --home-dir ./.borealis-indexer/testnet/ check --root-cert-path ./.nats/seed/root-ca.crt --creds-path ./.nats/seed/nats.creds --nats-server "tls://eastcoast.nats.backend.aurora.dev:4222,tls://westcoast.nats.backend.aurora.dev:4222" --subject "BlockIndex_StreamerMessages_testnet" --msg-format "CBOR" --sync-mode "LatestSynced" --await-synced "WaitForFullSync" >> ./log/borealis-indexer.debug.log 2>&1 & disown;
time /bin/time -v env RUST_BACKTRACE=full ./borealis-indexer --home-dir ./.borealis-indexer/testnet/ run --root-cert-path ./.nats/seed/root-ca.crt --creds-path ./.nats/seed/nats.creds --nats-server "tls://eastcoast.nats.backend.aurora.dev:4222,tls://westcoast.nats.backend.aurora.dev:4222" --subject "BlockIndex_StreamerMessages_testnet" --msg-format "CBOR" --sync-mode "LatestSynced" --await-synced "WaitForFullSync" >> ./log/borealis-indexer.debug.log 2>&1 & disown;

echo -e "\n";

