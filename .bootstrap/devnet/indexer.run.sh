#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

echo -e "\n";

time /bin/time -v env RUST_BACKTRACE=full ./borealis-indexer --home-dir ./.borealis-indexer/devnet/ run --root-cert-path ./.nats/seed/root-ca.crt --creds-path ./.nats/seed/nats.creds --nats-server "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222" --subject "BlockIndex_StreamerMessages_devnet" --msg-format "CBOR" >> ./log/borealis-indexer.debug.log 2>&1 & disown;

echo -e "\n";

