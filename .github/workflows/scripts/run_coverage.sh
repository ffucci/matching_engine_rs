#!/bin/bash
set -xeu

rustup component add llvm-tools-preview
RUSTFLAGS="-Cinstrument-coverage" cargo test --all-targets
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/ --excl-start "#\[cfg\(test\)\]" --keep-only **/src/**/*
Footer