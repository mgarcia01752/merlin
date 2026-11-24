/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

UpstreamConfigurationDraftDialog {
    sectionId: "preEqualization"
    captureFamily: "pre_eq"
    approvedFields: ["initial_id"]
    title: "CM Pre-Eq"
    purpose: "Inspect measured cable-modem pre-equalization coefficients and their admitted use."
    referencePlane: "Transmit correction C precedes physical channel H. These coefficients are not a CMTS channel-estimation capture."
    stages: []
}
