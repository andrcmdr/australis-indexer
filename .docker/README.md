# Prebuild

Testnet and mainnet prebuild images are available:

  - nearaurora/nearindex:testnet
  - nearaurora/nearindex:mainnet

Then run:

  `$ docker run --rm -v /abusolutepath/near:/near nearaurora/nearindex:testnet`
  
or

  `$ docker run --rm -v /abusolutepath/near:/near nearaurora/nearindex:mainnet`

The directory /absolutepath/near MUST contain two files to start:

  - "server": Which must contain the URL of the NATS server to connect to.
  - "nats.creds": Which must be the NATS credentials file to use.

# Building

Make sure to put borealis-indexer-main.zip (boerealis-indexer source) into current directory before docker build.

For mainnet:

  `$ docker build -t nearaurora/nearindex:mainnet --build-arg NEARNETWORK=mainnet -f Dockerfile .`

For testnet:

  `$ docker build -t nearaurora/nearindex:mainnet --build-arg NEARNETWORK=mainnet -f Dockerfile .`


