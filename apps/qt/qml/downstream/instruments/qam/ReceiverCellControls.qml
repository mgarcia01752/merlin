/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"
import "ReceiverSelectionLimits.js" as ReceiverSelectionLimits
ColumnLayout {
    id: controls
    required property var theme
    property var metadata: ({})
    readonly property var displayedMetadata: presentation.value ? presentation.value.metadata : ({})
    readonly property var selectorLabels: presentation.value ? presentation.value.labels : ({})
    readonly property bool contextCurrent: presentation.value && displayedMetadata === metadata
    PresentationOwner { id:presentation; theme:controls.theme }
    /**
     * Admit selector projections before ComboBox converts their plain arrays.
     * @returns False preserves previous metadata/labels and disables stale actions.
     */
    function refreshPresentation() {
        const value = metadata
        const labels = {
            signals:["Data Profile","NCP","PLC","Continuous Pilots","Scattered Pilots"],
            views:["Raw","Equalized","Paired"],parts:["Data","Preamble"],
            runs:[value.latest_run_profile===undefined || value.latest_run_profile===null ? "Latest Run" : "Latest Run · Profile "+value.latest_run_profile].concat((value.runs || []).map(function(run){return "Profile "+run.id+(run.available ? "" : " · No Result")})),
            profiles:["All received"].concat((value.profiles || []).map(function(id){return "Profile "+id})),
            orders:(value.orders || []).map(function(order){return order+" QAM"}),
            summary:value.retained ? "Available: "+value.retained+" cells · bins "+value.first_bin+"–"+value.last_bin+" · RX symbols "+value.first_symbol+"–"+value.last_symbol : "No data for this signal"
        }
        return presentation.replace({metadata:value,labels:labels})
    }
    /** Admit the actual existing editor capacities; no validator limit changes. */
    function prepareText() {
        return presentation.reserveTextFields([firstBinField.contentItem,lastBinField.contentItem,firstSymbolField.contentItem,lastSymbolField.contentItem])
    }
    onMetadataChanged: refreshPresentation()
    Component.onCompleted: {prepareText();refreshPresentation()}
    property int signalIndex: 0
    property int viewIndex: 2
    property int plcPartIndex: 0
    property int runProfile: -1
    property int profile: -1
    property int order: 0
    property int firstBin: ReceiverSelectionLimits.ALL_PHYSICAL_BINS
    property int lastBin: ReceiverSelectionLimits.ALL_PHYSICAL_BINS
    property int firstSymbol: ReceiverSelectionLimits.ALL_RECEIVER_SYMBOLS
    property int lastSymbol: ReceiverSelectionLimits.ALL_RECEIVER_SYMBOLS
    property bool showView: true
    property bool resetting: false
    signal selectionChanged()
    function query() { return {run_profile:runProfile,signal:signalIndex,view:viewIndex,part:plcPartIndex,profile:profile,order:order,first_bin:firstBin,last_bin:lastBin,first_symbol:firstSymbol,last_symbol:lastSymbol} }
    function resetFilters(resetRun) {
        resetting=true;if(resetRun!==false){runProfile=-1};profile=-1;order=0;firstBin=ReceiverSelectionLimits.ALL_PHYSICAL_BINS;lastBin=ReceiverSelectionLimits.ALL_PHYSICAL_BINS;firstSymbol=ReceiverSelectionLimits.ALL_RECEIVER_SYMBOLS;lastSymbol=ReceiverSelectionLimits.ALL_RECEIVER_SYMBOLS;resetting=false
        selectionChanged()
    }
    onRunProfileChanged: if(!resetting) resetFilters(false)
    onSignalIndexChanged: resetFilters(false)
    onPlcPartIndexChanged: resetFilters(false)
    onViewIndexChanged: { if (!resetting) { selectionChanged() } }
    Label { visible:presentation.rejected;text:presentation.message;color:controls.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap }
    spacing: 8
    RowLayout {
        Layout.fillWidth:true
        Label { text:"Signal";color:controls.theme.text }
        ComboBox { objectName:"receiverQamSignal";model:controls.selectorLabels.signals || [];enabled:controls.contextCurrent;currentIndex:controls.signalIndex;Layout.preferredWidth:180;onActivated:function(index){controls.signalIndex=index} }
        Label { visible:controls.showView;text:"QAM View";color:controls.theme.text }
        ComboBox { objectName:"receiverQamView";visible:controls.showView;model:controls.selectorLabels.views || [];enabled:controls.contextCurrent;currentIndex:controls.viewIndex;Layout.preferredWidth:135;onActivated:function(index){controls.viewIndex=index} }
        Label { visible:controls.signalIndex===2;text:"PLC Part";color:controls.theme.text }
        ComboBox { objectName:"receiverPlcPart";visible:controls.signalIndex===2;model:controls.selectorLabels.parts || [];enabled:controls.contextCurrent;currentIndex:controls.plcPartIndex;onActivated:function(index){controls.plcPartIndex=index} }
        Item { Layout.fillWidth:true }
        WorkbenchActionButton { theme: controls.theme; text:"Reset Filters";onClicked:controls.resetFilters() }
    }
    RowLayout {
        Layout.fillWidth:true
        Label {text:"Run Profile";color:controls.theme.muted}
        ComboBox {
            objectName:"receiverRunProfile"
            readonly property var runs:controls.displayedMetadata.runs || []
            model:controls.selectorLabels.runs || [];enabled:controls.contextCurrent
            currentIndex:controls.runProfile<0 ? 0 : runs.findIndex(function(run){return run.id===controls.runProfile})+1
            Layout.preferredWidth:190
            onActivated:function(index){controls.runProfile=index===0 ? -1 : runs[index-1].id}
            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus
            ToolTip.text:"Captured profile ID from a separate simulation run. Select profiles in Configure WorkBench before running them."
        }
        Label { visible:controls.signalIndex===0 && (controls.displayedMetadata.profiles || []).length>1;text:"Wire Profile";color:controls.theme.muted }
        ComboBox {
            objectName:"receiverProfile";visible:controls.signalIndex===0 && (controls.displayedMetadata.profiles || []).length>1
            model:controls.selectorLabels.profiles || []
            enabled:controls.contextCurrent && (controls.displayedMetadata.profiles || []).length>0
            currentIndex:controls.profile<0 ? 0 : (controls.displayedMetadata.profiles || []).indexOf(controls.profile)+1
            Layout.preferredWidth:160
            onActivated:function(index){controls.profile=index===0 ? -1 : controls.displayedMetadata.profiles[index-1];controls.order=0;controls.selectionChanged()}
        }
        Label { text:"Modulation";color:controls.theme.muted }
        ComboBox {
            objectName:"receiverModulation";model:controls.selectorLabels.orders || []
            displayText:(controls.displayedMetadata.orders || []).length ? currentText : "No available group"
            currentIndex:controls.order===0 ? 0 : (controls.displayedMetadata.orders || []).indexOf(controls.order)
            Layout.preferredWidth:170;enabled:controls.contextCurrent && (controls.displayedMetadata.orders || []).length>0
            onActivated:function(index){controls.order=controls.displayedMetadata.orders[index];controls.selectionChanged()}
        }
        Item { Layout.fillWidth:true }
    }
    GridLayout {
        columns:8;columnSpacing:8;rowSpacing:4;Layout.fillWidth:true
        Label { text:"First Bin";color:controls.theme.muted }
        SpinBox { id:firstBinField;objectName:"receiverFirstBin";from:ReceiverSelectionLimits.ALL_PHYSICAL_BINS;to:ReceiverSelectionLimits.MAXIMUM_PHYSICAL_BIN_HINT;enabled:presentation.textAdmitted && controls.contextCurrent;editable:presentation.textAdmitted;value:controls.firstBin;textFromValue:presentation.textAdmitted ? function(v){return v<0 ? "All" : v.toString()} : function(v){return ""};valueFromText:function(t){return t==="All" ? ReceiverSelectionLimits.ALL_PHYSICAL_BINS : Number(t)};Layout.preferredWidth:105;onValueModified:{controls.firstBin=value;controls.selectionChanged()} }
        Label { text:"Last Bin";color:controls.theme.muted }
        SpinBox { id:lastBinField;objectName:"receiverLastBin";from:ReceiverSelectionLimits.ALL_PHYSICAL_BINS;to:ReceiverSelectionLimits.MAXIMUM_PHYSICAL_BIN_HINT;enabled:presentation.textAdmitted && controls.contextCurrent;editable:presentation.textAdmitted;value:controls.lastBin;textFromValue:presentation.textAdmitted ? function(v){return v<0 ? "All" : v.toString()} : function(v){return ""};valueFromText:function(t){return t==="All" ? ReceiverSelectionLimits.ALL_PHYSICAL_BINS : Number(t)};Layout.preferredWidth:105;onValueModified:{controls.lastBin=value;controls.selectionChanged()} }
        Label { text:"First RX Symbol";color:controls.theme.muted }
        SpinBox { id:firstSymbolField;objectName:"receiverFirstSymbol";from:ReceiverSelectionLimits.ALL_RECEIVER_SYMBOLS;to:ReceiverSelectionLimits.MAXIMUM_RX_SYMBOL_HINT;enabled:presentation.textAdmitted && controls.contextCurrent;editable:presentation.textAdmitted;value:controls.firstSymbol;textFromValue:presentation.textAdmitted ? function(v){return v<0 ? "All" : v.toString()} : function(v){return ""};valueFromText:function(t){return t==="All" ? ReceiverSelectionLimits.ALL_RECEIVER_SYMBOLS : Number(t)};Layout.preferredWidth:115;onValueModified:{controls.firstSymbol=value;controls.selectionChanged()} }
        Label { text:"Last RX Symbol";color:controls.theme.muted }
        SpinBox { id:lastSymbolField;objectName:"receiverLastSymbol";from:ReceiverSelectionLimits.ALL_RECEIVER_SYMBOLS;to:ReceiverSelectionLimits.MAXIMUM_RX_SYMBOL_HINT;enabled:presentation.textAdmitted && controls.contextCurrent;editable:presentation.textAdmitted;value:controls.lastSymbol;textFromValue:presentation.textAdmitted ? function(v){return v<0 ? "All" : v.toString()} : function(v){return ""};valueFromText:function(t){return t==="All" ? ReceiverSelectionLimits.ALL_RECEIVER_SYMBOLS : Number(t)};Layout.preferredWidth:115;onValueModified:{controls.lastSymbol=value;controls.selectionChanged()} }
    }
    Label { text:controls.selectorLabels.summary || "";color:controls.theme.muted;font.pixelSize:11;Layout.fillWidth:true;wrapMode:Text.WordWrap }
}
