/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

UpstreamConfigurationDraftDialog {
    sectionId: "grants"
    approvedFields: ["grant_id", "epoch_sample", "first_frame", "frames", "minislots"]
    title: "MAP / Grants"
    purpose: "Edit the explicit synthetic grant using supplied timing and minislot choices. Represented MAP and probe controls are inspection only."
    referencePlane: "Control path to the CM transmitter and matching receiver/session configuration."
}
