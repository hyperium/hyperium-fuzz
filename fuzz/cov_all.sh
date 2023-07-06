#!/usr/bin/env bash

if [[ -z "$MERGE_RSS" ]]; then
    MERGE_RSS=122880
fi
if [[ -z "$FUZZ_SYNC_TARGET" ]]; then
    FUZZ_SYNC_TARGET=""
fi

if [[ "$TEST" -eq 1 ]]; then
    set -e
else
    TEST=0
fi 

# prevent logging output
export RUST_LOG=

set -x -u -o pipefail

if [[ -n "$FUZZ_SYNC_TARGET" ]]; then
    echo "[+] synching corpus"
    rsync -rtu "$FUZZ_SYNC_TARGET/corpus/" ./corpus || true
fi

# cargo fuzz build -O

for target in $(cargo fuzz list | shuf); do 
    echo "[+] running target: $target"
    cargo fuzz cmin -O -s none "$target" -- -set_cover_merge=1 -rss_limit_mb=$MERGE_RSS
    cargo fuzz coverage -O "$target"

    if [[ "$TEST" -eq 1 ]]; then
        break
    fi
done

echo "[+] making coverage report"
./grcov.sh || true
