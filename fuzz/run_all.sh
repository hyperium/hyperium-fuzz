#!/usr/bin/env bash

if [[ -z "$USE_CARGO_LIBAFL" ]]; then
    USE_CARGO_LIBAFL=1
fi
if [[ -z "$CORPUS_TIME" ]]; then
    CORPUS_TIME=$(python -c 'print(int(60 * 60 * 4))')
fi
if [[ -z "$SANIT_TIME" ]]; then
    SANIT_TIME=$(python -c 'print(int(60 * 60 * 1))')
fi
if [[ -z "$CPUS" ]]; then
    CPUS="$(nproc)"
fi
if [[ -z "$MERGE_RSS" ]]; then
    MERGE_RSS=122880
fi
if [[ -z "$FUZZ_SYNC_TARGET" ]]; then
    FUZZ_SYNC_TARGET=""
fi

if [[ "$TEST" -eq 1 ]]; then
    CORPUS_TIME=15
    SANIT_TIME=5
    set -e
else
    TEST=0
fi 

export USE_CARGO_LIBAFL
export CORPUS_TIME
export SANIT_TIME
export CPUS
export MERGE_RSS
export FUZZ_SYNC_TARGET
export TEST

# disable any log output during fuzzing
export RUST_LOG=

set -x -u -o pipefail

for target in $(cargo fuzz list | shuf); do 
    echo "[+] fuzzing target: $target"
    artifacts="$(realpath -m "$PWD/artifacts/$target/")"
    corpus="$(realpath -m "$PWD/corpus/$target/")"
    mkdir -p "$artifacts"
    mkdir -p "$corpus"

    echo "[+] initial corpus count for $target: $(ls "$corpus" | wc -l)"
    echo "[+] initial artifacts count for $target: $(find "$artifacts" -type f | wc -l)"
    if ./run_one.sh "$target"; then
        echo "[+] fuzzing $target done"
    else
        echo "[FAIL] fuzzing $target failed"
    fi
    echo "[+] corpus count after fuzzing for $target: $(ls "$corpus" | wc -l)"
    artifcats_counts="$(find "$artifacts" -type f | wc -l)"
    echo "[+] artifacts count after fuzzing for $target: $artifcats_counts"

    if [[ "$artifcats_counts" -gt 0 ]]; then
        printf "found artifacts for $target\n\n"
        ls -R "$artifacts"
        printf "\n\n\n"
    fi
    
    if [[ "$TEST" -eq 1 ]]; then
        break
    fi
done

echo "[+] done with fuzzing - making coverage report"
./grcov.sh
