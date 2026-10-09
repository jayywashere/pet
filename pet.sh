#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
EXECUTABLE="$SCRIPT_DIR/target/release/pet"

if [[ ! -x "$EXECUTABLE" ]]; then
    printf 'Error: pet executable not found at %s\n' "$EXECUTABLE" >&2
    printf 'Build it first with: cargo build --release\n' >&2
    exit 1
fi

exec "$EXECUTABLE" "$@"