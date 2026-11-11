/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
WorkbenchDialog {
    id: dialog
    required property var backend
    required property var settingsBackend
    property bool busy: false
    property bool loading: false
    property var preview: ({error:"",points:[]})
    property string error: ""
    signal modelApplied()
    function document() {
        if (mode.currentIndex===1) { return {mode:"manual",tilt_db:slope.text.trim()==="" ? NaN : Number(slope.text)} }
        if (mode.currentIndex===2) { return {mode:"cable",cable:"belden1694a_v1",length:length.text.trim()==="" ? NaN : Number(length.text),unit:units.currentIndex===0 ? "feet" : "metres"} }
        return {mode:"off"}
    }
    function refresh() {
        if (loading || !backend) { return }
        preview=JSON.parse(backend.loss_preview(JSON.stringify(document()),settingsBackend.current()))
        error=preview.error || ""
    }
    onOpened: {
        loading=true
        const accepted=JSON.parse(backend.loss_current())
        mode.currentIndex=accepted.mode==="manual" ? 1 : accepted.mode==="cable" ? 2 : 0
        slope.text=String(accepted.tilt_db || 0);length.text=String(accepted.length || 0)
        units.currentIndex=accepted.unit==="metres" ? 1 : 0
        loading=false;refresh()
    }
    title:"Channel Model · Additional Loss"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(780,parent ? parent.width-40 : 780)
    height:Math.min(590,parent ? parent.height-40 : 590)
    modal:true
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "channelLossApply";applyEnabled:!dialog.busy && !dialog.error;onApplied:{dialog.error=dialog.backend.loss_apply(JSON.stringify(dialog.document()),dialog.settingsBackend.current());if (!dialog.error) {dialog.modelApplied();dialog.close()} }
        onCancelled: dialog.reject()
    }
    contentItem:ColumnLayout {
        spacing:12
        GridLayout {
            columns:2;columnSpacing:16;rowSpacing:10;Layout.fillWidth:true
            Label { text:"Additional Response";color:dialog.theme.text }
            ComboBox { id:mode;objectName:"channelLossMode";model:["Off","Manual Tilt","Cable / Distance"];Layout.fillWidth:true;onActivated:dialog.refresh() }
            Label { visible:mode.currentIndex===1;text:"High − Low Gain (dB)";color:dialog.theme.text }
            TextField { id:slope;objectName:"channelManualTilt";visible:mode.currentIndex===1;text:"0";validator:DoubleValidator {bottom:-30;top:30;locale:"C"} onTextChanged:dialog.refresh() }
            Label { visible:mode.currentIndex===2;text:"Cable Type";color:dialog.theme.text }
            ComboBox { visible:mode.currentIndex===2;model:["Belden 1694A · RG-6"];Layout.fillWidth:true }
            Label { visible:mode.currentIndex===2;text:"Additional Cable Length";color:dialog.theme.text }
            RowLayout {
                visible:mode.currentIndex===2
                TextField { id:length;objectName:"channelCableLength";text:"0";validator:DoubleValidator {bottom:0;top:1000;locale:"C"} Layout.fillWidth:true;onTextChanged:dialog.refresh() }
                ComboBox { id:units;objectName:"channelCableUnits";model:["ft","m"];onActivated:dialog.refresh() }
            }
            Label { text:"Channel Band";color:dialog.theme.muted }
            Label { text:dialog.preview.band || "—";color:dialog.theme.text }
            Label { text:"Low / High Gain";color:dialog.theme.muted }
            Label { text:(dialog.preview.low_gain || "—")+" / "+(dialog.preview.high_gain || "—");color:dialog.theme.text }
            Label { text:"High − Low Tilt";color:dialog.theme.muted }
            Label { text:dialog.preview.tilt || "—";color:dialog.theme.accent }
        }
        RowLayout {
            Label { text:"━ Additional Gain";color:dialog.theme.accent }
            Label { text:"━ Effective Response";color:dialog.theme.qamNcp }
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
        Label { visible:!!dialog.error;text:dialog.error;color:dialog.theme.statusError;wrapMode:Text.WordWrap;Layout.fillWidth:true }
    }
}
