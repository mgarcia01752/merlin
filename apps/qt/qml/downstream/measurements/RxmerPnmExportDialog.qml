/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../shared/components"
import "../../shared/presentation"
WorkbenchDialog {
    id:root
    objectName:"rxmerPnmExportDialog"
    required property var backend
    readonly property string documentSnapshot: presentation.value ? presentation.value.documentSnapshot : ""
    readonly property int captureCount: presentation.value ? presentation.value.captureCount : 1
    readonly property int previewCapture: presentation.value ? presentation.value.previewCapture : 1
    property bool busy:false
    property bool saving:false
    property bool dirty:false
    property bool canExport:false
    property bool canJson:false
    property int completed:0
    readonly property var summary: presentation.value ? presentation.value.summary : ({})
    readonly property string message: presentation.value ? presentation.value.message : ""
    readonly property var diagnostic: presentation.value ? presentation.value.diagnostic : null
    property bool saved: false
    readonly property string savedWithoutDetailsMessage: "Export saved; detail display unavailable."
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: root.theme
        initialValue: ({documentSnapshot: "", captureCount: 1, previewCapture: 1,
            summary: {}, message: "", diagnostic: null, savedPath: "",scopes:["Preview capture","All captures","Capture range"],summaryText:""})
    }
    Component.onCompleted: Qt.callLater(root.admitTextFields)
    Component.onDestruction: {
        if (presentation) {
            presentation.clear()
        }
    }
    function admitTextFields() {
        return presentation.reserveTextFields([first, last])
    }
    function showSnapshot(document, captures, preview) {
        if (!admitTextFields() || !presentation.replace({documentSnapshot: document, captureCount: captures,
                previewCapture: preview, summary: {}, message: "", diagnostic: null, savedPath: "",scopes:["Preview capture","All captures","Capture range"],summaryText:""})) {
            return false
        }
        return openChecked()
    }
    function replaceDetails(nextSummary, nextDiagnostic, nextMessage, nextPath) {
        if (!presentation.value) {
            return false
        }
        return presentation.replace({documentSnapshot: documentSnapshot, captureCount: captureCount,
            previewCapture: previewCapture, summary: nextSummary, diagnostic: nextDiagnostic,
            message: nextMessage, savedPath: nextPath,scopes:["Preview capture","All captures","Capture range"],
            summaryText:(nextSummary.captures || 0)+" capture(s) · "+(nextSummary.samples || 0)+" samples · "+(nextSummary.missing || 0)+" missing\n"+
                (nextSummary.rounded || 0)+" rounded · "+(nextSummary.clipped || 0)+" clipped · maximum change "+Number(nextSummary.maximum_change_db || 0).toFixed(3)+" dB"})
    }
    signal contextHelpRequested(string code)
    readonly property string savedPath: presentation.value ? presentation.value.savedPath : ""
    title:"Save RxMER"
    parent:Overlay.overlay;anchors.centerIn:parent;modal:true
    width:Math.min(650,parent ? parent.width-30:650)
    function selection() {
        return JSON.stringify({first:scope.currentIndex===0 ? previewCapture:scope.currentIndex===1 ? 1:Number(first.text),
            last:scope.currentIndex===0 ? previewCapture:scope.currentIndex===1 ? captureCount:Number(last.text),
            collection:scope.currentIndex!==0,clamp:clamp.checked})
    }
    function queueCheck() {
        canExport = false
        canJson = false
        dirty = true
        saved = false
        if (presentation.value) {
            replaceDetails(summary, diagnostic, message, "")
        }
        if (visible && presentation.textAdmitted) {
            debounce.restart()
        }
    }
    function check() {
        if (!visible || busy || !presentation.textAdmitted) {
            return
        }
        dirty = false
        canExport = false
        canJson = false
        completed = 0
        replaceDetails(summary, diagnostic, "", savedPath)
        const state = JSON.parse(backend.designer_export_prepare(documentSnapshot,selection()))
        busy = !!state.running
        saving = false
        if (!busy) {
            replaceDetails(summary, diagnostic, state.error || "", savedPath)
        }
    }
    function saveFormat(format) {
        if (!presentation.textAdmitted) {
            return
        }
        canExport = false
        canJson = false
        completed = 0
        saving = true
        const state = JSON.parse(backend.designer_export_save(format))
        busy = !!state.running
        replaceDetails(summary, diagnostic, state.error || "", savedPath)
        if (!busy) {
            queueCheck()
        }
    }
    onOpened: {
        if (admitTextFields()) {
            last.text = String(captureCount)
            queueCheck()
        }
    }
    onClosed:{canExport=false;dirty=false;backend.designer_export_cancel()}
    Timer {id:debounce;interval:180;onTriggered:root.check()}
    Timer {
        interval:80;repeat:true;running:root.busy
        onTriggered:{
            const state=JSON.parse(root.backend.designer_export_poll())
            root.completed=state.completed || root.completed
            if (!state.running) {
                root.busy = false
                root.saved = !!state.saved
                if (!root.visible) {return}
                if (root.dirty) {debounce.restart();return}
                const nextMessage = state.saved ? "Saved data and provenance: "+state.path : state.error || (state.cancelled ? "Export cancelled.":"")
                const accepted = root.replaceDetails(state.summary || state.json_summary || ({}), state.diagnostic || null,
                    nextMessage, state.saved ? state.path : root.savedPath)
                root.canJson = accepted && !!state.can_json
                root.canExport = accepted && !!state.can_export
            }
        }
    }
    contentItem:ColumnLayout {
        spacing:12
        Label {text:"Choose a format, then a destination in the file manager.";color:root.theme.text;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        GridLayout {
            columns:2;Layout.fillWidth:true;enabled:!root.saving || !root.busy
            Label {text:"Captures";color:root.theme.text}
            ComboBox {id:scope;objectName:"pnmExportScope";model:presentation.value ? presentation.value.scopes : [];Layout.fillWidth:true;onCurrentIndexChanged:root.queueCheck()}
            Label {visible:scope.currentIndex===2;text:"First / last";color:root.theme.text}
            RowLayout {visible:scope.currentIndex===2
                TextField {id:first;objectName:"pnmExportFirst";text:presentation.textAdmitted ? "1" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:75;onTextChanged:root.queueCheck();Accessible.name:"First exported capture"}
                TextField {id:last;objectName:"pnmExportLast";text:presentation.textAdmitted ? "1" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:75;onTextChanged:root.queueCheck();Accessible.name:"Last exported capture"}
            }
        }
        CheckBox {id:clamp;objectName:"pnmExportClamp";text:"Clamp to PNM range";enabled:!root.busy || !root.saving;onToggled:root.queueCheck()
            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus;ToolTip.text:"Off: values outside 0–63.5 dB block export. On: Rust explicitly clamps and reports changed samples. Nonfinite values always fail."}
        Label {text:"PNM binary: 0–63.5 dB, rounded to the nearest 0.25 dB (half steps upward). JSON: full precision, without rounding or clamping. Both preserve missing readings, the full grid and capture times.";color:root.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label {objectName:"pnmExportSummary";visible:!presentation.rejected && (root.canJson || root.canExport || root.saved)
            text:presentation.value ? presentation.value.summaryText : ""
            color:root.theme.text;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label {text:"A MERLIN provenance JSON accompanies each export. A collection is saved in a new folder with one file per capture. Existing files are never overwritten.";color:root.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        ProgressBar {visible:root.busy;indeterminate:root.saving;from:0;to:Math.max(1,root.captureCount);value:root.completed;Layout.fillWidth:true}
        ContextualStatus {theme:root.theme;diagnostic:presentation.rejected ? null : root.diagnostic;onHelpRequested:function(code){root.contextHelpRequested(code)};objectName:"pnmExportStatus";text:root.busy ? (root.saving ? "Choose a destination, then export…":"Checking capture data…") : presentation.rejected ? (root.saved ? root.savedWithoutDetailsMessage : presentation.message) : root.message
            color:root.canExport || root.saved ? root.theme.accent:root.theme.statusError;visible:text.length>0;Layout.fillWidth:true;wrapMode:Text.WordWrap}
    }
    footer:Pane {
        padding:10;background:Item {}
        contentItem:RowLayout {spacing:10
            Item {Layout.fillWidth:true}
            WorkbenchActionButton {theme:root.theme;objectName:"pnmExportSave";Layout.preferredWidth:120;text:"PNM binary";primary:true;enabled:presentation.textAdmitted && !presentation.rejected && root.canExport && !root.busy && !root.dirty
                onClicked:root.saveFormat("pnm")}
            WorkbenchActionButton {theme:root.theme;objectName:"jsonExportSave";Layout.preferredWidth:120;text:"JSON";enabled:presentation.textAdmitted && !presentation.rejected && root.canJson && !root.busy && !root.dirty
                onClicked:root.saveFormat("json")}
            WorkbenchActionButton {theme:root.theme;objectName:"pnmExportClose";Layout.preferredWidth:120;text:root.saved ? "Close":"Cancel";onClicked:root.close()}
        }
    }
}
