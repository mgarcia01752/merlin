/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"
import "../qam"
import "../spectrum"
WorkbenchDialog {
    id:dialog
    signal contextHelpRequested(string code)
    required property var backend
    property var runState: ({})
    readonly property string seenToken:presentation.value ? presentation.value.token : ""
    readonly property bool decoderInput: tabs.currentIndex === 2
    property int selectedCell: 0
    property bool updating:false
    readonly property var diagnosticState:presentation.value ? presentation.value.diagnosticState : ({})
    readonly property var display:presentation.value ? presentation.value.display : ({})
    property PresentationOwner presentationCoverage: PresentationOwner {
        id:presentation
        theme:dialog.theme
    }

    /**
     * Couple both original queries with all independently retained display text.
     * @param value Original slicer DTO, including unchanged confidence/cell data.
     * @param metadata Original optional QAM selector metadata response.
     * @param query Exact receiver selection used for both calls.
     * @param mode Existing symbol/confidence/decoder tab index.
     * @returns Plain candidate admitted before replacing either response.
     */
    function slicerCandidate(value, metadata, query, mode) {
        const confidence = value.confidence || {}
        return {diagnosticState:value,metadata:metadata,query:query,mode:mode,token:String(runState.result_token || ""),
            cellLabels:(value.rows || []).map(function(p){return "RX "+p.symbol+" · bin "+p.bin+" · "+p.order+" QAM"}),
            decisionDetails:(value.rows || []).map(function(p){return "Physical label "+p.label+" · I/Q "+p.iq.map(function(v){return Number(v).toPrecision(5)}).join(", ")+" · residual² "+Number(p.distance_squared).toExponential(3)+"\nPost-descrambling LLRs: "+(p.llrs.length ? p.llrs.map(function(v){return Number(v).toPrecision(4)}).join("  ") : "Not retained for this stream")}),
            decoderLabels:["Latest Run"].concat((metadata.runs || []).map(function(p){return "Profile "+p.id})),
            display:{fullBits:"Retained bits: "+(confidence.count || 0)+" · In bars: "+((confidence.full || {}).shown || 0),
                nearBits:"Retained bits: "+(confidence.count || 0)+" · In bars: "+((confidence.near || {}).shown || 0),
                limits:"At/beyond −100: "+(confidence.negative_limit || 0)+"   |   At/beyond +100: "+(confidence.positive_limit || 0)+"   · Separate from bars. Original values beyond the demapper limit are not retained."}}
    }

    /**
     * Restore a complete accepted query without dispatching intermediate states.
     * @param query Existing scalar ReceiverCellControls selection.
     * @returns No result; this helper does not query or mutate numerical data.
     */
    function restoreSelection(query) {
        updating = true
        selection.resetting = true
        selection.runProfile = query.run_profile===undefined ? -1 : query.run_profile
        selection.signalIndex = query.signal
        selection.viewIndex = query.view
        selection.plcPartIndex = query.part
        selection.profile = query.profile
        selection.order = query.order
        selection.firstBin = query.first_bin
        selection.lastBin = query.last_bin
        selection.firstSymbol = query.first_symbol
        selection.lastSymbol = query.last_symbol
        selection.resetting = false
        updating = false
    }

    /**
     * Admit original slicer and metadata responses as one coherent selection.
     * @returns False preserves old data/tab/selected cell and restores filters.
     */
    function refresh() {
        if (!backend || !selection || updating) {
            return false
        }
        const query = selection.query()
        const mode = tabs.currentIndex
        const value = JSON.parse(backend.slicer(JSON.stringify(query),mode===2))
        const metadata = backend.qam_selected ? JSON.parse(backend.qam_selected(JSON.stringify(query))) : (presentation.value ? presentation.value.metadata : ({}))
        if (!presentation.replace(slicerCandidate(value,metadata,query,mode))) {
            if (presentation.value) {
                restoreSelection(presentation.value.query)
                updating = true
                tabs.currentIndex = presentation.value.mode
                updating = false
            }
            return false
        }
        selectedCell = 0
        return true
    }

    /**
     * Apply the existing external selection request, then admit its response.
     * @param query Current QAM selection; omitted run_profile retains Latest Run.
     * @returns Result of refreshing the complete requested selection.
     */
    function useSelection(query) {
        const complete = {run_profile:query.run_profile===undefined ? -1 : query.run_profile,
            signal:query.signal,view:selection.viewIndex,part:query.part,profile:query.profile,order:query.order,
            first_bin:query.first_bin,last_bin:query.last_bin,first_symbol:query.first_symbol,last_symbol:query.last_symbol}
        restoreSelection(complete)
        return refresh()
    }

    /**
     * Keep the tab context admitted without adding a symbol/confidence query.
     * @returns False restores the prior tab; decoder transitions use refresh.
     */
    function selectMode() {
        const accepted = presentation.value
        const mode = tabs.currentIndex
        if (!accepted || (mode===2)!==(accepted.mode===2)) {
            return refresh()
        }
        const candidate = slicerCandidate(accepted.diagnosticState,accepted.metadata,accepted.query,mode)
        candidate.token = accepted.token
        if (!presentation.replace(candidate)) {
            updating = true
            tabs.currentIndex = accepted.mode
            updating = false
            return false
        }
        return true
    }
    Component.onCompleted: presentation.replace(slicerCandidate({rows:[],histogram:{points:[],x_ticks:[],y_ticks:[]}},({}),selection.query(),tabs.currentIndex))
    onOpened:refresh()
    onRunStateChanged:{if (opened && String(runState.result_token || "")!==seenToken){refresh()} }
    title:"Slicer / LLR"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1120,parent ? parent.width-40 : 1120);height:Math.min(720,parent ? parent.height-40 : 720)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        spacing:8
        TabBar {
            id:tabs;objectName:"receiverSoftTabs";Layout.fillWidth:true;currentIndex:1;onCurrentIndexChanged:if(dialog.opened && !dialog.updating){dialog.selectMode()}
            TabButton {text:"Symbol Decisions"}
            TabButton {text:"Bit Confidence"}
            TabButton {text:"Decoder Input"}
        }
        Label {objectName:"receiverSlicerRetention";visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label {text:dialog.diagnosticState.detail || "";color:dialog.theme.muted;wrapMode:Text.WordWrap;Layout.fillWidth:true}
        ComboBox {
            visible:dialog.decoderInput;objectName:"decoderRunProfile"
            readonly property var runs:selection.metadata.runs || []
            model:presentation.value ? presentation.value.decoderLabels : []
            currentIndex:selection.runProfile<0 ? 0 : runs.findIndex(function(p){return p.id===selection.runProfile})+1
            onActivated:function(index){selection.runProfile=index===0 ? -1 : runs[index-1].id}
        }
        ReceiverCellControls { id:selection;theme:dialog.theme;metadata:presentation.value ? presentation.value.metadata : ({});visible:!dialog.decoderInput;showView:false;Layout.fillWidth:true;onSelectionChanged:dialog.refresh() }
        ContextualStatus { theme:dialog.theme;diagnostic:dialog.diagnosticState.diagnostic || null;onHelpRequested:function(code){dialog.contextHelpRequested(code)};visible:text.length>0;text:!dialog.diagnosticState.available ? dialog.diagnosticState.reason || "No data for this selection." : dialog.diagnosticState.llr && dialog.diagnosticState.llr.count===0 ? "No LLR data for this signal." : "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11 }
        RowLayout {
            visible:tabs.currentIndex===1;Layout.fillWidth:true
            CheckBox {id:nearZero;objectName:"llrNearZero";text:"Zoom near zero (−10 to +10)"}
            Item {Layout.fillWidth:true}
            Label {text:(nearZero.checked ? dialog.display.nearBits : dialog.display.fullBits) || "";color:dialog.theme.accent}
        }
        Label {
            visible:tabs.currentIndex===1;Layout.fillWidth:true;wrapMode:Text.WordWrap;color:dialog.theme.muted
            text:dialog.display.limits || ""
        }
        RowLayout {
            visible:tabs.currentIndex===0;Layout.fillWidth:true
            Label {text:"Retained cell";color:dialog.theme.text}
            ComboBox {objectName:"slicerCell";Layout.fillWidth:true;model:presentation.value ? presentation.value.cellLabels : [];currentIndex:dialog.selectedCell;onActivated:function(index){dialog.selectedCell=index}}
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
            text:presentation.value ? presentation.value.decisionDetails[dialog.selectedCell] || "No retained cells." : "No retained cells."
        }
    }
}
