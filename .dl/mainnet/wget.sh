#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

wget --verbose --report-speed=bits --append-output=wget.log --trust-server-names --content-disposition --tries=30 --continue --progress=bar --show-progress --timestamping --server-response --dns-timeout=60 --connect-timeout=60 --read-timeout=60 --waitretry=60 --prefer-family=IPv4 --retry-connrefused --user-agent='Mozilla/5.0 (X11; Linux x86_64; rv:95.0) Gecko/20100101 Firefox/95.0' --referer= --recursive --level=30 --no-parent --no-directories --no-host-directories --directory-prefix=./ --input-file='./download.list' "${@}" ;

mkdir -v -p ./data/ ;
tar -vx -f ./data.tar -C ./data/ ;
find ./data/ -type f -exec chmod -v a-x '{}' \+ ;

