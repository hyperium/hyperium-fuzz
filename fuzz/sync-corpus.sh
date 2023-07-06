#!/usr/bin/env bash

set -ex

if [[ -z "$FUZZ_SYNC_TARGET" ]]; then
    exit 0
fi


set -u

for what in artifacts corpus; do
    rsync -rtu "$FUZZ_SYNC_TARGET/$what/" "./$what" || true
    rsync -rtu "./$what/" "$FUZZ_SYNC_TARGET/$what"
done
