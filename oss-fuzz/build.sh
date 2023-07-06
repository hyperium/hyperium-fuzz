#!/bin/bash -eu
# Copyright 2021 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
################################################################################

cd "$SRC/fuzz-hyperium/fuzz/"
cargo fuzz build -O -a

cp *.dict "$OUT"
for fuzzer in $(cargo fuzz list); do
  cp "./fuzz/target/x86_64-unknown-linux-gnu/release/$fuzzer" "$OUT/"
	
  echo "[libfuzzer]" > "$OUT/${fuzzer}.options"
  if [[ "$fuzzer" == "fuzz_h2_client" ]] || [[ "$fuzzer" == "fuzz_h2_e2e" ]]; then
      echo "detect_leaks=0" >> "$OUT/${fuzzer}.options"
  fi
  case "$fuzzer" in
      *h2*)
          echo "dict=http2.dict" >> "$OUT/${fuzzer}.options"
          ;;
      *)
          echo "dict=http.dict" >> "$OUT/${fuzzer}.options"
          ;;
  esac
done
