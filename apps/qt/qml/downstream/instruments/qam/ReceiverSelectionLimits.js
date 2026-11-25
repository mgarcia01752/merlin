/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
.pragma library

// Shared legacy coordinate input hints for receiver-cell and tap-QAM selectors.
// Rust validates every query; these are not claims of scientific support.
// Independent negative sentinels retain the existing All selection wire values.
const ALL_PHYSICAL_BINS = -1;
const ALL_RECEIVER_SYMBOLS = -1;
const MAXIMUM_PHYSICAL_BIN_HINT = 8191;
const MAXIMUM_RX_SYMBOL_HINT = 1000000;
