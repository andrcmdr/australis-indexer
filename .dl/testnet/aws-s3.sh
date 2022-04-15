#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

mkdir -v -p ./aws-s3/ ;

./s5cmd -r 100 --numworkers 256 --no-sign-request cp --flatten --if-source-newer "s3://near-protocol-public/backups/testnet/rpc/latest" ./aws-s3/ ;
./s5cmd -r 100 --numworkers 256 --no-sign-request ls "s3://near-protocol-public/backups/testnet/rpc/$(cat ./aws-s3/latest)/*" > ./aws-s3.ls ;

sed -r -s -e "s/^([0-9]+\/[0-9]+\/[0-9]+\s+[0-9]+\:[0-9]+\:[0-9]+\s+[0-9]+\s+)/https\:\/\/near\-protocol\-public\.s3\.ca\-central\-1\.amazonaws\.com\/backups\/testnet\/rpc\/$(cat ./aws-s3/latest)\//gI" ./aws-s3.ls > ./aws-s3.list ;

wget --verbose --report-speed=bits --append-output=wget-aws-s3.log --trust-server-names --content-disposition --tries=100 --continue --progress=bar --show-progress --timestamping --server-response --dns-timeout=60 --connect-timeout=60 --read-timeout=60 --waitretry=60 --prefer-family=IPv4 --retry-connrefused --user-agent='Mozilla/5.0 (X11; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0' --referer= --recursive --level=30 --no-parent --no-directories --no-host-directories --directory-prefix=./data/ --input-file='./aws-s3.list' "${@}" ;

