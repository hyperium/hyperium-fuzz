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

# disable any log output during fuzzing
export RUST_LOG=

set -x -u -o pipefail

if [[ -n "$FUZZ_SYNC_TARGET" ]]; then
    echo "[+] synching corpus"
    rsync -rtu "$FUZZ_SYNC_TARGET/corpus/" ./corpus || true
    if [[ -d ./corpus/corpus ]]; then
        echo "...rsync syntax is wrong"
        exit 1 
    fi
fi

echo "[+] corpus across all harnesses: $(ls -lR corpus | wc -l)"

for target in $(cargo fuzz list | shuf); do 
    echo "[+] running target: $target"
    cargo fuzz cmin -O -s none "$target" -- -set_cover_merge=1 -rss_limit_mb=$MERGE_RSS

    if [[ "$TEST" -eq 1 ]]; then
        break
    fi
done

echo "[+] minimized corpus across all harnesses: $(ls -lR corpus | wc -l)"

if [[ -n "$FUZZ_SYNC_TARGET" ]]; then
    echo "[+] overwriting corpus with minimized local corpus"
    rsync --delete -rtu ./corpus/ "$FUZZ_SYNC_TARGET/corpus" || true
    if [[ -d "$FUZZ_SYNC_TARGET/corpus/corpus" ]]; then
        echo "...rsync syntax is wrong"
        exit 1 
    fi
fi
