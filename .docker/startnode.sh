#!/bin/sh

network="testnet"
namePostfix="test"

if $(is-mainnet); then
	network="mainnet"
	namePostfix="near"
fi

if [ ! -d /near/${network} ]; then
	mkdir /near/${network}
	/usr/local/bin/borealis-indexer --home-dir /near/${network} init --chain-id ${network} --boot-nodes "" --download-genesis --download-genesis-url "" --download-config --download-config-url "https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/${network}/config.json"
fi

/usr/local/bin/borealis-indexer --home-dir /near/testnet run --creds-path "/near/testnet/nats.creds" --nats-server nats://westcoast.nats.backend.aurora.dev:4222 --subject "BlockIndex_StreamerMessages" --msg-format JSON
