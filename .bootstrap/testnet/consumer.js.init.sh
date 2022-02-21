#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

echo -e "\n";

# time /bin/time -v env RUST_BACKTRACE=1 ./borealis-consumer init --root-cert-path ./.nats/seed/root-ca.crt --creds-path ./.nats/seed/nats.creds --nats-server "tls://eastcoast.nats.backend.aurora.dev:4222,tls://westcoast.nats.backend.aurora.dev:4222" --work-mode "jetstream" --subject "BlockIndex_StreamerMessages_testnet" --msg-format "CBOR" >> ./log/borealis-consumer.debug.log 2>&1 & disown;
# time /bin/time -v env RUST_BACKTRACE=1 ./borealis-consumer init --root-cert-path ./.nats/seed/root-ca.crt --creds-path ./.nats/seed/nats.creds --nats-server "tls://eastcoast.nats.backend.aurora.dev:4222,tls://westcoast.nats.backend.aurora.dev:4222" --work-mode "jetstream" --subject "BlockIndex_StreamerMessages_testnet" --msg-format "CBOR" 2>&1 | tee -a ./log/borealis-consumer.debug.log & disown;
time /bin/time -v env RUST_BACKTRACE=1 ./borealis-consumer init --root-cert-path ./.nats/seed/root-ca.crt --creds-path ./.nats/seed/nats.creds --nats-server "tls://eastcoast.nats.backend.aurora.dev:4222,tls://westcoast.nats.backend.aurora.dev:4222" --work-mode "jetstream" --subject "BlockIndex_StreamerMessages_testnet" --msg-format "CBOR"

echo -e "\n";

