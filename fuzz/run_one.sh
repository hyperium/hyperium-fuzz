#!/usr/bin/env bash

if [[ -z "$USE_CARGO_LIBAFL" ]]; then
    USE_CARGO_LIBAFL=0
fi
if [[ -z "$CORPUS_TIME" ]]; then
    CORPUS_TIME=$(python -c 'print(60 * 60 * 2)')
fi
if [[ -z "$SANIT_TIME" ]]; then
    SANIT_TIME=$(python -c 'print(60 * 30)')
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

# disable any log output during fuzzing
export RUST_LOG=

set -x -u -o pipefail

function sync_corpus_for {
    if [[ -n "$FUZZ_SYNC_TARGET" ]]; then
        echo "[+] synching corpus for $1"
        for path in "corpus/$1" "artifacts/$1"; do 
            rsync -rtu "$FUZZ_SYNC_TARGET/$path/" "./$path" || true
            rsync -rtu "./$path/" "$FUZZ_SYNC_TARGET/$path" || true
        done
    fi
}

TARGET="$1"

echo "[+] fuzzing TARGET: $TARGET ($CORPUS_TIME sec fuzzing for corpus expansion; $SANIT_TIME sec fuzzing with sanitizers)"
ARTIFACTS="$(realpath "$PWD/artifacts/$TARGET/")"

sync_corpus_for "$TARGET"

cargo fuzz cmin -O -s none "$TARGET" -- -set_cover_merge=1 -rss_limit_mb=$MERGE_RSS 

if [[ "$TARGET" = "fuzz_h2_server2" ]] && [[ -e "../afl-h2server/afl_out/" ]]; then
    mkdir -p ./corpus/$TARGET.new
    "./TARGET/x86_64-unknown-linux-gnu/release/$TARGET" \
        -artifact_prefix=`realpath -m ./artifacts/$TARGET/` \
        -set_cover_merge=1 -rss_limit_mb=$MERGE_RSS \
        ./corpus/$TARGET.new ./corpus/$TARGET ../afl-h2server/afl_out/*/queue/
    rm -rf ./corpus/$TARGET.old
    mv ./corpus/$TARGET ./corpus/$TARGET.old
    mv ./corpus/$TARGET.new ./corpus/$TARGET
fi

if [[ "$CORPUS_TIME" -ne 0 ]]; then
    # first run without sanitizer/debug assertions for greater speed to build
    # the corpus.
    cargo fuzz run -O -s none "$TARGET" -- -fork=$CPUS -max_total_time=$CORPUS_TIME -use_value_profile=1 -shrink=1 -dict=$(realpath ./http2.dict)

    # run the harness also with cargo libafl
    if [[ "$USE_CARGO_LIBAFL" -eq 1 ]]; then
        cargo clean
        mkdir -p "$ARTIFACTS/libafl/"
        # cargo-libafl has not built-in mechanism for max_total_time
        timeout -k 120 "$(($CORPUS_TIME + 120))" cargo libafl run \
            --no-default-features --features use_libafl \
            -O -a -s address "$TARGET" -- \
            -s -g --cores "0-$(($CPUS - 1))" -x $(realpath ./http2.dict) \
                || true
        sleep 1 
        pkill "$TARGET" || true
        sleep 10
        pkill -9 "$TARGET" || true
    fi

    # minimize the corpus before running with sanitizers
    cargo fuzz cmin -O -s none "$TARGET" -- -set_cover_merge=1 -rss_limit_mb=$MERGE_RSS
fi

# fuzz with sanitizers
SANIT_ARGS="-fork=$CPUS -max_total_time=$SANIT_TIME -use_value_profile=1 -dict=$(realpath ./http2.dict)"

mkdir -p "$ARTIFACTS/"{address,thread,memory}

if [[ "$SANIT_TIME" -ne 0 ]]; then
    # asan for OOB and memory errors
    cargo fuzz run -O -a --careful -s address "$TARGET" -- $SANIT_ARGS -artifact_prefix=$ARTIFACTS/address/
    # thread sanitizer for data race detection
    cargo fuzz run -O -a -s thread "$TARGET" -- $SANIT_ARGS -artifact_prefix=$ARTIFACTS/thread/
    # memory sanitizer for uninit reads
    cargo fuzz run -O -a --careful -s memory "$TARGET" -- $SANIT_ARGS -artifact_prefix=$ARTIFACTS/memory/

    # again minimize the corpus; get rid of testcases that only exercise new
    # coverage due to instrumentation.
    cargo fuzz cmin -O -s none "$TARGET" -- -set_cover_merge=1 -rss_limit_mb=$MERGE_RSS
fi

# get coverage
cargo fuzz coverage -O "$TARGET"

sync_corpus_for "$TARGET"
