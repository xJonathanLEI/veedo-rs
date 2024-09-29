#!/usr/bin/env bash

# Build benchmark wasm artifacts with `build_bench_wasm.sh` first

set -e

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_ROOT=$( dirname -- $SCRIPT_DIR )

RUNTIME="$1"

if [ -z "$RUNTIME" ]; then
  echo "Runtime not specified"
  exit 1
fi

benches=(
  compute_100k_iterations
  inverse_100k_iterations
)

for bench in ${benches[@]}; do
  if [[ "$RUNTIME" == "wasmtime" ]]; then
    # https://github.com/bytecodealliance/wasmtime/issues/7384
    $RUNTIME run --dir=. -- $REPO_ROOT/target/bench-wasm/$bench.wasm --bench
  else
    $RUNTIME run --dir=. $REPO_ROOT/target/bench-wasm/$bench.wasm -- --bench
  fi
done
