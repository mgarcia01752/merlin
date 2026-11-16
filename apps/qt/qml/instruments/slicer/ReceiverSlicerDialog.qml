/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
import "../qam"
import "../spectrum"
WorkbenchDialog {
    id:dialog
    signal contextHelpRequested(string code)
    required property var backend
    property var runState: ({})
    property string seenToken:""
    readonly property bool decoderInput: tabs.currentIndex === 2
    property int selectedCell: 0
    property bool updating:false
    property var diagnosticState: ({rows:[],histogram:{points:[],x_ticks:[],y_ticks:[]}})
    function refresh(){if (!backend || !selection || updating){return};selectedCell=0;diagnosticState=JSON.parse(backend.slicer(JSON.stringify(selection.query()),decoderInput));if (backend.qam_selected){selection.metadata=JSON.parse(backend.qam_selected(JSON.stringify(selection.query())))}}
    function useSelection(query) {
        updating=true
        selection.runProfile=query.run_profile===undefined ? -1 : query.run_profile
        selection.signalIndex=query.signal;selection.plcPartIndex=query.part;selection.profile=query.profile;selection.order=query.order
        selection.firstBin=query.first_bin;selection.lastBin=query.last_bin;selection.firstSymbol=query.first_symbol;selection.lastSymbol=query.last_symbol
        updating=false;refresh()
    }
    onDecoderInputChanged:if(opened){refresh()}
    onOpened:refresh()
    onRunStateChanged:{if (opened && runState.result_token!==seenToken){seenToken=runState.result_token || "";refresh()} }
    title:"Slicer / LLR"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1120,parent ? parent.width-40 : 1120);height:Math.min(720,parent ? parent.height-40 : 720)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        spacing:8
        TabBar {
            id:tabs;objectName:"receiverSoftTabs";Layout.fillWidth:true;currentIndex:1
            TabButton {text:"Symbol Decisions"}
            TabButton {text:"Bit Confidence"}
            TabButton {text:"Decoder Input"}
        }
        Label {text:dialog.diagnosticState.detail || "";color:dialog.theme.muted;wrapMode:Text.WordWrap;Layout.fillWidth:true}
        ComboBox {
            visible:dialog.decoderInput;objectName:"decoderRunProfile"
            readonly property var runs:selection.metadata.runs || []
            model:["Latest Run"].concat(runs.map(function(p){return "Profile "+p.id}))
            currentIndex:selection.runProfile<0 ? 0 : runs.findIndex(function(p){return p.id===selection.runProfile})+1
            onActivated:function(index){selection.runProfile=index===0 ? -1 : runs[index-1].id}
        }
        ReceiverCellControls { id:selection;theme:dialog.theme;visible:!dialog.decoderInput;showView:false;Layout.fillWidth:true;onSelectionChanged:dialog.refresh() }
        ContextualStatus { theme:dialog.theme;diagnostic:dialog.diagnosticState.diagnostic || null;onHelpRequested:function(code){dialog.contextHelpRequested(code)};visible:text.length>0;text:!dialog.diagnosticState.available ? dialog.diagnosticState.reason || "No data for this selection." : dialog.diagnosticState.llr && dialog.diagnosticState.llr.count===0 ? "No LLR data for this signal." : "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11 }
        RowLayout {
            visible:tabs.currentIndex===1;Layout.fillWidth:true
            CheckBox {id:nearZero;objectName:"llrNearZero";text:"Zoom near zero (−10 to +10)"}
            Item {Layout.fillWidth:true}
            Label {text:"Retained bits: "+((dialog.diagnosticState.confidence || {}).count || 0)+" · In bars: "+(((nearZero.checked ? (dialog.diagnosticState.confidence || {}).near : (dialog.diagnosticState.confidence || {}).full) || {}).shown || 0);color:dialog.theme.accent}
        }
        Label {
            visible:tabs.currentIndex===1;Layout.fillWidth:true;wrapMode:Text.WordWrap;color:dialog.theme.muted
            text:"At/beyond −100: "+((dialog.diagnosticState.confidence || {}).negative_limit || 0)+"   |   At/beyond +100: "+((dialog.diagnosticState.confidence || {}).positive_limit || 0)+"   · Separate from bars. Original values beyond the demapper limit are not retained."
        }
        RowLayout {
            visible:tabs.currentIndex===0;Layout.fillWidth:true
            Label {text:"Retained cell";color:dialog.theme.text}
            ComboBox {objectName:"slicerCell";Layout.fillWidth:true;model:(dialog.diagnosticState.rows || []).map(function(p){return "RX "+p.symbol+" · bin "+p.bin+" · "+p.order+" QAM"});currentIndex:dialog.selectedCell;onActivated:function(index){dialog.selectedCell=index}}
            Label {text:"Zoom";color:dialog.theme.text}
            Slider {id:symbolZoom;from:0.25;to:4;value:1;Layout.preferredWidth:160}
        }
        SoftDiagnosticPlot {
            objectName:"softDiagnosticPlot";theme:dialog.theme;mode:tabs.currentIndex
            observations:dialog.diagnosticState.confidence || ({})
            cell:(dialog.diagnosticState.rows || [])[dialog.selectedCell] || ({})
            nearZero:nearZero.checked;zoom:symbolZoom.value
            Layout.fillWidth:true;Layout.fillHeight:true;Layout.minimumHeight:140
        }
        Label {
            Layout.fillWidth:true;wrapMode:Text.WordWrap;color:dialog.theme.muted
            text:tabs.currentIndex===0 ? "+ Ideal symbols · Teal: received symbol and error vector · Yellow: selected symbol decision region (viewport-clipped). Physical labels precede descrambling."
                : tabs.currentIndex===1 ? "LLR = log[P(0)/P(1)]. Near zero means uncertain; large magnitude means confident, not necessarily correct. Bars show retained bits, not BER."
                : "Actual retained decoder inputs, grouped into min/max ranges to preserve extremes. This is a separate codeword snapshot; cell filters do not apply."
        }
        TextArea {
            objectName:"receiverDecisionDetails";visible:tabs.currentIndex===0;readOnly:true;selectByMouse:true;wrapMode:TextEdit.Wrap
            Layout.fillWidth:true;Layout.preferredHeight:90;color:dialog.theme.text
            text:{const p=(dialog.diagnosticState.rows || [])[dialog.selectedCell];return p ? "Physical label "+p.label+" · I/Q "+p.iq.map(function(v){return Number(v).toPrecision(5)}).join(", ")+" · residual² "+Number(p.distance_squared).toExponential(3)+"\nPost-descrambling LLRs: "+(p.llrs.length ? p.llrs.map(function(v){return Number(v).toPrecision(4)}).join("  ") : "Not retained for this stream") : "No retained cells."}
        }
    }
}
