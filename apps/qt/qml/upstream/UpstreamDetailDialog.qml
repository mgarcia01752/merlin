/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
import "../shared/presentation"

// Read-only presentation. Supplied rows are display strings, not an API DTO.
WorkbenchDialog {
    id: dialog
    property var presentation: ({})
    readonly property var displayedPresentation: presentationOwner.value ? presentationOwner.value.presentation : ({})
    readonly property var displayedRows: presentationOwner.value ? presentationOwner.value.rows : []
    readonly property bool presentationRejected: presentationOwner.rejected
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentationOwner
        theme: dialog.theme
    }
    property bool presentationReady: false
    function adoptPresentation() {
        if (!presentationReady) {
            return false
        }
        const rows = (presentation.rows || []).map(row => row.label + ": " + row.value)
        return presentationOwner.replace({presentation: presentation, rows: rows})
    }
    onPresentationChanged: adoptPresentation()
    onOpened: adoptPresentation()
    Component.onCompleted: {
        presentationReady = true
        adoptPresentation()
    }
    Component.onDestruction: {
        presentationReady = false
        if (presentationOwner) {
            presentationOwner.clear()
        }
    }
    property string purpose: ""
    property string referencePlane: ""
    property string captureFamily: ""
    property bool captureBusy: false
    signal captureRequested(string family)
    property bool fecInspection: false
    signal fecRequested()
    property var stages: []
    readonly property real preferredDialogWidth: 620
    readonly property real preferredDialogHeight: 560
    readonly property real windowMargin: 24
    readonly property real sectionSpacing: 18
    readonly property real rowSpacing: 8
    readonly property real bodyFontSize: 14
    readonly property real captionFontSize: 12
    readonly property real headingFontSize: 15
    readonly property real panelRadius: 8
    readonly property real panelPadding: 14
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(preferredDialogWidth, parent ? parent.width - windowMargin * 2 : preferredDialogWidth)
    height: Math.min(preferredDialogHeight, parent ? parent.height - windowMargin * 2 : preferredDialogHeight)
    modal: true
    focus: true
    standardButtons: Dialog.Close
    closePolicy: Popup.CloseOnEscape
    background: Rectangle {
        color: dialog.theme.panel
        border.color: dialog.theme.line
        radius: dialog.panelRadius
    }
    contentItem: ScrollView {
        id: scroller
        clip: true
        contentWidth: availableWidth
        ColumnLayout {
            width: scroller.availableWidth
            spacing: dialog.sectionSpacing
            WorkbenchActionButton {
                objectName: "upstreamDetailFecInspect"
                theme: dialog.theme
                text: "Inspect FEC / LLR"
                visible: dialog.fecInspection
                enabled: !dialog.captureBusy
                onClicked: {
                    dialog.close();
                    dialog.fecRequested();
                }
            }
            WorkbenchActionButton {
                objectName: "upstreamCaptureInspect"
                theme: dialog.theme
                text: "View Capture"
                visible: dialog.captureFamily.length > 0
                enabled: !dialog.captureBusy
                onClicked: {
                    dialog.close();
                    dialog.captureRequested(dialog.captureFamily);
                }
            }
            Label {
                Layout.fillWidth: true
                text: dialog.purpose
                wrapMode: Text.WordWrap
                color: dialog.theme.text
                font.pixelSize: dialog.bodyFontSize
            }
            Label {
                objectName: "upstreamReferencePlane"
                Layout.fillWidth: true
                text: dialog.referencePlane
                wrapMode: Text.WordWrap
                color: dialog.theme.accent
                font.pixelSize: dialog.captionFontSize
            }
            Label {
                Layout.fillWidth: true
                text: "Processing path"
                visible: dialog.stages.length > 0
                color: dialog.theme.text
                font.bold: true
                font.pixelSize: dialog.headingFontSize
            }
            Repeater {
                model: dialog.stages
                Label {
                    required property string modelData
                    Layout.fillWidth: true
                    text: modelData
                    color: dialog.theme.muted
                    wrapMode: Text.WordWrap
                    font.pixelSize: dialog.bodyFontSize
                }
            }
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: observationColumn.implicitHeight + dialog.panelPadding * 2
                color: dialog.theme.background
                radius: dialog.panelRadius
                ColumnLayout {
                    id: observationColumn
                    anchors.fill: parent
                    anchors.margins: dialog.panelPadding
                    spacing: dialog.rowSpacing
                    Label {
                        Layout.fillWidth: true
                        text: "Accepted state / observations"
                        color: dialog.theme.text
                        font.bold: true
                        font.pixelSize: dialog.headingFontSize
                    }
                    Label {
                        objectName: "upstreamObservationSummary"
                        Layout.fillWidth: true
                        textFormat: Text.PlainText
                        text: dialog.displayedPresentation.summary || "Unavailable. Configure this direction to obtain accepted state and observations."
                        color: dialog.theme.muted
                        wrapMode: Text.WordWrap
                        font.pixelSize: dialog.bodyFontSize
                    }
                    Label {
                        Layout.fillWidth: true
                        text: presentationOwner.message
                        visible: presentationOwner.rejected
                        color: dialog.theme.statusAttention
                        wrapMode: Text.WordWrap
                    }
                    Repeater {
                        model: dialog.displayedRows
                        Label {
                            required property string modelData
                            Layout.fillWidth: true
                            textFormat: Text.PlainText
                            text: modelData
                            color: dialog.theme.text
                            wrapMode: Text.WrapAnywhere
                            font.pixelSize: dialog.bodyFontSize
                        }
                    }
                }
            }
        }
    }
}
