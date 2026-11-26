/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

UpstreamConfigurationDraftDialog {
    sectionId: "source"
    approvedFields: ["source_seed"]
    title: "PRBS-TX"
    purpose: "Inspect the accepted source supplied to the CM transmitter."
    referencePlane: "PRBS-TX is a separate source block. Source identity and generation belong to the direction session."
    stages: []
}
