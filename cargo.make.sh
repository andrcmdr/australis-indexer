#!/bin/bash
#/usr/bin/env bash

shopt -s extglob
shopt -s extquote
# shopt -s xpg_echo

set -f

# --check works since cargo-fmt 1.4.38
cargo fmt -v --all --check ;

read -n 1 -s -p "Proceed with cargo fmt/check/build? [press any key to continue] : " choice
echo -e "\n"

cargo fmt -v --all ;

cargo check ;

cargo clippy ;

cargo build
# cargo build --release

