/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import "../../shared/instruments"

// Preserve downstream independent-comparison labels on the shared drawing.
CountBarChart {
    emptyText: "No retained comparison data."
    categoryAxisLabel: "Received payload sequence"
    zeroMessage: "Zero errors in retained comparisons"
}
