#!/usr/bin/env bash
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Audit the locked dependency graph without installing host packages.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
AUDIT_VERSION="0.22.2"
AUDIT_SHA256="7fb9497f8594b389e5fce5ef9b92db08432996895b2e0c5a0167a69ed445c428"
AUDIT_DIRECTORY="${PROJECT_DIR}/local/security-tools/cargo-audit-${AUDIT_VERSION}-musl"
REPORT_DIRECTORY="${PROJECT_DIR}/local/security-audit"
if [[ "$(uname -s)" != Linux || "$(uname -m)" != x86_64 ]]; then
  echo "ERROR: This audit launcher requires Linux x64." >&2
  exit 2
fi
mkdir -p -- "${AUDIT_DIRECTORY}" "${REPORT_DIRECTORY}"
AUDIT_ARCHIVE="${AUDIT_DIRECTORY}/cargo-audit.tgz"
# Reverify cached downloads before every extraction and execution.
if [[ ! -f "${AUDIT_ARCHIVE}" ]]; then
  DOWNLOAD_FILE="$(mktemp "${AUDIT_DIRECTORY}/download.XXXXXX")"
  trap 'rm -f -- "${DOWNLOAD_FILE}"' EXIT
  curl --fail --location --retry 3 --output "${DOWNLOAD_FILE}" \
    "https://github.com/rustsec/rustsec/releases/download/cargo-audit/v${AUDIT_VERSION}/cargo-audit-x86_64-unknown-linux-musl-v${AUDIT_VERSION}.tgz"
  printf '%s  %s\n' "${AUDIT_SHA256}" "${DOWNLOAD_FILE}" | sha256sum --check --status
  mv -- "${DOWNLOAD_FILE}" "${AUDIT_ARCHIVE}"
fi
printf '%s  %s\n' "${AUDIT_SHA256}" "${AUDIT_ARCHIVE}" | sha256sum --check --status
tar -xzf "${AUDIT_ARCHIVE}" --directory "${AUDIT_DIRECTORY}" --strip-components=1
AUDIT_STATUS=0
"${AUDIT_DIRECTORY}/cargo-audit" audit --file "${PROJECT_DIR}/Cargo.lock" --deny unsound --json \
  > "${REPORT_DIRECTORY}/report.json" || AUDIT_STATUS=$?
cat -- "${REPORT_DIRECTORY}/report.json"
exit "${AUDIT_STATUS}"
