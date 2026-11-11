#!/usr/bin/env bash
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Run local Rust tests across binaries with bounded process concurrency.
# Arguments: all (default), workspace, analysis, example, list; nextest options.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
ACTION="${1:-all}"
if [[ $# -gt 0 ]]; then shift; fi
case "$ACTION" in
  --help|-h)
    echo 'Usage: scripts/test-local.sh [all|workspace|analysis|example|list] [nextest options]'
    echo 'Default: 4 test processes. Set MERLIN_TEST_THREADS to select 1..32.'
    echo 'First use downloads a checksum-pinned runner into local/test-tools/.'
    exit 0 ;;
  all|workspace|analysis|example|list) ;;
  *) echo "Unknown local test phase: $ACTION" >&2; exit 2 ;;
esac
WORKERS="${MERLIN_TEST_THREADS:-4}"
if [[ ! "$WORKERS" =~ ^([1-9]|[12][0-9]|3[0-2])$ ]]; then
  echo 'MERLIN_TEST_THREADS must be an integer from 1 to 32.' >&2
  exit 2
fi
cd "$PROJECT_DIR"
EXECUTABLE="$(python3 tools/testing/nextest.py)"
PATH="$(dirname -- "$EXECUTABLE"):${PATH}"
export PATH
CARGO=("${SCRIPT_DIR}/cargo-local.sh")
echo "Local tests: nextest 0.9.151, ${WORKERS} concurrent test processes; phase ${ACTION}." >&2
if [[ "$ACTION" == list ]]; then
  exec "${CARGO[@]}" nextest list --workspace --locked "$@"
fi
if [[ "$ACTION" == all || "$ACTION" == workspace ]]; then
  "${CARGO[@]}" nextest run --profile local --workspace --locked --test-threads "$WORKERS" "$@"
  "${CARGO[@]}" test --workspace --doc --locked
fi
if [[ "$ACTION" == all || "$ACTION" == example ]]; then
  "${CARGO[@]}" nextest run --profile local -p docsis-ofdm-fec --example prepare_rxmer_variants --locked --test-threads "$WORKERS" "$@"
fi
if [[ "$ACTION" == all || "$ACTION" == analysis ]]; then
  "${CARGO[@]}" nextest run --profile local -p docsis-ofdm-fec --no-default-features --locked --test-threads "$WORKERS" "$@"
  "${CARGO[@]}" test -p docsis-ofdm-fec --no-default-features --doc --locked
fi
