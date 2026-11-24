/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
// Adapted from PNM Viewer ProfileBandwidth.qml.
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"
WorkbenchDialog {
    id: dialog
    objectName: "profileBandwidthDialog"
    property var context: ({})
    property bool sourceCurrent:true
    property PresentationOwner presentationCoverage: PresentationOwner {
        id:coverage;theme:dialog.theme
        initialValue:({context:{profiles:[],selectedIds:[]},rows:[]})
    }

    /**
     * Admit a coherent parent profile/selection snapshot and converted row labels.
     * @returns False preserves all previous rows/highlights under their old owner.
     */
    function refresh() {
        if (!sourceCurrent) {
            return false
        }
        const selected = context.selectedIds || []
        return coverage.replace({context:context,rows:(context.profiles || []).map(function(profile){return {source:profile,label:"Profile "+profile.id,selected:selected.indexOf(profile.id)>=0}})})
    }
    onOpened:refresh()
    onContextChanged:if(opened){refresh()}
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(460, parent.width - 32)
    height: Math.min(400, parent.height - 32)
    modal: true
    title: "Profile Bandwidth"
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        RowLayout {
            Label { text: "Profile"; color: dialog.theme.muted; Layout.fillWidth: true }
            Label { text: "Bandwidth"; color: dialog.theme.muted }
        }
        ScrollView {
            id: scroll
            Layout.fillWidth: true; Layout.fillHeight: true; contentWidth: availableWidth; clip: true
            ColumnLayout {
                width: scroll.availableWidth
                Repeater {
                    model:coverage.value ? coverage.value.rows : []
                    Rectangle {
                        required property var modelData
                        Layout.fillWidth: true; implicitHeight: 36; radius: 4
                        color:modelData.selected ? dialog.theme.accentSoft : "transparent"
                        RowLayout {
                            anchors.fill: parent; anchors.margins: 8
                            Label { text:modelData.label; color: dialog.theme.text; Layout.fillWidth: true }
                            Label { text:modelData.source.bandwidth_text; color: dialog.theme.text; font.family: "monospace" }
                        }
                    }
                }
            }
        }
        Label {objectName:"profileBandwidthRetention";visible:coverage.rejected;text:coverage.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { text: "Data carriers only · ideal gross rate before overhead"; color: dialog.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
    }
}
