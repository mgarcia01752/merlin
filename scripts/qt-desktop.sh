#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Build, run or test the MERLIN Qt shell using an explicitly available SDK.
# Arguments: run (default), build, check, lock. No SDK downloads or installations.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
ACTION="${1:-run}"
if [[ "${ACTION}" == --help ]]; then
  echo 'Usage: scripts/qt-desktop.sh [run|build|check|lock]'
  echo 'Set QT_SDK_DIR to Qt 6.8.3 gcc_64; default: local/qt/6.8.3/gcc_64.'
  exit 0
fi
QT_SDK_DIR="${QT_SDK_DIR:-${PROJECT_DIR}/local/qt/6.8.3/gcc_64}"
if [[ ! -x "${QT_SDK_DIR}/bin/qmake" ]]; then
  echo 'Qt SDK missing. Set QT_SDK_DIR; see docs/development/qt.md.' >&2
  exit 1
fi
export PATH="${QT_SDK_DIR}/bin:${PATH}"
export LD_LIBRARY_PATH="${QT_SDK_DIR}/lib${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}"
export QMAKE="${QT_SDK_DIR}/bin/qmake"
if [[ "$("${QMAKE}" -query QT_VERSION)" != 6.8.3 ]]; then
  echo "Expected pinned Qt 6.8.3 SDK." >&2
  exit 1
fi
export QT_VERSION_MAJOR=6
export QT_QUICK_CONTROLS_STYLE=Fusion
# Use an independent Cargo cache; existing local/cargo may link into the reference repo.
export PATH="${PROJECT_DIR}/local/cargo/bin:${PATH}"
export CARGO_HOME="${PROJECT_DIR}/local/merlin-cargo"
export CARGO_INCREMENTAL=0
export CARGO_PROFILE_DEV_DEBUG=0
export CARGO_PROFILE_TEST_DEBUG=0
MANIFEST="${PROJECT_DIR}/apps/qt/Cargo.toml"
CARGO=("${SCRIPT_DIR}/cargo-local.sh")
case "${ACTION}" in
  lock) "${CARGO[@]}" generate-lockfile --manifest-path "${MANIFEST}" ;;
  build) "${CARGO[@]}" build --manifest-path "${MANIFEST}" --locked ;;
  check)
    "${CARGO[@]}" fmt --manifest-path "${MANIFEST}" -- --check
    "${CARGO[@]}" clippy --manifest-path "${MANIFEST}" --all-targets --locked -- -D warnings
    "${CARGO[@]}" test --manifest-path "${MANIFEST}" --locked
    "${CARGO[@]}" build --manifest-path "${MANIFEST}" --locked
    mkdir -p "${PROJECT_DIR}/docs/development/local/visual-workspace"
    cd "${PROJECT_DIR}/apps/qt/tests"
    QT_QPA_PLATFORM=offscreen QT_QUICK_BACKEND=software "${QT_SDK_DIR}/bin/qmltestrunner" -input "${PROJECT_DIR}/apps/qt/tests"
    ;;
  run)
    # The pinned development SDK uses XCB, including under XWayland sessions.
    export QT_QPA_PLATFORM="${QT_QPA_PLATFORM:-xcb}"
    if [[ -z "${DISPLAY:-}" && -z "${WAYLAND_DISPLAY:-}" ]]; then
      while IFS='=' read -r setting value; do
        case "${setting}" in
          DISPLAY|XAUTHORITY) export "${setting}=${value}" ;;
        esac
      done < <(systemctl --user show-environment 2>/dev/null || true)
    fi
    if [[ -z "${DISPLAY:-}" && -z "${WAYLAND_DISPLAY:-}" && "${QT_QPA_PLATFORM:-}" != offscreen ]]; then
      echo 'Run from the VM desktop terminal, or configure display forwarding.' >&2
      exit 1
    fi
    exec "${CARGO[@]}" run --manifest-path "${MANIFEST}" --locked
    ;;
  *) echo 'Expected run, build, check or lock.' >&2; exit 2 ;;
esac
