/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"
import "../spectrum"
WorkbenchDialog {
    id:dialog
    required property var backend
    property var runState: ({})
    readonly property var response: presentation.value ? presentation.value.response : ({})
    readonly property var display: presentation.value ? presentation.value.display : ({})
    // The accepted result and its labels stay independent of next-run settings.
    readonly property bool mceResult: presentation.value ? presentation.value.mceResult : false
    readonly property string estimateLabel: display.estimateLabel || "Pilot Estimate (Ĥ)"
    readonly property string seenToken: presentation.value ? presentation.value.token : ""
    property int sourceIndex:0
    property int snapshotIndex:0
    property int viewIndex:0
    property int velocityPercent:87
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: dialog.responseCandidate({points:[],x_ticks:[],y_ticks:[]},{source:0,snapshot:0,view:0,velocity_percent:87})
    }

    /**
     * Couple the original response, exact query and independently retained labels.
     * @param value Original response/comparison points; no numerical rebuilding.
     * @param query Original response query including velocity factor in percent.
     * @returns Plain candidate admitted before selectors/plots change together.
     */
    function responseCandidate(value, query) {
        const estimated = value.available ? value.mce!==undefined : runState.options?.receiver_mode==="merlin_estimated"
        const label = estimated ? "MCE Estimate (Ĥ)" : "Pilot Estimate (Ĥ)"
        return {response:value,query:query,mceResult:estimated,receiverMode:runState.options?.receiver_mode || "",token:String(runState.result_token || ""),
            comparison:value.comparison || {points:[],x_ticks:[],y_ticks:[]},
            sources:["Applied Channel (H)",label,estimated ? "Compare Channel / MCE" : "Compare Channel / Pilots"],
            views:["Gain","Wrapped Phase","Real","Imaginary","Impulse / Echo"],
            snapshots:(value.snapshots || []).map(function(symbol){return symbol===undefined || symbol===null ? "" : String(symbol)}),
            display:{estimateLabel:label,comparisonTitle:label+" · independent vertical scale",
                mce:value.mce ? "Latest MCE: "+(value.mce.tracking ? "Tracking" : "Training")+" · raw pilot noise estimate: "+value.mce.noise_text : ""}}
    }

    /**
     * Admit actual TextInput capacity before enabling the existing VF editor.
     * @returns False leaves dynamic text and editing disabled; open retries.
     */
    function prepareVelocity() {
        return presentation.reserveTextFields([velocitySpin.contentItem])
    }

    /**
     * Query and admit response/context atomically; rejection restores selectors.
     * @returns True after accepted replacement, false without changing old data.
     */
    function refresh() {
        if (!backend) {
            return false
        }
        const query = {source:sourceIndex,snapshot:snapshotIndex,view:viewIndex,velocity_percent:velocityPercent}
        const value = JSON.parse(backend.response(JSON.stringify(query)))
        if (!presentation.replace(responseCandidate(value,query))) {
            if (presentation.value) {
                const accepted = presentation.value.query
                sourceIndex = accepted.source
                snapshotIndex = accepted.snapshot
                viewIndex = accepted.view
                velocityPercent = accepted.velocity_percent
            }
            return false
        }
        return true
    }
    Component.onCompleted: prepareVelocity()
    onOpened:{prepareVelocity();refresh()}
    onRunStateChanged: {
        if (opened && String(runState.result_token || "")!==seenToken) {
            refresh()
        } else if (presentation && presentation.value && !response.available && (runState.options?.receiver_mode || "")!==presentation.value.receiverMode) {
            // Missing-result labels still follow settings without a new query.
            const accepted = presentation.value
            const candidate = responseCandidate(accepted.response,accepted.query)
            candidate.token = accepted.token
            presentation.replace(candidate)
        }
    }
    title:"Channel Response / Echo"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1120,parent ? parent.width-40 : 1120);height:Math.min(690,parent ? parent.height-40 : 690)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        spacing:10
        RowLayout {
            Label { text:"Display";color:dialog.theme.text }
            ComboBox { objectName:"receiverResponseSource";model:presentation.value ? presentation.value.sources : [];currentIndex:dialog.sourceIndex;Layout.preferredWidth:240;onActivated:function(index){dialog.sourceIndex=index;dialog.snapshotIndex=0;dialog.refresh()} }
            Label { text:"View";color:dialog.theme.text }
            ComboBox { objectName:"receiverResponseView";model:presentation.value ? presentation.value.views : [];currentIndex:dialog.viewIndex;Layout.preferredWidth:165;onActivated:function(index){dialog.viewIndex=index;dialog.refresh()} }
            Label { visible:dialog.viewIndex===4;text:"VF (%)";color:dialog.theme.text }
            SpinBox { id:velocitySpin;visible:dialog.viewIndex===4;objectName:"receiverVelocity";from:1;to:100;enabled:presentation.textAdmitted;editable:presentation.textAdmitted;textFromValue:presentation.textAdmitted ? function(value,locale){return Number(value).toLocaleString(locale,"f",0)} : function(value,locale){return ""};value:dialog.velocityPercent;onValueModified:{dialog.velocityPercent=value;dialog.refresh()} }
            Item { Layout.fillWidth:true }
        }
        RowLayout {
            visible:dialog.sourceIndex>0
            Label { text:"RX Symbol";color:dialog.theme.muted }
            ComboBox { objectName:"receiverResponseSnapshot";model:presentation.value ? presentation.value.snapshots : [];currentIndex:dialog.snapshotIndex;enabled:model.length>0;onActivated:function(index){dialog.snapshotIndex=index;dialog.refresh()} }
        }
        Label {objectName:"receiverResponseRetention";visible:presentation.rejected;text:presentation.message;color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { text:dialog.response.summary || dialog.response.reason || "";color:dialog.theme.accent;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        RowLayout {
            Layout.fillWidth:true;Layout.fillHeight:true;spacing:16
            ColumnLayout {
                Layout.fillWidth:true;Layout.fillHeight:true;Layout.preferredWidth:1;Layout.minimumWidth:0
                Label {visible:dialog.sourceIndex===2;text:"Applied Channel (H)";color:dialog.theme.text}
                Loader {active:presentation.value!==null;Layout.fillWidth:true;Layout.fillHeight:true;sourceComponent:Component {SpectrumPlot {theme:dialog.theme;viewport:dialog.response}}}
            }
            ColumnLayout {
                visible:dialog.sourceIndex===2;Layout.fillWidth:true;Layout.fillHeight:true;Layout.preferredWidth:1;Layout.minimumWidth:0
                Label {text:dialog.display.comparisonTitle || "";color:dialog.theme.text;Layout.fillWidth:true;wrapMode:Text.WordWrap}
                Loader {active:presentation.value!==null;Layout.fillWidth:true;Layout.fillHeight:true;sourceComponent:Component {SpectrumPlot {objectName:"receiverResponseComparison";theme:dialog.theme;viewport:presentation.value.comparison}}}
            }
        }
        Label {visible:dialog.response.mce!==undefined;text:dialog.display.mce || "";color:dialog.theme.accent;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { text:dialog.response.detail || "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11 }
    }
}
