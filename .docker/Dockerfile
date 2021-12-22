# syntax=docker/dockerfile-upstream:experimental

FROM ubuntu:latest as build

ARG NEARVERSION="master"
ARG NEARNETWORK="mainnet"
ENV RUST_TOOLCHAIN="1.57" \
    DEBIAN_FRONTEND="noninteractive" \
    TZ="UTC" \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    CARGO_TARGET_DIR=/tmp/target \
    RUSTC_FLAGS='-C target-cpu=x86-64' \
    PORTABLE=ON

RUN apt-get update -qq && apt-get install -y \
    git \
    cmake \
    g++ \
    pkg-config \
    libssl-dev \
    curl \
    llvm \
    clang \
    binutils-dev \
    libcurl4-openssl-dev \
    zlib1g-dev \
    libdw-dev \
    libiberty-dev \
    gcc \
    protobuf-compiler \
    unzip \
    golang \
    && rm -rf /var/lib/apt/lists/* \
    && curl https://sh.rustup.rs -sSf | \
    sh -s -- -y --no-modify-path --default-toolchain "${RUST_TOOLCHAIN}"

WORKDIR /borealis-indexer/

COPY borealis-indexer-main.zip borealis-indexer-main.zip
RUN unzip borealis-indexer-main.zip \
    && cd borealis-indexer-main/ \
    && cargo build

RUN mkdir /tmp/build \
    && mv /tmp/target/debug/borealis-indexer /tmp/build

ADD https://files.deploy.aurora.dev/contrib/nearkey/nearkey.go nearkey.go
COPY startnode.sh startnode.sh
RUN go build nearkey.go \
    && mv nearkey /tmp/build \
    && mv startnode.sh /tmp/build \
    && chmod a+x /tmp/build/startnode.sh \
    && cp /bin/false /tmp/build/is-testnet \
    && cp /bin/false /tmp/build/is-mainnet \
    && cp /bin/true /tmp/build/is-${NEARNETWORK}

# Post Compile Image
FROM ubuntu:latest

EXPOSE 3030 24567

ENV DEBIAN_FRONTEND="noninteractive" \
    TZ="UTC"

RUN apt-get update -qq && apt-get install -y \
    libssl-dev ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /usr/local/bin
    && mkdir -p /home/borealis-indexer/${NEARNETWORK}/

COPY --from=build /tmp/build/borealis-indexer /tmp/build/nearkey /tmp/build/is-testnet /tmp/build/is-mainnet /tmp/build/startnode.sh /usr/local/bin/
COPY --from=build /borealis-indexer-main/.nats/seed/root-ca.crt /borealis-indexer-main/.nats/seed/nats.creds /home/borealis-indexer/${NEARNETWORK}/

VOLUME /home/borealis-indexer/

ENTRYPOINT [ "/usr/local/bin/startnode.sh" ]

# docker build -t nearaurora/nearindex:testnet --build-arg NEARNETWORK=testnet -f Dockerfile .
# docker run --rm -v /absolutepath/near:/near nearaurora/nearindex:testnet
# Make sure to have /absolutepath/near/creds.nat set to the credentials file.
# Make sure that /abosultepath/near/server contains the server address to connect to.
