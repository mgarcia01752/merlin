/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

UpstreamDetailDialog {
    title: "CM Transmitter"
    purpose: "The cable modem creates the upstream waveform from the accepted source and CMTS configuration."
    referencePlane: "Transmit correction C is applied before the physical channel H."
    stages: ["PRBS source → FEC encoder → QAM mapper", "Pilot / resource allocation → frequency interleaving", "CM Pre-Eq C → IFFT → waveform output"]
}
