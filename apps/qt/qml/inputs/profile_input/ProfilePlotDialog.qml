/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
// Adapted from PNM Viewer ModulationControls.qml and ProfileBandwidth.qml.
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"

WorkbenchDialog {
    id: dialog
    objectName: "profilePlotDialog"
    required property var backend
    property var profileIds: []
    property int initialProfile: -1
    property int selectedProfile: -1
    property var selectedIds: []
    property var selectedRoles: []
    property int metric: 0
    property var viewport: ({points: [], roles: [], profiles: []})
    readonly property var allRoles: ["data", "continuous_pilot", "plc", "exclusion", "zero_bit_loaded", "reserved", "unknown"]
    readonly property var roleColors: theme.dark ? ({data: "#51d6bf", continuous_pilot: "#ffd166", plc: "#7cb7ff", exclusion: "#f4a261", zero_bit_loaded: "#bfdd73", reserved: "#c998ff", unknown: "#ff80bb"}) : ({data:"#007e75",continuous_pilot:"#996000",plc:"#2563b9",exclusion:"#b64b20",zero_bit_loaded:"#517c13",reserved:"#8544bb",unknown:"#b82c68"})
    function profileColor(id) { return (theme.dark ? ["#51d6bf", "#7cb7ff", "#ffd166", "#c998ff", "#ff80bb"] : ["#007e75", "#2563b9", "#996000", "#8544bb", "#b82c68"])[id % 5] }
    function toggle(values, value) { const copy = values.slice(); const index = copy.indexOf(value); if (index >= 0) copy.splice(index, 1); else copy.push(value); return copy }
    function refresh(start, count) { viewport = JSON.parse(backend.inspect(JSON.stringify({profiles: selectedIds, roles: selectedRoles, metric: metric, start: start, count: count}))) }
    function refreshRange() { refresh(viewport.start || 0, viewport.count || 0) }
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(1100, parent ? parent.width - 32 : 1100)
    height: Math.min(760, parent ? parent.height - 32 : 760)
    title: "Downstream Modulation Profile"
    modal: true
    standardButtons: Dialog.Close
    onOpened: { selectedProfile = initialProfile; selectedIds = [initialProfile]; selectedRoles = allRoles.slice(); metric = 0; refresh(0, 0) }
    ProfileBandwidthDialog { id: bandwidth; theme: dialog.theme; profiles: dialog.viewport.profiles || []; selectedIds: dialog.selectedIds }
    component ProfileToggle: Button {
        id: toggle
        property color traceColor: dialog.theme.accent
        property bool showIndicator: true
        implicitWidth: 108; implicitHeight: 30
        checkable: true; padding: 8
        contentItem: Row {
            spacing: 6
            Rectangle { visible: toggle.showIndicator; width: 10; height: 3; anchors.verticalCenter: parent.verticalCenter; color: toggle.traceColor }
            Label { text: toggle.text; color: dialog.theme.text; font.pixelSize: 12 }
        }
        background: Rectangle {
            radius: 5; color: toggle.checked ? dialog.theme.accentSoft : dialog.theme.button
            border.color: toggle.checked ? toggle.traceColor : dialog.theme.line
            border.width: toggle.activeFocus ? 2 : 1
            opacity: toggle.enabled ? 1 : 0.4
        }
    }
    contentItem: ColumnLayout {
        spacing: 8
        RowLayout {
            Layout.fillWidth: true
            Repeater {
                model: ["Channel  " + (dialog.viewport.channel ?? "—"), "MAC  " + (dialog.viewport.mac || "—")]
                Label { required property string modelData; text: modelData; color: dialog.theme.text; padding: 8; background: Rectangle { radius: height/2; color: dialog.theme.button; border.color: dialog.theme.line } }
            }
            Button {
                objectName: "profileBandwidthChip"
                text: "Profile BW  " + (dialog.selectedIds.length === 1 ? (dialog.viewport.profiles || []).filter(p => dialog.selectedIds.indexOf(p.id) >= 0).map(p => p.id + " · " + p.bandwidth_text).join("") : dialog.selectedIds.length + " profiles") + "  ⌄"
                Layout.maximumWidth: 490
                onClicked: bandwidth.open()
            }
            Item { Layout.fillWidth: true }
            Button { objectName: "profilePlotFullSpan"; text: "Full Span"; onClicked: dialog.refresh(0, 0) }
        }
        GridLayout {
            columns: 2; columnSpacing: 10; rowSpacing: 6; Layout.fillWidth: true
            Label { text: "Y Axis"; color: dialog.theme.muted }
            ComboBox { objectName: "modulationMetricSelector"; model: ["dB", "Bit loading"]; currentIndex: dialog.metric; Layout.preferredWidth: 160; onActivated: function(index) { dialog.metric = index; dialog.refreshRange() } ToolTip.visible: hovered; ToolTip.text: "dB: ideal Shannon minimum MER, not measured RxMER" }
            Label { text: "Profiles"; color: dialog.theme.muted; Layout.alignment: Qt.AlignTop; topPadding: 8 }
            Flow {
                Layout.fillWidth: true; Layout.preferredHeight: childrenRect.height; spacing: 6
                Repeater {
                    objectName: "profileToggleRepeater"
                    model: (dialog.viewport.profiles || []).slice().reverse()
                    ProfileToggle {
                        required property var modelData
                        objectName: "modulationProfileToggle" + modelData.id
                        text: "Profile " + modelData.id; checkable: true; checked: dialog.selectedIds.indexOf(modelData.id) >= 0
                        traceColor: dialog.profileColor(modelData.id)
                        onClicked: { dialog.selectedIds = dialog.toggle(dialog.selectedIds, modelData.id); dialog.refreshRange() }
                    }
                }
                ProfileToggle { objectName: "modulationAllProfiles"; text: "All"; checkable: false; showIndicator: false; checked: dialog.selectedIds.length === dialog.profileIds.length; onClicked: { dialog.selectedIds = dialog.profileIds.slice(); dialog.refreshRange() } }
            }
            Label { text: "Carriers"; color: dialog.theme.muted; Layout.alignment: Qt.AlignTop; topPadding: 8 }
            Flow {
                Layout.fillWidth: true; Layout.preferredHeight: childrenRect.height; spacing: 6
                Repeater {
                    objectName: "carrierToggleRepeater"
                    model: dialog.viewport.roles || []
                    ProfileToggle {
                        required property var modelData
                        objectName: "modulationCarrierToggle" + modelData.kind
                        text: modelData.label; checkable: true; checked: dialog.selectedRoles.indexOf(modelData.kind) >= 0
                        enabled: modelData.count > 0
                        traceColor: dialog.roleColors[modelData.kind]
                        onClicked: { dialog.selectedRoles = dialog.toggle(dialog.selectedRoles, modelData.kind); dialog.refreshRange() }
                        ToolTip.visible: hovered; ToolTip.text: modelData.count + " assignments in selected profiles"
                    }
                }
                ProfileToggle { objectName: "modulationAllCarriers"; text: "All"; checkable: false; showIndicator: false; checked: dialog.selectedRoles.length === 7; onClicked: { dialog.selectedRoles = dialog.allRoles.slice(); dialog.refreshRange() } }
            }
        }
        ProfileChart {
            Layout.fillWidth: true; Layout.fillHeight: true
            theme: dialog.theme; viewport: dialog.viewport; roleColors: dialog.roleColors
            profileColor: dialog.profileColor
            onViewportRequested: function(start, count) { dialog.refresh(start, count) }
        }
        Label { visible: !!dialog.viewport.error; text: dialog.viewport.error || ""; color: dialog.theme.statusError }
    }
}
