/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

UpstreamConfigurationDraftDialog {
    sectionId: "receiver"
    approvedFields: ["estimator", "maximum_pilot_gap", "reference_variance", "equalizer_power_floor", "ldpc_iterations"]
    title: "CMTS Receiver"
    purpose: "Receiver processing obtains its observations from the modeled channel."
    referencePlane: "Normal receiver operations use received observations, rather than transmitter truth."
    stages: ["Burst detection / framing → timing → FFT", "Pilot-based channel estimation → equalization", "Resource deinterleaving → soft demapping → LDPC decoding", "Received-data recovery"]
}
