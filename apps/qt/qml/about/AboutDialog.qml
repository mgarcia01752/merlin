/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../components"

WorkbenchDialog {
    id: dialog
    required property var backend
    property string iconRoot: "qrc:/branding/"
    signal updatesRequested()
    signal linkRequested(url target)
    title: "About MERLIN"
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(650, parent ? parent.width - 48 : 650)
    modal: true
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 20
        RowLayout {
            spacing: 20
            Image {
                objectName: "aboutIcon"
                source: dialog.iconRoot + (dialog.theme.dark ? "merlin-icon-dark.png" : "merlin-icon-light.png")
                Layout.preferredWidth: 80
                Layout.preferredHeight: 80
                fillMode: Image.PreserveAspectFit
                sourceSize.width: 160
                sourceSize.height: 160
            }
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6
                Label { text: "MERLIN"; color: dialog.theme.accent; font.pixelSize: 28; font.bold: true }
                Label { text: "DOCSIS OFDM Simulation Toolkit"; color: dialog.theme.text; font.pixelSize: 15; Layout.fillWidth: true; wrapMode: Text.WordWrap }
                Label { objectName: "aboutVersion"; text: "Version " + dialog.backend.version(); color: dialog.theme.muted }
            }
        }
        Label {
            Layout.fillWidth: true
            text: "Modulation, Encoding, Reception, Link Impairments & Noise"
            wrapMode: Text.WordWrap
            color: dialog.theme.muted
        }
        Rectangle { Layout.fillWidth: true; implicitHeight: 1; color: dialog.theme.line }
        GridLayout {
            id: actions
            Layout.fillWidth: true
            property real minimumActionWidth: Math.ceil(Math.max(projectAction.implicitWidth,
                reportAction.implicitWidth, updateAction.implicitWidth))
            columns: width >= minimumActionWidth * 3 + columnSpacing * 2 ? 3 : 1
            columnSpacing: 10
            rowSpacing: 10
            WorkbenchActionButton {
                theme: dialog.theme
                id: projectAction
                objectName: "aboutProjectButton"
                text: "Project on GitHub"
                Layout.fillWidth: true; Layout.minimumWidth: actions.minimumActionWidth
                Layout.preferredWidth: actions.minimumActionWidth
                onClicked: dialog.linkRequested("https://github.com/mgarcia01752/merlin")
            }
            WorkbenchActionButton {
                theme: dialog.theme
                id: reportAction
                objectName: "aboutReportIssue"
                text: "Report Issue"
                Layout.fillWidth: true; Layout.minimumWidth: actions.minimumActionWidth
                Layout.preferredWidth: actions.minimumActionWidth
                onClicked: dialog.linkRequested("https://github.com/mgarcia01752/merlin/issues/new")
                Accessible.description: "Open the MERLIN issue form on GitHub"
            }
            WorkbenchActionButton {
                theme: dialog.theme
                id: updateAction
                objectName: "aboutCheckUpdates"
                text: "Check For Updates"
                Layout.fillWidth: true; Layout.minimumWidth: actions.minimumActionWidth
                Layout.preferredWidth: actions.minimumActionWidth
                onClicked: { dialog.close(); dialog.updatesRequested() }
            }
        }
    }
}
