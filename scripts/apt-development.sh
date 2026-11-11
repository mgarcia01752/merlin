#!/usr/bin/env bash
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
# Sourced helper: refresh trusted indexes and install requested development packages.

# Install the supplied package names after a complete index refresh.
# Arguments: one or more package names. Uses normal sudo authentication.
# Return success only after installation; stop after three failed attempts.
apt_development_packages() {
  local attempt
  for attempt in 1 2 3; do
    if sudo apt-get -o Acquire::Retries=2 -o Acquire::http::Timeout=20 \
        -o Acquire::https::Timeout=20 -o APT::Update::Error-Mode=any update &&
       sudo env DEBIAN_FRONTEND=noninteractive apt-get -o Acquire::Retries=2 \
        -o Acquire::http::Timeout=20 -o Acquire::https::Timeout=20 \
        install -y --no-install-recommends "$@"; then
      return 0
    fi
    if [[ "$attempt" != 3 ]]; then
      printf 'Development package setup failed; refreshing indexes before retry %s/3.\n' "$((attempt + 1))" >&2
      sleep "$((attempt * 2))"
    fi
  done
  printf 'Development package setup failed after three attempts.\n' >&2
  return 1
}
