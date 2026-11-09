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
"${SCRIPT_DIR}/cargo-local.sh" test -p docsis-ofdm-fec --example prepare_rxmer_variants --locked
"${SCRIPT_DIR}/cargo-local.sh" test -p docsis-ofdm-fec --no-default-features --locked
"${SCRIPT_DIR}/cargo-local.sh" build -p ofdm-fec-cli --release --locked
SMOKE_DIRECTORY="$(mktemp -d)"
trap 'rm -rf -- "${SMOKE_DIRECTORY}"' EXIT
"${SCRIPT_DIR}/cargo-local.sh" run -p ofdm-fec-cli --release --locked --quiet -- --version
"${SCRIPT_DIR}/cargo-local.sh" run -p ofdm-fec-cli --release --locked --quiet -- simulate \
  --input examples/awgn.json --output "${SMOKE_DIRECTORY}/awgn.json"
python3 -m json.tool "${SMOKE_DIRECTORY}/awgn.json" >/dev/null
"${SCRIPT_DIR}/cargo-local.sh" run -p ofdm-fec-cli --release --locked --quiet -- channel \
  --input examples/channel-probes.json --output "${SMOKE_DIRECTORY}/channel.json" \
  --probes-output "${SMOKE_DIRECTORY}/channel-probes.jsonl"
python3 - "${SMOKE_DIRECTORY}/channel.json" "${SMOKE_DIRECTORY}/channel-probes.jsonl" <<'PY_CHANNEL'
import json
import sys
with open(sys.argv[1], encoding="utf-8") as source:
    result = json.load(source)
assert result["status"] == "complete"
assert result["summary"]["comparison"]["bit_errors"] == 0
assert result["summary"]["window"]["payload_rate_bps"] is not None
with open(sys.argv[2], encoding="utf-8") as source:
    records = [json.loads(line) for line in source]
assert records and all(record["schema_version"] == 1 for record in records)
PY_CHANNEL
"${SCRIPT_DIR}/cargo-local.sh" run -p ofdm-fec-cli --release --locked --quiet -- import-pnm \
  --input examples/measurements/single-channel/import.json --output "${SMOKE_DIRECTORY}/scenario.json"
"${SCRIPT_DIR}/cargo-local.sh" run -p ofdm-fec-cli --release --locked --quiet -- channel \
  --input examples/measurements/single-channel/channel-constellation.json \
  --scenario "${SMOKE_DIRECTORY}/scenario.json" --output "${SMOKE_DIRECTORY}/measured.json"
"${SCRIPT_DIR}/cargo-local.sh" run -p ofdm-fec-cli --release --locked --quiet -- channel \
  --input examples/channel-awgn.json --output "${SMOKE_DIRECTORY}/waveform-awgn.json"
python3 - "${SMOKE_DIRECTORY}/measured.json" "${SMOKE_DIRECTORY}/waveform-awgn.json" <<'PY_IMPAIRMENTS'
import json
import sys
for path in sys.argv[1:]:
    with open(path, encoding="utf-8") as source:
        result = json.load(source)
    assert result["status"] == "complete"
    assert result["summary"]["comparison"]["bit_errors"] == 0
    assert result["summary"]["measured_mer_db"] is not None
    assert result["summary"]["payload_mean_ldpc_iterations"] is not None
    if "constellation" in result:
        assert result["constellation"]["status"] == "complete"
        assert result["constellation"]["collected_pairs"] == 8192
        assert len(result["constellation"]["pairs"]) == 8192
PY_IMPAIRMENTS
"${SCRIPT_DIR}/cargo-local.sh" run -p ofdm-fec-cli --release --locked --quiet -- channel \
  --input examples/channel-file.json --payload-file examples/payloads/message.txt \
  --output "${SMOKE_DIRECTORY}/file-result.json" \
  --recovered-file "${SMOKE_DIRECTORY}/recovered-message.txt"
cmp examples/payloads/message.txt "${SMOKE_DIRECTORY}/recovered-message.txt"
python3 tools/release/versions.py
echo 'All local pre-push checks passed.'
