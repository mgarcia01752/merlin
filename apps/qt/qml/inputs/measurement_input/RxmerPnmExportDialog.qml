/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
WorkbenchDialog {
    id:root
    objectName:"rxmerPnmExportDialog"
    required property var backend
    property string documentSnapshot:""
    property int captureCount:1
    property int previewCapture:1
    property bool busy:false
    property bool saving:false
    property bool dirty:false
    property bool canExport:false
    property bool canJson:false
    property int completed:0
    property var summary:({})
    property string message:""
    property var diagnostic:null
    signal contextHelpRequested(string code)
    property string savedPath:""
    title:"Save RxMER"
    parent:Overlay.overlay;anchors.centerIn:parent;modal:true
    width:Math.min(650,parent ? parent.width-30:650)
    function selection() {
        return JSON.stringify({first:scope.currentIndex===0 ? previewCapture:scope.currentIndex===1 ? 1:Number(first.text),
            last:scope.currentIndex===0 ? previewCapture:scope.currentIndex===1 ? captureCount:Number(last.text),
            collection:scope.currentIndex!==0,clamp:clamp.checked})
    }
    function queueCheck() {
        canExport=false;canJson=false;dirty=true;savedPath=""
        if (visible) {debounce.restart()}
    }
    function check() {
        if (!visible || busy) {return}
        dirty=false;canExport=false;message="";completed=0
        const state=JSON.parse(backend.designer_export_prepare(documentSnapshot,selection()))
        busy=!!state.running;saving=false
        if (!busy) {message=state.error || ""}
    }
    function saveFormat(format) {
        canExport=false;canJson=false;completed=0;saving=true
        const state=JSON.parse(backend.designer_export_save(format))
        busy=!!state.running;message=state.error || ""
        if (!busy) {queueCheck()}
    }
    onOpened:{last.text=String(captureCount);queueCheck()}
    onClosed:{canExport=false;dirty=false;backend.designer_export_cancel()}
    Timer {id:debounce;interval:180;onTriggered:root.check()}
    Timer {
        interval:80;repeat:true;running:root.busy
        onTriggered:{
            const state=JSON.parse(root.backend.designer_export_poll())
            root.completed=state.completed || root.completed
            if (!state.running) {
                root.busy=false
                if (!root.visible) {return}
                if (root.dirty) {debounce.restart();return}
                root.diagnostic=state.diagnostic || null;root.canJson=!!state.can_json;root.canExport=!!state.can_export;root.summary=state.summary || state.json_summary || ({})
                root.message=state.error || (state.cancelled ? "Export cancelled.":"")
                if (state.saved) {root.savedPath=state.path;root.message="Saved data and provenance: "+state.path}
            }
        }
    }
    contentItem:ColumnLayout {
        spacing:12
        Label {text:"Choose a format, then a destination in the file manager.";color:root.theme.text;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        GridLayout {
            columns:2;Layout.fillWidth:true;enabled:!root.saving || !root.busy
            Label {text:"Captures";color:root.theme.text}
            ComboBox {id:scope;objectName:"pnmExportScope";model:["Preview capture","All captures","Capture range"];Layout.fillWidth:true;onCurrentIndexChanged:root.queueCheck()}
            Label {visible:scope.currentIndex===2;text:"First / last";color:root.theme.text}
            RowLayout {visible:scope.currentIndex===2
                TextField {id:first;objectName:"pnmExportFirst";text:"1";Layout.preferredWidth:75;onTextChanged:root.queueCheck();Accessible.name:"First exported capture"}
                TextField {id:last;objectName:"pnmExportLast";text:"1";Layout.preferredWidth:75;onTextChanged:root.queueCheck();Accessible.name:"Last exported capture"}
            }
        }
        CheckBox {id:clamp;objectName:"pnmExportClamp";text:"Clamp to PNM range";enabled:!root.busy || !root.saving;onToggled:root.queueCheck()
            ToolTip.visible:hovered || activeFocus;ToolTip.text:"Off: values outside 0–63.5 dB block export. On: Rust explicitly clamps and reports changed samples. Nonfinite values always fail."}
        Label {text:"PNM binary: 0–63.5 dB, rounded to the nearest 0.25 dB (half steps upward). JSON: full precision, without rounding or clamping. Both preserve missing readings, the full grid and capture times.";color:root.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label {objectName:"pnmExportSummary";visible:root.canJson || root.canExport || root.savedPath.length>0
            text:(root.summary.captures || 0)+" capture(s) · "+(root.summary.samples || 0)+" samples · "+(root.summary.missing || 0)+" missing\n"+
                (root.summary.rounded || 0)+" rounded · "+(root.summary.clipped || 0)+" clipped · maximum change "+Number(root.summary.maximum_change_db || 0).toFixed(3)+" dB"
            color:root.theme.text;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label {text:"A MERLIN provenance JSON accompanies each export. A collection is saved in a new folder with one file per capture. Existing files are never overwritten.";color:root.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        ProgressBar {visible:root.busy;indeterminate:root.saving;from:0;to:Math.max(1,root.captureCount);value:root.completed;Layout.fillWidth:true}
        ContextualStatus {theme:root.theme;diagnostic:root.diagnostic;onHelpRequested:function(code){root.contextHelpRequested(code)};objectName:"pnmExportStatus";text:root.busy ? (root.saving ? "Choose a destination, then export…":"Checking capture data…"):root.message
            color:root.canExport || root.savedPath.length ? root.theme.accent:root.theme.statusError;visible:text.length>0;Layout.fillWidth:true;wrapMode:Text.WordWrap}
    }
    footer:Pane {
        padding:10;background:Item {}
        contentItem:RowLayout {spacing:10
            Item {Layout.fillWidth:true}
            WorkbenchActionButton {theme:root.theme;objectName:"pnmExportSave";Layout.preferredWidth:120;text:"PNM binary";primary:true;enabled:root.canExport && !root.busy && !root.dirty
                onClicked:root.saveFormat("pnm")}
            WorkbenchActionButton {theme:root.theme;objectName:"jsonExportSave";Layout.preferredWidth:120;text:"JSON";enabled:root.canJson && !root.busy && !root.dirty
                onClicked:root.saveFormat("json")}
            WorkbenchActionButton {theme:root.theme;objectName:"pnmExportClose";Layout.preferredWidth:120;text:root.savedPath.length ? "Close":"Cancel";onClicked:root.close()}
        }
    }
}
