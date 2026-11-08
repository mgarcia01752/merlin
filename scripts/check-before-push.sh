#!/usr/bin/env bash
#
# Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
# SPDX-License-Identifier: Apache-2.0
#
# Validate this exact checkout before a guarded push or release commit.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
cd "${PROJECT_DIR}"
python3 tools/release/versions.py
python3 -m unittest discover -s tools/release -p 'test_*.py'
python3 -m unittest discover -s tools/git/tests -p 'test_*.py'
SHELL_SCRIPTS=(scripts/cargo-local.sh scripts/check-before-push.sh tools/bump-version.sh tools/git/git-save.sh tools/git/git-push.sh)
for script in "${SHELL_SCRIPTS[@]}"; do
  bash -n "${script}"
done
command -v shellcheck >/dev/null
shellcheck "${SHELL_SCRIPTS[@]}"
"${SCRIPT_DIR}/cargo-local.sh" fmt --all -- --check
"${SCRIPT_DIR}/cargo-local.sh" clippy --workspace --all-targets --locked -- -D warnings
"${SCRIPT_DIR}/cargo-local.sh" test --workspace --locked
"${SCRIPT_DIR}/cargo-local.sh" test -p docsis-ofdm-fec --no-default-features --locked
"${SCRIPT_DIR}/cargo-local.sh" build -p ofdm-fec-cli --release --locked
SMOKE_DIRECTORY="$(mktemp -d)"
trap 'rm -rf -- "${SMOKE_DIRECTORY}"' EXIT
"${SCRIPT_DIR}/cargo-local.sh" run -p ofdm-fec-cli --release --locked --quiet -- --version
"${SCRIPT_DIR}/cargo-local.sh" run -p ofdm-fec-cli --release --locked --quiet -- simulate \
  --input examples/awgn.json --output "${SMOKE_DIRECTORY}/awgn.json"
python3 -m json.tool "${SMOKE_DIRECTORY}/awgn.json" >/dev/null
python3 tools/release/versions.py
echo 'All local pre-push checks passed.'
