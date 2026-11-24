/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

UpstreamConfigurationDraftDialog {
    sectionId: "channel"
    responseInspection: true
    approvedFields: ["delay_samples", "noise_seed", "noise_variance", "phase_radians", "frequency_offset_hz"]
    title: "Channel Simulator"
    purpose: "Inspect the admitted upstream physical channel and measurement-driven impairment model."
    referencePlane: "Physical response H and effective response C×H are distinct. CMTS-measured RxMER follows the admitted residual model."
    stages: []
}
