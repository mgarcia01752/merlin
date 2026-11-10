#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Adapted from PNM Viewer Toolkit development setup; all destinations are MERLIN.
# Install missing Ubuntu packages and project-local Rust/Qt, then fetch locked sources.
# --check/--dry-run never download, elevate privileges or change files.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
DEVELOPMENT=0
MODE=install

# Describe supported modes and effects without modifying host state.
# Arguments are unused. Return success.
usage() {
  cat <<'HELP'
Usage: ./install.sh --development [--check | --dry-run]

  -d, --development  Set up Ubuntu 22.04/24.04/26.04 x64 development
      --check        Report package/tool/source readiness without changes
      --dry-run      Show missing packages and setup plan without changes
  -h, --help         Show this help

Install mode uses sudo apt-get only for missing system packages. Run as your
normal user. Rust, Qt and pinned dependency sources stay under ignored local/.
No application install, compilation, tests, services or shell-profile changes.
HELP
}
for argument in "$@"; do
  case "$argument" in
    -d|--development) DEVELOPMENT=1 ;;
    --check|--dry-run)
      if [[ "$MODE" != install ]]; then
        printf 'Choose only one of --check or --dry-run.\n' >&2
        exit 2
      fi
      MODE="${argument#--}"
      ;;
    -h|--help) usage; exit 0 ;;
    *) printf 'Unknown option: %s\n' "$argument" >&2; usage >&2; exit 2 ;;
  esac
done
if [[ "$DEVELOPMENT" != 1 ]]; then
  usage >&2
  exit 2
fi
if [[ "$(uname -s)" != Linux || "$(uname -m)" != x86_64 || ! -r /etc/os-release ]]; then
  printf 'Initial development setup targets Ubuntu x86_64.\n' >&2
  exit 2
fi
# shellcheck source=/dev/null
source /etc/os-release
case "${ID:-}:${VERSION_ID:-}" in
  ubuntu:22.04|ubuntu:24.04|ubuntu:26.04) ;;
  *) printf 'Unsupported host: %s\n' "${PRETTY_NAME:-unknown}" >&2; exit 2 ;;
esac
# Whole channel line: channel = "1.85.1" -> capture the quoted version only.
RUST_VERSION="$(sed -n 's/^channel = "\([^"]*\)"$/\1/p' "${PROJECT_DIR}/rust-toolchain.toml")"
if [[ -z "$RUST_VERSION" ]]; then
  printf 'Missing Rust channel in rust-toolchain.toml.\n' >&2
  exit 2
fi
PACKAGES=(
  ca-certificates curl git build-essential cmake ninja-build pkg-config libssl-dev
  clang libclang-dev lld ccache patchelf dpkg-dev fakeroot
  libgl-dev libegl-dev libopengl-dev libxkbcommon-x11-dev
  libxcb-cursor-dev libxcb-icccm4-dev libxcb-image0-dev libxcb-keysyms1-dev
  libxcb-render-util0-dev libxcb-shape0-dev libxcb-randr0-dev libxcb-xfixes0-dev
  libxcb-sync-dev libxcb-xinerama0-dev libxcb-xkb-dev libx11-xcb-dev libxtst6
  libxcb-glx0-dev libxcb-shm0-dev libxcb-util-dev libxcb1-dev libx11-dev
  libxext-dev libxfixes-dev libxi-dev libxkbcommon-dev libxrender-dev
  libsm-dev libice-dev libatspi2.0-dev
  libgtk-3-dev libglib2.0-dev libb2-dev libdbus-1-dev libfontconfig1-dev libfreetype6-dev
  wget file xz-utils unzip zip zstd python3 python3-venv python3-pip python3-pil
  python3-gi gir1.2-gtk-3.0 shellcheck xvfb xauth x11-utils xwayland desktop-file-utils fonts-dejavu-core
)
MISSING_PACKAGES=()
for package in "${PACKAGES[@]}"; do
  if [[ "$(dpkg-query -W -f='${Status}' "$package" 2>/dev/null || true)" != 'install ok installed' ]]; then
    MISSING_PACKAGES+=("$package")
  fi
done
export CARGO_HOME="${PROJECT_DIR}/local/cargo"
export RUSTUP_HOME="${PROJECT_DIR}/local/rustup"
export PATH="${CARGO_HOME}/bin:${PATH}"
RUST_READY=0
if [[ -x "${CARGO_HOME}/bin/rustup" ]] &&
   "${CARGO_HOME}/bin/rustup" run "$RUST_VERSION" rustc --version >/dev/null 2>&1 &&
   "${CARGO_HOME}/bin/rustup" run "$RUST_VERSION" cargo fmt --version >/dev/null 2>&1 &&
   "${CARGO_HOME}/bin/rustup" run "$RUST_VERSION" cargo clippy --version >/dev/null 2>&1; then
  RUST_READY=1
fi
QT_READY=0
if command -v python3 >/dev/null 2>&1 && python3 "${PROJECT_DIR}/tools/development/qt_sdk.py" check >/dev/null 2>&1; then
  QT_READY=1
fi
SOURCE_READY=0
if command -v python3 >/dev/null 2>&1 && python3 "${PROJECT_DIR}/tools/development/check_sources.py" >/dev/null 2>&1; then
  SOURCE_READY=1
fi
printf 'Host: %s (x86_64)\n' "$PRETTY_NAME"
printf 'Missing apt packages: %s\n' "${MISSING_PACKAGES[*]:-none}"
printf 'Local Rust %s ready: %s\nQt SDK ready: %s\nPinned PNM source ready: %s\n' "$RUST_VERSION" "$RUST_READY" "$QT_READY" "$SOURCE_READY"
if [[ "$MODE" == check ]]; then
  [[ ${#MISSING_PACKAGES[@]} == 0 && "$RUST_READY" == 1 && "$QT_READY" == 1 && "$SOURCE_READY" == 1 ]]
  exit $?
fi
if [[ "$MODE" == dry-run ]]; then
  printf 'Would install missing apt packages and project-local Rust/Qt, then cargo fetch --locked for CLI and Qt.\n'
  printf 'No dependency source is copied into tracked MERLIN directories. No compilation or tests.\n'
  exit 0
fi
if [[ "$EUID" == 0 ]]; then
  printf 'Run as your normal user; only apt uses sudo.\n' >&2
  exit 2
fi
# Reject redirected local tool roots before writes or installation.
python3 - "${PROJECT_DIR}" <<'PY'
from pathlib import Path
import sys
root = Path(sys.argv[1]).resolve()
for name in ('local', 'local/cargo', 'local/rustup', 'local/merlin-cargo'):
    if not (root / name).resolve().is_relative_to(root):
        raise SystemExit(f'Refusing setup through an external tool directory: {name}')
PY
if [[ ${#MISSING_PACKAGES[@]} -gt 0 ]]; then
  sudo apt-get update
  sudo env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "${MISSING_PACKAGES[@]}"
fi
mkdir -p "${PROJECT_DIR}/local"
TEMP_DIR="$(mktemp -d "${PROJECT_DIR}/local/development-install.XXXXXX")"
trap 'rm -rf -- "$TEMP_DIR"' EXIT
if [[ "$RUST_READY" != 1 ]]; then
  if [[ ! -x "${CARGO_HOME}/bin/rustup" ]]; then
    curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs -o "${TEMP_DIR}/rustup-init.sh"
    bash "${TEMP_DIR}/rustup-init.sh" -y --no-modify-path --profile minimal --default-toolchain none
  fi
  "${CARGO_HOME}/bin/rustup" toolchain install "$RUST_VERSION" --profile minimal --component rustfmt --component clippy
fi
python3 "${PROJECT_DIR}/tools/development/qt_sdk.py" install
"${SCRIPT_DIR}/cargo-local.sh" fetch --manifest-path "${PROJECT_DIR}/Cargo.toml" --locked
CARGO_HOME="${PROJECT_DIR}/local/merlin-cargo" "${SCRIPT_DIR}/cargo-local.sh" fetch --manifest-path "${PROJECT_DIR}/apps/qt/Cargo.toml" --locked
bash "${PROJECT_DIR}/install.sh" --development --check
printf 'Development setup ready. Build with scripts/qt-desktop.sh build.\n'
