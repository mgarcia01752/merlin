/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"
WorkbenchDialog {
    id:dialog
    required property var backend
    signal contextHelpRequested(string code)
    property int tap:0
    // Request alias: the existing caller supplies exactly three compiled labels.
    // Displayed name/title belongs to the admitted context, even after rejection.
    property string tapName:"Before Amplifier"
    readonly property var observation: presentation.value ? presentation.value.observation : ({})
    readonly property int displayedTap: presentation.value ? presentation.value.tap : 0
    readonly property var display: presentation.value ? presentation.value.display : ({})
    property bool restoringSelection: false
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: dialog.tapCandidate({available:false,points:[]},0,"Before Amplifier",null)
    }

    /**
     * Couple the original response with its actual tap/query and display strings.
     * @param value Original backend DTO; points remain under this parent owner.
     * @param selectedTap Original requested tap.
     * @param name One of the caller's three fixed compiled tap labels.
     * @param query Existing ReceiverCellControls query, or null before first query.
     * @returns Plain candidate admitted before replacing displayed context.
     */
    function tapCandidate(value, selectedTap, name, query) {
        return {observation:value,tap:selectedTap,tapName:name,query:query,display:{
            title:"QAM Analyzer · "+name,
            shown:"Showing "+(value.points || []).length+" of "+value.retained+" retained cells.",
            axis:"Raw I/Q Axis: ±"+Number(value.extent || 0).toPrecision(4)+" normalized units"
        }}
    }

    /**
     * Restore only the existing scalar filters after rejected presentation.
     * @param query Previously admitted query; null has no captured selection.
     * @returns No result; callbacks cannot dispatch an intermediate restored query.
     */
    function restoreSelection(query) {
        if (!query) {
            return
        }
        restoringSelection = true
        selection.resetting = true
        selection.signalIndex = query.signal
        selection.viewIndex = query.view
        selection.plcPartIndex = query.part
        selection.runProfile = query.run_profile
        selection.profile = query.profile
        selection.order = query.order
        selection.firstBin = query.first_bin
        selection.lastBin = query.last_bin
        selection.firstSymbol = query.first_symbol
        selection.lastSymbol = query.last_symbol
        selection.resetting = false
        restoringSelection = false
    }

    /**
     * Admit the actual finite TextInput capacities before enabling bin editing.
     * @returns True for both existing fields; failure keeps their text/editing gated.
     */
    function prepareFilters() {
        return presentation.reserveTextFields([firstSpin.contentItem,lastSpin.contentItem])
    }

    /**
     * Query and admit one complete tap response without relabeling old samples.
     * @returns False keeps the old DTO/context and restores its scalar filters.
     */
    function refresh() {
        if (!backend || restoringSelection) {
            return false
        }
        const selectedTap = tap
        const name = tapName
        const query = selection.query()
        const value = JSON.parse(backend.tap_qam(selectedTap,JSON.stringify(query)))
        if (!presentation.replace(tapCandidate(value,selectedTap,name,query))) {
            if (presentation.value) {
                tap = presentation.value.tap
                tapName = presentation.value.tapName
                restoreSelection(presentation.value.query)
            }
            return false
        }
        return true
    }
    Component.onCompleted: prepareFilters()
    onOpened:{prepareFilters();refresh()}
    title:dialog.display.title || "QAM Analyzer"
    parent:Overlay.overlay;anchors.centerIn:parent;width:Math.min(1060,parent ? parent.width-40 : 1060);height:Math.min(700,parent ? parent.height-40 : 700)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        ReceiverCellControls {id:selection;visible:dialog.displayedTap===2;showView:false;viewIndex:0;theme:dialog.theme;metadata:dialog.observation;Layout.fillWidth:true;onSelectionChanged:if(dialog.opened && !dialog.restoringSelection){dialog.refresh()}}
        RowLayout {
            visible:dialog.displayedTap<2;Layout.fillWidth:true
            Label {text:"First Bin";color:dialog.theme.muted}
            SpinBox {id:firstSpin;objectName:"tapQamFirstBin";from:-1;to:8191;value:selection.firstBin;enabled:presentation.textAdmitted;editable:presentation.textAdmitted;textFromValue:presentation.textAdmitted ? function(v){return v<0 ? "All" : v.toString()} : function(v){return ""};valueFromText:function(t){return t==="All" ? -1 : Number(t)};onValueModified:{selection.firstBin=value;dialog.refresh()}}
            Label {text:"Last Bin";color:dialog.theme.muted}
            SpinBox {id:lastSpin;objectName:"tapQamLastBin";from:-1;to:8191;value:selection.lastBin;enabled:presentation.textAdmitted;editable:presentation.textAdmitted;textFromValue:presentation.textAdmitted ? function(v){return v<0 ? "All" : v.toString()} : function(v){return ""};valueFromText:function(t){return t==="All" ? -1 : Number(t)};onValueModified:{selection.lastBin=value;dialog.refresh()}}
            Item {Layout.fillWidth:true}
        }
        Label {objectName:"tapQamRetention";visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label {
            objectName:"tapQamMer"
            visible:dialog.displayedTap===2
            text:(dialog.observation.reference_mer || {}).text || "TX-reference MER unavailable"
            color:dialog.theme.accent
            HoverHandler {id:merHover}
            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:merHover.hovered
            ToolTip.text:(dialog.observation.reference_mer || {}).help || "Select retained cells to calculate MER."
        }
        Label {visible:dialog.displayedTap===2;text:(dialog.observation.mer || {}).text || "";color:dialog.theme.muted;font.pixelSize:11;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        TransmitterScatter {objectName:"tapQamScatter";theme:dialog.theme;observation:dialog.observation;receivedSamples:true;Layout.fillWidth:true;Layout.fillHeight:true}
        Label {objectName:"tapQamDisplayLimit";visible:dialog.displayedTap===2 && dialog.observation.available && dialog.observation.retained>dialog.observation.points.length;text:dialog.display.shown || "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:12}
        Label {visible:!!dialog.observation.extent;text:dialog.display.axis || "";color:dialog.theme.muted}
        ContextualStatus {theme:dialog.theme;diagnostic:dialog.observation.diagnostic || null;onHelpRequested:function(code){dialog.contextHelpRequested(code)};visible:!dialog.observation.available;text:dialog.observation.reason || "Run the simulation to retain this TapPoint.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
    }
}
