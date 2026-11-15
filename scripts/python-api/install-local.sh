#!/usr/bin/env bash
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Build and install only the native Python API in a private environment.
# Forward installer options to the shared Python implementation; no Qt setup.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_PYTHON="${MERLIN_INSTALL_PYTHON:-python3}"
if [[ "${OS:-}" == Windows_NT && -z "${MERLIN_INSTALL_PYTHON:-}" ]]; then
  INSTALL_PYTHON=python
fi
exec "$INSTALL_PYTHON" "${SCRIPT_DIR}/install.py" --source "$@"
