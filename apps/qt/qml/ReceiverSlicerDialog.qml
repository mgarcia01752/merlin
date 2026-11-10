/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
WorkbenchDialog {
    id:dialog
    required property var backend
    property var runState: ({})
    property string seenToken:""
    property bool decoderInput:false
    property bool updating:false
    property var diagnosticState: ({rows:[],histogram:{points:[],x_ticks:[],y_ticks:[]}})
    function refresh(){if (!backend || !selection || updating){return};diagnosticState=JSON.parse(backend.slicer(JSON.stringify(selection.query()),decoderInput));if (backend.qam_selected){selection.metadata=JSON.parse(backend.qam_selected(JSON.stringify(selection.query())))}}
    function useSelection(query) {
        updating=true
        selection.signalIndex=query.signal;selection.plcPartIndex=query.part;selection.profile=query.profile;selection.order=query.order
        selection.firstBin=query.first_bin;selection.lastBin=query.last_bin;selection.firstSymbol=query.first_symbol;selection.lastSymbol=query.last_symbol
        updating=false;refresh()
    }
    onOpened:refresh()
    onRunStateChanged:{if (opened && runState.result_token!==seenToken){seenToken=runState.result_token || "";refresh()} }
    title:"Slicer / LLR"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1120,parent ? parent.width-40 : 1120);height:Math.min(720,parent ? parent.height-40 : 720)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        spacing:8
        RowLayout {
            Label { text:"Soft Boundary";color:dialog.theme.text }
            ComboBox { objectName:"receiverSoftBoundary";model:["Selected Cell Decisions / LLRs","Payload LDPC Input"];currentIndex:dialog.decoderInput ? 1 : 0;Layout.preferredWidth:270;onActivated:function(index){dialog.decoderInput=index===1;dialog.refresh()} }
            Label { text:dialog.diagnosticState.llr ? dialog.diagnosticState.llr.count+" actual LLRs" : "No LLRs";color:dialog.theme.accent }
            Item { Layout.fillWidth:true }
        }
        ReceiverCellControls { id:selection;theme:dialog.theme;visible:!dialog.decoderInput;showView:false;Layout.fillWidth:true;onSelectionChanged:dialog.refresh() }
        Label { text:dialog.diagnosticState.detail || dialog.diagnosticState.reason || "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11 }
        SpectrumPlot { theme:dialog.theme;viewport:dialog.diagnosticState.histogram || ({points:[],x_ticks:[],y_ticks:[],error:dialog.diagnosticState.reason});Layout.fillWidth:true;Layout.fillHeight:true;Layout.minimumHeight:100 }
        Label { visible:!dialog.decoderInput;text:"Physical decisions · matched receive coordinates · actual post-descrambling LLRs";color:dialog.theme.accent;font.bold:true }
        ListView {
            objectName:"receiverDecisionRows";visible:!dialog.decoderInput;model:dialog.diagnosticState.rows || []
            Layout.fillWidth:true;Layout.preferredHeight:145;clip:true;spacing:4
            ScrollBar.vertical:ScrollBar {}
            delegate:Rectangle {
                required property var modelData
                width:ListView.view.width;height:64;color:dialog.theme.panel;radius:5
                ColumnLayout {
                    anchors.fill:parent;anchors.margins:6;spacing:2
                    Label { text:"RX "+modelData.symbol+" · bin "+modelData.bin+" · "+modelData.order+" QAM · I/Q "+modelData.iq.map(function(v){return Number(v).toFixed(4)}).join(", ")+" · label "+modelData.label+" · residual² "+Number(modelData.distance_squared).toPrecision(4);color:dialog.theme.text;Layout.fillWidth:true;elide:Text.ElideRight;font.pixelSize:11 }
                    TextEdit { text:"LLRs: "+(modelData.llrs.length ? modelData.llrs.map(function(v){return Number(v).toPrecision(4)}).join("  ") : "No retained soft boundary for this stream");readOnly:true;selectByMouse:true;wrapMode:TextEdit.Wrap;color:dialog.theme.muted;Layout.fillWidth:true;Layout.fillHeight:true;font.pixelSize:11 }
                }
            }
        }
    }
}
