#!/bin/sh
/usr/local/bin/borealis-indexer --home-dir /home/near run --root-cert-path /home/near/ca.crt --creds-path /home/near/nats.creds --nats-server nats://genesis.nats.backend.aurora.dev:4222 --msg-format CBOR  --subject BlockIndex_StreamerMessages_testnet_CBOR 2>&1 | tee -a /home/near/indexer.log
