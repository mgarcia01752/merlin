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
    id: dialog
    required property var backend
    required property var settingsBackend
    property bool busy: false
    property bool loading: false
    readonly property var preview: presentation.value ? presentation.value.preview : ({error:"",points:[]})
    readonly property string error: presentation.value ? presentation.value.error : ""
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: dialog.previewCandidate({error:"",points:[]},"")
    }
    Component.onCompleted: Qt.callLater(dialog.admitTextFields)
    Component.onDestruction: {
        if (presentation) {
            presentation.clear()
        }
    }
    function admitTextFields() {
        return presentation.reserveTextFields([slope, length])
    }
    /** Prepare existing preview text and plain selectors before conversion. */
    function previewCandidate(nextPreview,nextError) {
        return {preview:nextPreview,error:nextError,
            band: nextPreview.band || "—", gain: (nextPreview.low_gain || "—")+" / "+(nextPreview.high_gain || "—"),
            tilt:nextPreview.tilt || "—",modes:["Off","Manual Tilt","Cable / Distance"],cables:["Belden 1694A · RG-6"],units:["ft","m"]}
    }
    function replacePreview(nextPreview,nextError) {
        return presentation.replace(previewCandidate(nextPreview,nextError))
    }
    signal modelApplied()
    function documentFor(modeIndex, slopeText, lengthText, unitIndex) {
        if (modeIndex===1) { return {mode:"manual",tilt_db:slopeText.trim()==="" ? NaN : Number(slopeText)} }
        if (modeIndex===2) { return {mode:"cable",cable:"belden1694a_v1",length:lengthText.trim()==="" ? NaN : Number(lengthText),unit:unitIndex===0 ? "feet" : "metres"} }
        return {mode:"off"}
    }
    function document() {
        return documentFor(mode.currentIndex,slope.text,length.text,units.currentIndex)
    }
    function refresh() {
        if (loading || !backend || !presentation || !presentation.textAdmitted) {
            return false
        }
        const nextPreview=JSON.parse(backend.loss_preview(JSON.stringify(document()),settingsBackend.current()))
        return replacePreview(nextPreview,nextPreview.error || "")
    }
    function restore() {
        if (!admitTextFields()) {
            return false
        }
        const accepted=JSON.parse(backend.loss_current())
        const nextMode=accepted.mode==="manual" ? 1 : accepted.mode==="cable" ? 2 : 0
        const nextSlope=String(accepted.tilt_db || 0), nextLength=String(accepted.length || 0)
        const nextUnits=accepted.unit==="metres" ? 1 : 0
        const nextPreview=JSON.parse(backend.loss_preview(JSON.stringify(documentFor(nextMode,nextSlope,nextLength,nextUnits)),settingsBackend.current()))
        if (!replacePreview(nextPreview,nextPreview.error || "")) {
            return false
        }
        loading=true
        try {
            mode.currentIndex=nextMode
            slope.text=nextSlope;length.text=nextLength
            units.currentIndex=nextUnits
        } finally {
            loading=false
        }
        return true
    }
    function applyDraft() {
        if (!presentation.textAdmitted || !presentation.value || presentation.rejected) {
            return
        }
        const result=backend.loss_apply(JSON.stringify(document()),settingsBackend.current())
        replacePreview(preview,result)
        if (!result) {modelApplied();close()}
    }
    onOpened: restore()
    title:"Channel Model · Additional Loss"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(780,parent ? parent.width-40 : 780)
    height:Math.min(590,parent ? parent.height-40 : 590)
    modal:true
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "channelLossApply";applyEnabled:presentation.textAdmitted && !presentation.rejected && !dialog.busy && !dialog.error;onApplied:dialog.applyDraft()
        onCancelled: dialog.reject()
    }
    contentItem:ColumnLayout {
        spacing:12
        GridLayout {
            columns:2;columnSpacing:16;rowSpacing:10;Layout.fillWidth:true
            Label { text:"Additional Response";color:dialog.theme.text }
            ComboBox { id:mode;objectName:"channelLossMode";model:presentation.value ? presentation.value.modes : [];enabled:presentation.textAdmitted;Layout.fillWidth:true;onActivated:dialog.refresh() }
            Label { visible:mode.currentIndex===1;text:"High − Low Gain (dB)";color:dialog.theme.text }
            TextField { id:slope;objectName:"channelManualTilt";visible:mode.currentIndex===1;text:presentation.textAdmitted ? "0" : "";enabled:presentation.textAdmitted;validator:DoubleValidator {bottom:-30;top:30;locale:"C"} onTextChanged:dialog.refresh() }
            Label { visible:mode.currentIndex===2;text:"Cable Type";color:dialog.theme.text }
            ComboBox { visible:mode.currentIndex===2;model:presentation.value ? presentation.value.cables : [];Layout.fillWidth:true }
            Label { visible:mode.currentIndex===2;text:"Additional Cable Length";color:dialog.theme.text }
            RowLayout {
                visible:mode.currentIndex===2
                TextField { id:length;objectName:"channelCableLength";text:presentation.textAdmitted ? "0" : "";enabled:presentation.textAdmitted;validator:DoubleValidator {bottom:0;top:1000;locale:"C"} Layout.fillWidth:true;onTextChanged:dialog.refresh() }
                ComboBox { id:units;objectName:"channelCableUnits";model:presentation.value ? presentation.value.units : [];enabled:presentation.textAdmitted;onActivated:dialog.refresh() }
            }
            Label { text:"Channel Band";color:dialog.theme.muted }
            Label { text:presentation.value ? presentation.value.band : "—";color:dialog.theme.text }
            Label { text:"Low / High Gain";color:dialog.theme.muted }
            Label { text:presentation.value ? presentation.value.gain : "— / —";color:dialog.theme.text }
            Label { text:"High − Low Tilt";color:dialog.theme.muted }
            Label { text:presentation.value ? presentation.value.tilt : "—";color:dialog.theme.accent }
        }
        RowLayout {
            Label { text:"━ Additional Gain";color:dialog.theme.accent }
            Label { text:"━ Supplied H + Loss";color:dialog.theme.qamNcp }
        }
        Canvas {
            id:plot;objectName:"channelLossPlot";Layout.fillWidth:true;Layout.fillHeight:true
            Connections { target:dialog;function onPreviewChanged(){plot.requestPaint()} }
            Connections { target:dialog.theme;function onDarkChanged(){plot.requestPaint()} }
            onWidthChanged:requestPaint();onHeightChanged:requestPaint()
            onPaint: {
                const ctx=getContext("2d");ctx.reset()
                const points=dialog.preview.chart || []
                const left=52,top=8,w=width-70,h=height-35
                if (w<=0 || h<=0) {return}
                ctx.fillStyle=dialog.theme.background;ctx.fillRect(0,0,width,height)
                ctx.font="11px sans-serif";ctx.lineWidth=1
                for (const tick of dialog.preview.y_ticks || []) {
                    const y=top+tick.x*h
                    ctx.strokeStyle=dialog.theme.line;ctx.beginPath();ctx.moveTo(left,y);ctx.lineTo(left+w,y);ctx.stroke()
                    ctx.fillStyle=dialog.theme.muted;ctx.fillText(tick.label,2,y+4)
                }
                for (const tick of dialog.preview.x_ticks || []) {ctx.fillStyle=dialog.theme.muted;ctx.textAlign=tick.x===0 ? "left" : "right";ctx.fillText(tick.label,left+tick.x*w,top+h+20)}
                for (const series of ["additional","effective"]) {
                    ctx.strokeStyle=series==="additional" ? dialog.theme.accent : dialog.theme.qamNcp;ctx.lineWidth=2;ctx.beginPath()
                    for (let i=0;i<points.length;i++) {const p=points[i];if(i===0){ctx.moveTo(left+p.x*w,top+p[series]*h)}else{ctx.lineTo(left+p.x*w,top+p[series]*h)}}
                    ctx.stroke()
                }
            }
        }
        Label { visible:presentation.rejected || !!dialog.error;text:presentation.rejected ? presentation.message : dialog.error;color:dialog.theme.statusError;wrapMode:Text.WordWrap;Layout.fillWidth:true }
    }
}
