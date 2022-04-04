# syntax=docker/dockerfile-upstream:experimental

FROM ubuntu:latest as build

ARG NEARVERSION="master"
ARG NEARNETWORK="testnet"
ENV RUST_TOOLCHAIN="1.58.1" \
    DEBIAN_FRONTEND="noninteractive" \
    TZ="UTC" \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    CARGO_TARGET_DIR=/tmp/target \
    RUSTC_FLAGS='-C target-cpu=x86-64' \
    PORTABLE=ON \
    CARGO_NET_GIT_FETCH_WITH_CLI=true

COPY ssh_config /root/.ssh/config
COPY id_ed25519 /root/.ssh/id_ed25519
COPY startnode.sh /startnode.sh

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
    golang \
    patch \
    && chmod a+x /startnode.sh \
    && mkdir /near \
    && mkdir /s5cmd \
    && rm -rf /var/lib/apt/lists/* \
    && curl https://sh.rustup.rs -sSf | \
    sh -s -- -y --no-modify-path --default-toolchain "${RUST_TOOLCHAIN}" \
    && rustup target add wasm32-unknown-unknown \
    && mkdir /indexer \
    && cd /indexer \
    && git clone git@github.com:aurora-is-near/borealis-indexer.git \
    && cd borealis-indexer \
    && git submodule update --recursive --init \
    && cargo build --release


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

COPY --from=build /startnode.sh /tmp/target/release/borealis-indexer /tmp/target/release/borealis-consumer /usr/local/bin/

VOLUME /home/near

ENTRYPOINT [ "/usr/local/bin/startnode.sh" ]

# docker build -t dockerreg.internal.aurora.dev/borealis-indexer:latest -f Dockerfile .
