/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
import "../../instruments/spectrum"
WorkbenchDialog {
    id:dialog
    required property var configuration
    required property var backend
    required property var settingsBackend
    required property var runState
    property var preview:({points:[],x_ticks:[],y_ticks:[]})
    readonly property string filterSettings:JSON.stringify(runState.options?.channel_filter || {})
    signal lossRequested()
    signal filterRequested()
    signal contextHelpRequested(string code)
    property alias responsePopup:expanded
    function refresh(){
        if(opened && backend){preview=JSON.parse(backend.channel_preview(settingsBackend.current()))}
    }
    onOpened:refresh()
    onConfigurationChanged:refresh()
    onFilterSettingsChanged:refresh()
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
                caption:{
                    const loss=dialog.configuration.additional_loss || {mode:"off"}
                    if(loss.mode==="manual"){return "Manual tilt · "+loss.tilt_db+" dB"}
                    if(loss.mode==="cable"){return "Belden 1694A · "+loss.length+" "+(loss.unit==="metres" ? "m" : "ft")}
                    return "Additional loss Off"
                }
                onClicked:dialog.lossRequested()
            }
            ChannelModelCard {
                objectName:"channelConfigureFilter";theme:dialog.theme;text:"Echo / Delay";echoes:true;Layout.fillWidth:true;Layout.preferredWidth:1
                caption:dialog.runState.options?.channel_filter?.mode==="echoes" ? "Synthetic reflected paths" : dialog.runState.options?.channel_filter?.mode==="captured_fir" ? "CM capture · continuous FIR" : "Per-subcarrier reference"
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
            ToolTip.visible:hovered || visualFocus
            ToolTip.text:"Open the enlarged accepted channel response. This is a model preview, not received measurements."
            onClicked:expanded.open()
            contentItem:ColumnLayout {
                Label {text:"Combined Channel Response";color:dialog.theme.text;font.bold:true;Layout.fillWidth:true}
                SpectrumPlot {theme:dialog.theme;viewport:dialog.preview;enabled:false;Layout.fillWidth:true;Layout.fillHeight:true}
            }
        }
        Label {text:"Accepted configuration · before receiver correction";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11}
        ContextualStatus {theme:dialog.theme;text:dialog.preview.error || "";diagnostic:dialog.preview.diagnostic || null;Layout.fillWidth:true;onHelpRequested:function(code){dialog.contextHelpRequested(code)}}
    }
    ChannelResponsePreviewDialog {id:expanded;theme:dialog.theme;viewport:dialog.preview;onContextHelpRequested:function(code){dialog.contextHelpRequested(code)}}
}
