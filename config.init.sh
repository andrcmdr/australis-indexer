#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

read -n 1 -s -p "Proceed with indexer initial configuration? [press any key to continue] : " choice

./target/debug/aurora-indexer init
# ./target/release/aurora-indexer init
