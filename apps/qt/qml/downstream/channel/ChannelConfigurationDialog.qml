/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../shared/components"
import "../../shared/presentation"
import "../instruments/spectrum"
WorkbenchDialog {
    id:dialog
    required property var configuration
    required property var backend
    required property var settingsBackend
    required property var runState
    readonly property var preview: presentation.value ? presentation.value.preview : ({points:[],x_ticks:[],y_ticks:[]})
    readonly property string filterSettings: presentation.value ? presentation.value.filterSettings : ""
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: dialog.stateFor({points: [], x_ticks: [], y_ticks: []})
    }
    Component.onDestruction: {
        if (presentation) {
            presentation.clear()
        }
    }
    function lossCaption() {
        const loss=configuration.additional_loss || {mode:"off"}
        if(loss.mode==="manual"){return "Manual tilt · "+loss.tilt_db+" dB"}
        if(loss.mode==="cable"){return "Belden 1694A · "+loss.length+" "+(loss.unit==="metres" ? "m" : "ft")}
        return "Additional loss Off"
    }
    function stateFor(nextPreview) {
        const filter=runState.options?.channel_filter
        return {preview: nextPreview, filterSettings: JSON.stringify(filter || {}), lossCaption: lossCaption(),
            filterCaption: filter?.mode==="echoes" ? "Synthetic reflected paths" : filter?.mode==="captured_fir" ? "CM capture · continuous FIR" : "Per-subcarrier reference"}
    }
    signal lossRequested()
    signal filterRequested()
    signal contextHelpRequested(string code)
    property alias responsePopup:expanded
    function refresh() {
        if (!opened || !backend || !presentation) {
            return false
        }
        const nextPreview=JSON.parse(backend.channel_preview(settingsBackend.current()))
        return presentation.replace(stateFor(nextPreview))
    }
    onOpened:refresh()
    onConfigurationChanged:refresh()
    onRunStateChanged: {
        if (JSON.stringify(runState.options?.channel_filter || {})!==filterSettings) {
            refresh()
        }
    }
    title:"Channel Model"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(680,parent ? parent.width-40 : 680)
    height:Math.min(650,parent ? parent.height-40 : 650)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        spacing:12
        RowLayout {
            Layout.fillWidth:true;spacing:20
            ChannelModelCard {
                objectName:"channelConfigureLoss";theme:dialog.theme;text:"Tilt / Cable Loss";Layout.fillWidth:true;Layout.preferredWidth:1
                caption:presentation.value ? presentation.value.lossCaption : ""
                onClicked:dialog.lossRequested()
            }
            ChannelModelCard {
                objectName:"channelConfigureFilter";theme:dialog.theme;text:"Echo / Delay";echoes:true;Layout.fillWidth:true;Layout.preferredWidth:1
                caption:presentation.value ? presentation.value.filterCaption : ""
                onClicked:dialog.filterRequested()
            }
        }
        Item {
            Layout.fillWidth:true;Layout.preferredHeight:36
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:parent.width/4;fromY:0;toX:parent.width/4;toY:12;arrowHead:false}
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:parent.width*3/4;fromY:0;toX:parent.width*3/4;toY:12;arrowHead:false}
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:parent.width/4;fromY:12;toX:parent.width*3/4;toY:12;arrowHead:false}
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:parent.width/2;fromY:12;toX:parent.width/2;toY:35}
        }
        WorkbenchActionButton {
            objectName:"channelCombinedResponse";theme:dialog.theme;text:"Combined Channel Response"
            Layout.fillWidth:true;Layout.fillHeight:true
            Accessible.name:text;Accessible.description:"Accepted channel gain versus frequency. Open the enlarged view."
            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus
            ToolTip.text:"Open the enlarged accepted channel response. This is a model preview, not received measurements."
            onClicked:expanded.openChecked()
            contentItem:ColumnLayout {
                Label {text:"Combined Channel Response";color:dialog.theme.text;font.bold:true;Layout.fillWidth:true}
                SpectrumPlot {theme:dialog.theme;viewport:dialog.preview;enabled:false;Layout.fillWidth:true;Layout.fillHeight:true}
            }
        }
        Label {text:"Accepted configuration · before receiver correction";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11}
        ContextualStatus {theme:dialog.theme;text:presentation.rejected ? presentation.message : dialog.preview.error || "";diagnostic:presentation.rejected ? null : dialog.preview.diagnostic || null;Layout.fillWidth:true;onHelpRequested:function(code){dialog.contextHelpRequested(code)}}
    }
    ChannelResponsePreviewDialog {id:expanded;theme:dialog.theme;viewport:dialog.preview;presentationRejected:presentation.rejected;presentationMessage:presentation.message;onContextHelpRequested:function(code){dialog.contextHelpRequested(code)}}
}
