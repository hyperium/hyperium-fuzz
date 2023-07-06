#!/bin/bash

rm -rf coverage
cargo fuzz coverage -O fuzz_httparse_request
./grcov.sh

pushd ../../cov.html/
mv hyper httparse_req_v1
popd

rm -rf coverage
cargo fuzz coverage -O fuzz_httparse_request2
./grcov.sh

pushd ../../cov.html/
mv hyper httparse_req_v2
popd
