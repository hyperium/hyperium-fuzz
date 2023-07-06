#!/usr/bin/env bash

OUT="$(realpath -m "../../cov.html/hyper")"
SOURCE="$(realpath ../../)"

mkdir -p "$OUT"

exec grcov \
    -t html \
    -o "$OUT" \
    --branch \
    --llvm \
    --binary-path ./target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/ \
    --ignore '**/f0_http_generator.rs' \
    --ignore '**/fuzz-hyperium/src/*' \
    -s "$SOURCE" \
    ./coverage/*
