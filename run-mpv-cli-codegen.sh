#!/usr/bin/env bash

set -e

cd ./crates/mpv-cli-codegen
echo "use super::*;" > ../mpv-cli/src/protocol/message/low_level/property/codegen.rs
cargo run --quiet | rustfmt >> ../mpv-cli/src/protocol/message/low_level/property/codegen.rs
cargo fmt
