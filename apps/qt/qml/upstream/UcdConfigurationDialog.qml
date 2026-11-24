/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

UpstreamConfigurationDraftDialog {
    sectionId: "ucd"
    approvedFields: ["selected_preset", "cyclic_prefix_samples", "rolloff_samples", "subcarrier_spacing_khz", "subcarrier_zero_hz", "frame_symbols", "randomization_seed", "encompassed_bandwidth_hz"]
    title: "UCD / OFDMA Configuration"
    purpose: "CMTS control: UCD and channel numerology configure the CM transmitter and matching CMTS receiver."
    referencePlane: "Control input only. This block does not originate upstream samples."
}
