#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

mkdir -v -p ./data/ ;

./s5cmd -r 100 --numworkers 256 --stat --log "debug" --no-sign-request cp --flatten --if-source-newer "s3://near-protocol-public/backups/testnet/rpc/latest" ./data/ ;
# ./s5cmd -r 100 --numworkers 256 --stat --log "debug" --no-sign-request ls --humanize "s3://near-protocol-public/backups/testnet/rpc/$(cat ./data/latest)/*" ;
# ./s5cmd -r 100 --numworkers 256 --stat --log "debug" --no-sign-request cp --flatten --if-source-newer "s3://near-protocol-public/backups/testnet/rpc/$(cat ./data/latest)/*" ./data/ ;
./s5cmd -r 100 --numworkers 256 --stat --log "debug" --no-sign-request cp --flatten --if-size-differ "s3://near-protocol-public/backups/testnet/rpc/$(cat ./data/latest)/*" ./data/ ;

# tar -vx -f ./data.tar -C ./data/ ;
# find ./data/ -type f -exec chmod -v a-x '{}' \+ ;
