#!/bin/sh

network="testnet"
namePostfix="test"

if $(is-mainnet); then
	network="mainnet"
	namePostfix="near"
fi

if [ ! -d /near/${network} ]; then
	mkdir /near/${network}
fi

if [ ! -f /near/${network}/config.json ]; then
	curl -o /near/${network}/config.json https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json
fi

if [ ! -f /near/${network}/genesis.json ]; then
	curl -o /near/${network}/genesis.json https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/genesis.json
fi

if [ ! -f /near/${network}/node_key.json ]; then
	/usr/local/bin/nearkey "node%.${namePostfix}" > /near/${network}/node_key.json
fi

if [ ! -f /near/${network}/validator_key.json ]; then
	/usr/local/bin/nearkey "validator%.${namePostfix}" > /near/${network}/validator_key.json
fi

if [ ! -f /near/server ]; then
	echo "nats://westcoast.nats.backend.aurora.dev:4222" > /near/server
fi
if [ ! -f /near/subject ]; then
	echo "BlockIndex_StreamerMessages" > /near/subject
fi

/usr/local/bin/borealis-indexer --home-dir /near/${network} run --creds-path "/near/nats.creds" --nats-server "$(cat /near/server)" --subject "$(cat /near/subject)" --msg-format CBOR
