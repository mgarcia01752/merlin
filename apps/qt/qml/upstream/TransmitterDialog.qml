/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"

UpstreamDetailDialog {
    id: dialog
    signal sourceConfigurationRequested
    readonly property real footerPadding: 10
    readonly property real footerSpacing: 10
    title: "CM Transmitter"
    purpose: "The cable modem creates the upstream waveform from the accepted source and CMTS configuration."
    referencePlane: "Transmit correction C is applied before the physical channel H."
    stages: ["PRBS source → FEC encoder → QAM mapper", "Pilot / resource allocation → frequency interleaving", "CM Pre-Eq C → IFFT → waveform output"]
    footer: Pane {
        padding: dialog.footerPadding
        background: Item {}
        contentItem: RowLayout {
            spacing: dialog.footerSpacing
            WorkbenchActionButton {
                objectName: "upstreamTransmitterConfigureSource"
                theme: dialog.theme
                text: "Configure Source"
                onClicked: dialog.sourceConfigurationRequested()
            }
            Item {
                Layout.fillWidth: true
            }
            WorkbenchActionButton {
                objectName: "upstreamTransmitterClose"
                theme: dialog.theme
                text: "Close"
                onClicked: dialog.reject()
            }
        }
    }
}
