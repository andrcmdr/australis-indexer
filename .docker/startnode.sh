#!/bin/sh

network="testnet"

if $(is-mainnet); then
    network="mainnet"
fi

if [ ! -d /borealis-indexer/${network} ]; then
    mkdir /borealis-indexer/${network}
    /usr/local/bin/borealis-indexer --home-dir /borealis-indexer/${network} init --chain-id ${network} --download-genesis --download-config
fi

/usr/local/bin/borealis-indexer --home-dir /borealis-indexer/${network} run --root-cert-path /borealis-indexer/${network}/root-ca.crt --creds-path /borealis-indexer/${network}/nats.creds --nats-server "tls://westcoast.nats.backend.aurora.dev:4222,tls://eastcoast.nats.backend.aurora.dev:4222" --subject "BlockIndex_StreamerMessages_${network}" --msg-format "CBOR"
