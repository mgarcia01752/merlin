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
    property var parentOptions: ({})
    property var information: ({})
    property var viewport: ({})
    property int viewStart: 0
    property int viewCount: 0
    property var valueEdits: []
    property bool initializing: false
    property bool working: false
    property string errorMessage: ""
    property var diagnostic: null
    signal contextHelpRequested(string code)
    property int completed: 0
    property int captureCount: source.currentIndex === 0 ? (information.imported_count || 1) : Number(count.text)
    signal staged(var metadata)
    title: "RxMER Designer"
    objectName: "rxmerDesignerDialog"
    parent:Overlay.overlay;anchors.centerIn:parent;modal:true
    width:Math.min(1240,parent ? parent.width-30:1240)
    height:Math.min(820,parent ? parent.height-30:820)
    function recipe() {
        return {tilt:tilt.checked ? {scope:tiltScope.value(),tilt_db:Number(tiltDb.text)} : null,
            ripple:ripple.checked ? {scope:rippleScope.value(),peak_to_peak_db:Number(rippleDb.text),spacing:distanceMode.checked ? {mode:"echo_distance",distance_metres:Number(distance.text),velocity_factor:Number(vf.text)} : {mode:"frequency",spacing_hz:Math.round(Number(spacing.text)*1e6)}} : null,
            notch:notch.checked ? {scope:notchScope.value(),center_hz:Math.round(Number(center.text)*1e6),bandwidth_hz:Math.round(Number(bandwidth.text)*1e6),depth_db:Number(depth.text),edge_hz:Math.round(Number(edge.text)*1e6)} : null,edits:valueEdits}
    }
    function canStepTilt(delta) {
        const value=Number(tiltDb.text)
        return tilt.checked && tiltDb.text.trim().length>0 && Number.isFinite(value) && value>=-60 && value<=60 && value+delta>=-60 && value+delta<=60
    }
    function stepTilt(delta) {
        if (canStepTilt(delta)) {tiltDb.text=String(Number(tiltDb.text)+delta);refresh()}
    }
    function document() { return JSON.stringify({source:source.currentIndex === 0 ? "imported":"flat",capture_count:captureCount,baseline_db:Number(baseline.text),recipe:recipe()}) }
    function refresh() { if (!initializing && visible && !working) { viewport=JSON.parse(backend.designer_preview(document(),Math.round(preview.value),viewStart,viewCount));errorMessage=viewport.error || "";diagnostic=viewport.diagnostic || null } }
    function setWindow(start,count) {viewStart=start;viewCount=count;refresh()}
    function restore() {
        viewStart=0;viewCount=0
        initializing=true
        information=JSON.parse(backend.designer_begin())
        errorMessage=information.error || ""
        if (information.request) {
            const request=information.request, rules=request.recipe
            source.currentIndex=request.source === "flat" ? 1:0;count.text=String(request.capture_count);baseline.text=String(request.baseline_db)
            tilt.checked=!!rules.tilt;tiltDb.text=String(rules.tilt ? rules.tilt.tilt_db:-4);tiltScope.restore(rules.tilt ? rules.tilt.scope:{mode:"all"})
            ripple.checked=!!rules.ripple;rippleDb.text=String(rules.ripple ? rules.ripple.peak_to_peak_db:2);rippleScope.restore(rules.ripple ? rules.ripple.scope:{mode:"all"})
            const period=rules.ripple ? rules.ripple.spacing:{mode:"frequency",spacing_hz:20000000}
            distanceMode.checked=period.mode === "echo_distance";spacing.text=String((period.spacing_hz || 20000000)/1e6);distance.text=String(period.distance_metres || 6.37);vf.text=String(period.velocity_factor || 0.85)
            notch.checked=!!rules.notch;notchScope.restore(rules.notch ? rules.notch.scope:{mode:"all"})
            center.text=String((rules.notch ? rules.notch.center_hz:(information.first_hz+information.last_hz)/2)/1e6)
            bandwidth.text=String((rules.notch ? rules.notch.bandwidth_hz:10000000)/1e6);depth.text=String(rules.notch ? rules.notch.depth_db:18);edge.text=String((rules.notch ? rules.notch.edge_hz:1000000)/1e6)
            valueEdits=rules.edits || [];editFirst.text=String(information.first_hz/1e6);editLast.text=editFirst.text;editScope.restore({mode:"all"});preview.value=1
        }
        initializing=false
        if (information.request) { refresh() }
    }
    onOpened:restore()
    onClosed: { if (working) { backend.designer_cancel() } }
    Timer {
        interval:80;repeat:true;running:dialog.working
        onTriggered: {
            const status=JSON.parse(dialog.backend.designer_poll())
            dialog.completed=status.completed || dialog.completed
            if (!status.running) {
                dialog.working=false
                if (status.staged) {dialog.staged(status.metadata);dialog.close()}
                else {dialog.errorMessage=status.error || "";dialog.diagnostic=status.diagnostic || null}
            }
        }
    }
    contentItem:ColumnLayout {
        spacing:10
        RowLayout {
            enabled:!dialog.working
            Label {text:"Baseline";color:theme.text}
            ComboBox {id:source;objectName:"designerSource";model:["Imported captures","New flat sequence"];Layout.preferredWidth:200;onActivated:dialog.refresh()}
            Label {text:source.currentIndex === 0 ? (dialog.information.imported_count || 0)+" captures":"Captures";color:theme.muted}
            TextField {id:count;objectName:"designerCount";visible:source.currentIndex === 1;text:"50";Layout.preferredWidth:65;validator:IntValidator {bottom:1;top:256}
 onEditingFinished:dialog.refresh()}
            Label {visible:source.currentIndex === 1;text:"Flat RxMER (dB)";color:theme.text}
            TextField {id:baseline;visible:source.currentIndex === 1;text:"48";Layout.preferredWidth:65;onEditingFinished:dialog.refresh()}
            Item {Layout.fillWidth:true}
        }
        RowLayout {
            Layout.fillWidth:true;Layout.fillHeight:true;spacing:18
            ScrollView {
                Layout.preferredWidth:400;Layout.fillHeight:true;clip:true;enabled:!dialog.working
                contentWidth:availableWidth
                ColumnLayout {
                    width:parent.width;spacing:8
                    Label {text:"Frequency effects";color:theme.accent;font.bold:true}
                    CheckBox {id:tilt;objectName:"designerTilt";text:"RxMER tilt";onToggled:dialog.refresh()}
                    RowLayout {enabled:tilt.checked;opacity:enabled ? 1:0.5
                        Label {text:"High − low edge (dB)";color:theme.text}
                        TextField {
                            id:tiltDb;objectName:"designerTiltDb";text:"-4";Layout.preferredWidth:80
                            Accessible.name:"High minus low edge tilt in dB"
                            onEditingFinished:dialog.refresh()
                            Keys.onUpPressed:dialog.stepTilt(1)
                            Keys.onDownPressed:dialog.stepTilt(-1)
                        }
                        WorkbenchActionButton {
                            theme:dialog.theme;objectName:"designerTiltDown";text:"↓";compact:true
                            enabled:dialog.canStepTilt(-1);onClicked:dialog.stepTilt(-1)
                            Accessible.name:"Decrease RxMER tilt by 1 dB"
                            ToolTip.visible:hovered || activeFocus;ToolTip.text:Accessible.name
                        }
                        WorkbenchActionButton {
                            theme:dialog.theme;objectName:"designerTiltUp";text:"↑";compact:true
                            enabled:dialog.canStepTilt(1);onClicked:dialog.stepTilt(1)
                            Accessible.name:"Increase RxMER tilt by 1 dB"
                            ToolTip.visible:hovered || activeFocus;ToolTip.text:Accessible.name
                        }
                        WorkbenchActionButton {
                            theme:dialog.theme;objectName:"designerTiltFlip";text:"Flip";compact:true
                            enabled:dialog.canStepTilt(0) && Number(tiltDb.text)!==0
                            onClicked:{tiltDb.text=String(-Number(tiltDb.text));dialog.refresh()}
                            Accessible.name:"Reverse RxMER tilt direction"
                            ToolTip.visible:hovered || activeFocus
                            ToolTip.text:"Reverse the slope: −5 dB ↔ +5 dB. Positive tilt raises the high-frequency edge; negative tilt lowers it."
                            Accessible.description:ToolTip.text
                        }
                    }
                    RxmerDesignScope {id:tiltScope;objectName:"designerTiltScope";enabled:tilt.checked;opacity:enabled ? 1:0.5;theme:dialog.theme;captureCount:dialog.captureCount;onEdited:dialog.refresh()}
                    Rectangle {Layout.fillWidth:true; height:1;color:theme.line}
                    CheckBox {id:ripple;objectName:"designerRipple";text:"Ripple";onToggled:dialog.refresh()}
                    RowLayout {Label {text:"Peak-to-peak (dB)";color:theme.text}
TextField {id:rippleDb;text:"2";Layout.preferredWidth:80;onEditingFinished:dialog.refresh()} }
                    CheckBox {id:distanceMode;text:"Use echo-distance spacing helper";onToggled:dialog.refresh();ToolTip.visible:hovered || activeFocus;ToolTip.text:"Assumes a round-trip cable cavity: crest spacing = c × velocity factor / (2 × distance). Depth is an independent MER edit; no RF phase is modeled."}
                    RowLayout {visible:!distanceMode.checked;Label {text:"Crest spacing (MHz)";color:theme.text}
TextField {id:spacing;text:"20";Layout.preferredWidth:80;onEditingFinished:dialog.refresh()} }
                    RowLayout {visible:distanceMode.checked;Label {text:"Distance (m)";color:theme.text}
TextField {id:distance;text:"6.37";Layout.preferredWidth:70;onEditingFinished:dialog.refresh()}
Label {text:"VF";color:theme.text}
TextField {id:vf;text:"0.85";Layout.preferredWidth:65;onEditingFinished:dialog.refresh()} }
                    RxmerDesignScope {id:rippleScope;theme:dialog.theme;captureCount:dialog.captureCount;onEdited:dialog.refresh()}
                    Rectangle {Layout.fillWidth:true;height:1;color:theme.line}
                    CheckBox {id:notch;objectName:"designerNotch";text:"LTE / 5G-style notch";onToggled:dialog.refresh();ToolTip.visible:hovered || activeFocus;ToolTip.text:"Synthetic reduction in MER over a frequency band, not an LTE or 5G waveform simulation."}
                    GridLayout {columns:4
                        Label {text:"Center MHz";color:theme.text}
TextField {id:center;Layout.preferredWidth:75;onEditingFinished:dialog.refresh()}
                        Label {text:"Width MHz";color:theme.text}
TextField {id:bandwidth;text:"10";Layout.preferredWidth:65;onEditingFinished:dialog.refresh()}
                        Label {text:"Depth dB";color:theme.text}
TextField {id:depth;text:"18";Layout.preferredWidth:75;onEditingFinished:dialog.refresh()}
                        Label {text:"Edge MHz";color:theme.text}
TextField {id:edge;text:"1";Layout.preferredWidth:65;onEditingFinished:dialog.refresh()}
                    }
                    RxmerDesignScope {id:notchScope;objectName:"designerNotchScope";theme:dialog.theme;captureCount:dialog.captureCount;onEdited:dialog.refresh()}
                    Rectangle {Layout.fillWidth:true;height:1;color:theme.line}
                    Label {text:"Replace subcarrier values";color:theme.accent;font.bold:true}
                    Label {text:"Use exact carrier-center frequencies. Equal endpoints edit one subcarrier. Missing values stay missing.";color:theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11}
                    RowLayout {Label {text:"First / last MHz";color:theme.text}
TextField {id:editFirst;Layout.preferredWidth:85}
TextField {id:editLast;Layout.preferredWidth:85} }
                    RowLayout {Label {text:"RxMER dB";color:theme.text}
TextField {id:editDb;text:"30";Layout.preferredWidth:75} }
                    RxmerDesignScope {id:editScope;theme:dialog.theme;captureCount:dialog.captureCount}
                    WorkbenchActionButton {theme:dialog.theme;text:"Add Value Edit";enabled:dialog.valueEdits.length<32;onClicked:{dialog.valueEdits=dialog.valueEdits.concat([{scope:editScope.value(),first_hz:Math.round(Number(editFirst.text)*1e6),last_hz:Math.round(Number(editLast.text)*1e6),value_db:Number(editDb.text)}]);dialog.refresh()} }
                    Repeater {model:dialog.valueEdits;delegate:RowLayout {required property var modelData;required property int index
                        Label {text:(modelData.first_hz/1e6)+"–"+(modelData.last_hz/1e6)+" MHz: "+modelData.value_db+" dB";color:theme.text;Layout.fillWidth:true;elide:Text.ElideRight}
                        WorkbenchActionButton {theme:dialog.theme;text:"Remove";compact:true;onClicked:{const next=dialog.valueEdits.slice();next.splice(index,1);dialog.valueEdits=next;dialog.refresh()} }
                    } }
                }
            }
            ColumnLayout {
                Layout.fillWidth:true;Layout.fillHeight:true
                RowLayout {
                    Label {text:"Original ┄  Designed ━  PLC ▰";color:theme.accent;Layout.fillWidth:true}
                    WorkbenchActionButton {theme:dialog.theme;objectName:"designerZoomPlc";text:"Zoom to PLC";enabled:!!dialog.viewport.plc_view && !dialog.working;onClicked:dialog.setWindow(dialog.viewport.plc_view.start,dialog.viewport.plc_view.count)}
                    WorkbenchActionButton {theme:dialog.theme;objectName:"designerFullSpan";text:"Full Span";enabled:!dialog.working;onClicked:dialog.setWindow(0,0)}
                }
                MeasurementPlot {theme:dialog.theme;viewport:dialog.viewport;Layout.fillWidth:true;Layout.fillHeight:true;onViewportRequested:function(start,count){dialog.setWindow(start,count)} }
                Label {text:dialog.viewport.plc_summary || "";color:theme.muted;font.pixelSize:11;Layout.fillWidth:true}
                RowLayout {
                    Label {text:"Preview capture "+Math.round(preview.value)+" / "+dialog.captureCount;color:theme.text}
                    Slider {id:preview;objectName:"designerPreviewCapture";Layout.fillWidth:true;from:1;to:Math.max(1,dialog.captureCount);stepSize:1;enabled:!dialog.working;onMoved:dialog.refresh()}
                }
                Repeater {model:dialog.viewport.timeline || [];delegate:RowLayout {required property var modelData
                    Label {text:modelData.label;Layout.preferredWidth:65;color:theme.muted}
                    Rectangle {Layout.fillWidth:true;height:14;color:theme.line;radius:3
                        Rectangle {x:parent.width*modelData.first;width:parent.width*(modelData.last-modelData.first);height:parent.height;radius:3;color:theme.accent;opacity:0.7}
                    }
                    Label {text:modelData.summary;Layout.preferredWidth:95;color:theme.muted;font.pixelSize:11}
                } }
                Label {text:source.currentIndex === 1 ? "Synthetic captures use the profile epoch for association only; replay timing comes from the workbench." : "Original files and capture times are preserved. Repeated Apply derives from the original baseline.";Layout.fillWidth:true;wrapMode:Text.WordWrap;color:theme.muted;font.pixelSize:11}
            }
        }
        ProgressBar {visible:dialog.working;Layout.fillWidth:true;from:0;to:Math.max(1,dialog.captureCount);value:dialog.completed}
        ContextualStatus {theme:dialog.theme;diagnostic:dialog.diagnostic;onHelpRequested:function(code){dialog.contextHelpRequested(code)};text:dialog.working ? "Designing captures: "+dialog.completed+" / "+dialog.captureCount:dialog.errorMessage;visible:text.length>0;color:dialog.errorMessage.length ? theme.statusError:theme.accent;Layout.fillWidth:true;wrapMode:Text.WordWrap}
    }
    footer:ApplyCancelFooter {
        theme:dialog.theme;applyObjectName:"designerApply";applyEnabled:!dialog.working && !!dialog.information.request && !dialog.errorMessage.length
        onApplied: {const result=JSON.parse(dialog.backend.designer_stage(dialog.document(),JSON.stringify(dialog.parentOptions)));dialog.errorMessage=result.error || "";dialog.diagnostic=result.diagnostic || null;dialog.completed=0;dialog.working=!!result.running}
        onCancelled:dialog.reject()
    }
}
