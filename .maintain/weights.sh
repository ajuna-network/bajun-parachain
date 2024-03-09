#!/bin/bash
set -e

RUNTIME_WEIGHT_DIR=runtime/bajun/src/weights
COLLATOR=./target/release/bajun-node
CHAIN=local

mkdir -p $RUNTIME_WEIGHT_DIR

$COLLATOR benchmark pallet \
    --chain ${CHAIN} \
    --list |\
  tail -n+2 |\
  cut -d',' -f1 |\
  uniq > "bajun_runtime_pallets"

# For each pallet found in the previous command, run benches on each function
while read -r line; do
  pallet="$(echo "$line" | cut -d' ' -f1)";
  echo benchmarking "$pallet"...

  $COLLATOR \
  benchmark pallet \
  --chain=${CHAIN} \
  --steps=50 \
  --repeat=20 \
  --pallet="$pallet" \
  --extrinsic="*" \
  --wasm-execution=compiled \
  --heap-pages=4096 \
  --output=./$RUNTIME_WEIGHT_DIR/"$pallet".rs
done < "bajun_runtime_pallets"
rm "bajun_runtime_pallets"