/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: dialog
    required property var theme
    title: "About MERLIN"
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(560, parent ? parent.width - 48 : 560)
    modal: true
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 16
        Label { text: "MERLIN"; color: dialog.theme.accent; font.pixelSize: 24; font.bold: true }
        Label {
            Layout.fillWidth: true
            text: "Modulation, Encoding, Reception, Link Impairments & Noise"
            wrapMode: Text.WordWrap; color: dialog.theme.text
        }
        Label {
            Layout.fillWidth: true
            text: "Standalone single-channel DOCSIS downstream OFDM simulator."
            wrapMode: Text.WordWrap; color: dialog.theme.text
        }
        Label {
            Layout.fillWidth: true
            text: "Qt desktop: GPL-3.0-only. Reusable Rust libraries: Apache-2.0.\nBuilt with Qt and CXX-Qt; desktop foundations adapted from PNM Viewer contributors.\nThird-party components retain their licenses; see the distributed license and NOTICE files."
            wrapMode: Text.WordWrap; color: dialog.theme.muted
        }
    }
}
