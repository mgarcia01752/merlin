#!/usr/bin/env bash
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Dispatch explicit development setup; arguments/status are forwarded unchanged.
# No default install, compilation, system-wide application install or services.
set -euo pipefail
PROJECT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
if [[ "${1:-}" == "--install-pymerlin" ]]; then
  shift
  exec bash "${PROJECT_DIR}/scripts/python-api/install-local.sh" "$@"
fi
exec bash "${PROJECT_DIR}/scripts/install-development.sh" "$@"
