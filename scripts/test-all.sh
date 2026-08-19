#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
count=0

for lab in "$root"/labs/rust-*-debug-lab; do
  if [[ ! -d "$lab" ]]; then
    continue
  fi
  printf '=== %s ===\n' "$(basename "$lab")"
  (
    cd "$lab"
    cargo test
  )
  count=$((count + 1))
done

printf 'Verified %d Rust debug labs.\n' "$count"
