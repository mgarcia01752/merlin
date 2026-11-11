/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
import "../measurement_input"

WorkbenchDialog {
    id: dialog
    objectName: "profilePlotDialog"
    required property var backend
    property var profileIds: []
    property int initialProfile: -1
    property int selectedProfile: -1
    property var viewport: ({points: [], roles: []})
    readonly property var roleColors: ({data: "#36d985", continuous_pilot: "#ffd166", plc: "#ff667a", exclusion: "#94a3b8", zero_bit_loaded: "#7cb7ff", reserved: "#c998ff", unknown: "#f4a261"})
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(960, parent ? parent.width - 48 : 960)
    height: Math.min(620, parent ? parent.height - 48 : 620)
    title: "Downstream Modulation Profile"
    modal: true
    standardButtons: Dialog.Close
    function refresh(start, count) {
        viewport = JSON.parse(backend.plot(selectedProfile, start, count))
    }
    onOpened: { selectedProfile = initialProfile; refresh(0, 0) }
    contentItem: ColumnLayout {
        spacing: 12
        RowLayout {
            Layout.fillWidth: true
            Label { text: "Profile"; color: dialog.theme.text }
            ComboBox {
                objectName: "profilePlotChoice"
                model: dialog.profileIds
                currentIndex: dialog.profileIds.indexOf(dialog.selectedProfile)
                Accessible.name: "Viewed modulation profile"
                onActivated: function(index) { dialog.selectedProfile = dialog.profileIds[index]; dialog.refresh(0, 0) }
            }
            Item { Layout.fillWidth: true }
            Button { objectName: "profilePlotFullSpan"; text: "Full Span"; onClicked: dialog.refresh(0, 0) }
        }
        Flow {
            Layout.fillWidth: true
            Layout.preferredHeight: childrenRect.height
            spacing: 18
            Repeater {
                model: dialog.viewport.roles || []
                Row {
                    required property var modelData
                    spacing: 6
                    Rectangle { width: 10; height: 10; radius: 2; anchors.verticalCenter: parent.verticalCenter; color: dialog.roleColors[modelData.kind] || dialog.theme.accent }
                    Label { text: modelData.label; color: dialog.theme.text }
                }
            }
        }
        MeasurementPlot {
            Layout.fillWidth: true; Layout.fillHeight: true
            theme: dialog.theme
            viewport: dialog.viewport
            roleColors: dialog.roleColors
            onViewportRequested: function(start, count) { dialog.refresh(start, count) }
        }
        Label { visible: dialog.viewport.role_markers || false; text: "Roles without bit loading are marked along the baseline."; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
}
