/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
WorkbenchDialog {
    id: dialog
    required property var backend
    property string iconRoot: "qrc:/branding/"
    property alias licensesPopup: licenses
    title: "About MERLIN"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(610, parent ? parent.width - 48 : 610)
    modal: true; standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 16
        RowLayout {
            spacing: 18
            Image { objectName: "aboutIcon"; source: dialog.iconRoot + (dialog.theme.dark ? "merlin-icon-dark.png" : "merlin-icon-light.png"); Layout.preferredWidth: 76; Layout.preferredHeight: 76; fillMode: Image.PreserveAspectFit; sourceSize.width: 152; sourceSize.height: 152 }
            ColumnLayout {
                Label { text: "MERLIN"; color: dialog.theme.accent; font.pixelSize: 27; font.bold: true }
                Label { text: "DOCSIS OFDM Simulation Toolkit"; color: dialog.theme.text; font.pixelSize: 15 }
                Label { objectName: "aboutVersion"; text: "Version " + dialog.backend.version(); color: dialog.theme.muted }
            }
        }
        Label { Layout.fillWidth: true; text: "Modulation, Encoding, Reception, Link Impairments & Noise"; wrapMode: Text.WordWrap; color: dialog.theme.text }
        Label { Layout.fillWidth: true; text: "Standalone single-channel downstream OFDM simulation. Signal processing runs in Rust; the native desktop uses Qt and CXX-Qt."; wrapMode: Text.WordWrap; color: dialog.theme.muted }
        Label {
            objectName: "aboutProjectLink"
            text: '<a style="color: ' + dialog.theme.accent + ';" href="https://github.com/mgarcia01752/merlin">MERLIN on GitHub</a>'
            textFormat: Text.RichText; linkColor: dialog.theme.accent
            onLinkActivated: function(link) { Qt.openUrlExternally(link) }
            Accessible.name: "Open MERLIN GitHub project"
        }
        Rectangle { Layout.fillWidth: true; implicitHeight: 1; color: dialog.theme.line }
        Label { text: "Licenses &amp; Acknowledgements"; textFormat: Text.RichText; color: dialog.theme.text; font.bold: true }
        Label {
            objectName: "aboutLicenseLinks"
            Layout.fillWidth: true; wrapMode: Text.WordWrap; textFormat: Text.RichText
            text: 'Desktop: <a style="color: ' + dialog.theme.accent + ';" href="gpl">GPL-3.0-only</a><br/>Reusable Rust libraries: <a style="color: ' + dialog.theme.accent + ';" href="apache">Apache-2.0</a><br/><a style="color: ' + dialog.theme.accent + ';" href="notice">MERLIN Notices</a> · <a style="color: ' + dialog.theme.accent + ';" href="pnm">PNM Decoder Notices</a>'
            color: dialog.theme.muted; linkColor: dialog.theme.accent
            onLinkActivated: function(key) { licenses.showDocument(key) }
        }
        Label { Layout.fillWidth: true; text: "Desktop foundations and measurement viewers adapted from PNM Viewer contributors. Third-party components retain their own licenses. Bundled license links work offline."; wrapMode: Text.WordWrap; color: dialog.theme.muted; font.pixelSize: 12 }
    }
    LicenseDialog { id: licenses; theme: dialog.theme; backend: dialog.backend }
}
