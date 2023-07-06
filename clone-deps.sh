#!/usr/bin/env bash


set -ex

if [[ -z "$DEPTH" ]]; then
    DEPTH_ARG=""
else
    DEPTH_ARG="--depth=$DEPTH"
fi

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd $SCRIPT_DIR/../

for repo in \
    https://github.com/hyperium/hyper \
    https://github.com/hyperium/http \
    https://github.com/hyperium/http-body \
    https://github.com/seanmonstar/httparse \
    https://github.com/hyperium/h2 \
    https://github.com/pyfisch/httpdate \
; do
    git clone $DEPTH_ARG "$repo" \
        || (cd "$(echo "$repo" | cut -d '/' -f 5 )" && git pull --rebase || true)
done
