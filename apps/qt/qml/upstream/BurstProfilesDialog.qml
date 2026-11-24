/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

UpstreamConfigurationDraftDialog {
    sectionId: "profiles"
    approvedFields: ["selected_iuc", "modulation_index", "pilot_pattern", "additional_minislots"]
    title: "IUC / Burst Profiles"
    purpose: "Edit the supplied scheduled-data IUC selection and existing burst-profile groups. Other burst attributes remain unchanged."
    referencePlane: "Only modes admitted by the upstream configuration owner may be applied."
}
