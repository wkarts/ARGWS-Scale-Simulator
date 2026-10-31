#!/usr/bin/env bash
set -euo pipefail
APP_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
set -a
[[ -f "$APP_DIR/.env" ]] && source "$APP_DIR/.env"
set +a
BIN_PATH="${SCALE_SIMULATOR_BINARY:-$APP_DIR/bin/scale_simulator_tauri}"
if [[ "$BIN_PATH" != /* ]]; then
  BIN_PATH="$APP_DIR/${BIN_PATH#./}"
fi
export SCALE_SIMULATOR_ENV_FILE="${SCALE_SIMULATOR_ENV_FILE:-$APP_DIR/.env}"
export SCALE_SIMULATOR_DATA_DIR="${SCALE_SIMULATOR_DATA_DIR:-$APP_DIR/data}"
"$BIN_PATH" --mode=worker --data-dir "$SCALE_SIMULATOR_DATA_DIR" "$@"
