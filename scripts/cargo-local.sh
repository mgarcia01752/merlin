#!/usr/bin/env bash
#
# Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
# SPDX-License-Identifier: Apache-2.0
#
# Run the pinned toolchain without installing software or changing global settings.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
if [[ -x "${PROJECT_DIR}/local/cargo/bin/cargo" ]]; then
  export CARGO_HOME="${CARGO_HOME:-${PROJECT_DIR}/local/cargo}"
  export RUSTUP_HOME="${RUSTUP_HOME:-${PROJECT_DIR}/local/rustup}"
  export PATH="${CARGO_HOME}/bin:${PATH}"
fi
if ! command -v cargo >/dev/null 2>&1; then
  echo 'Rust is missing. See docs/development/build.md for local setup.' >&2
  exit 1
fi
cd "${PROJECT_DIR}"
exec cargo "$@"
