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
        return {random_variation:source.currentIndex === 1 && randomVariation.checked ? {standard_deviation_db:randomDeviation.text.trim().length ? Number(randomDeviation.text):NaN,seed:randomSeed.text.trim().length ? Number(randomSeed.text):NaN} : null,tilt:tilt.checked ? {scope:tiltScope.value(),tilt_db:Number(tiltDb.text)} : null,
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
    // Mirrors the existing positive u32-Hz input domain for button guards only.
    readonly property real maximumFrequencyHz: 4294967295
    function canStepRipple(field,delta) {
        const value=Number(field.text), next=value+delta
        if (!ripple.checked || field.text.trim().length===0 || !Number.isFinite(value) || !Number.isFinite(next)) {return false}
        if (field===rippleDb) {return value>=0 && value<=60 && next>=0 && next<=60}
        return !distanceMode.checked && Math.round(value*1e6)>=1 && Math.round(value*1e6)<=maximumFrequencyHz
            && Math.round(next*1e6)>=1 && Math.round(next*1e6)<=maximumFrequencyHz
    }
    function stepRipple(field,delta) {
        if (canStepRipple(field,delta)) {field.text=String(Number(field.text)+delta);refresh()}
    }
    function canStepNotch(field,delta) {
        const value=Number(field.text), next=value+delta
        if (!notch.checked || field.text.trim().length===0 || !Number.isFinite(value) || !Number.isFinite(next)) {return false}
        if (field===depth) {return value>=0 && value<=60 && next>=0 && next<=60}
        const centerHz=Math.round(Number(center.text)*1e6), widthHz=Math.round(Number(bandwidth.text)*1e6), edgeHz=Math.round(Number(edge.text)*1e6)
        function valid(centerValue,widthValue,edgeValue) {
            return Number.isFinite(centerValue) && Number.isFinite(widthValue) && Number.isFinite(edgeValue)
                && centerValue>=0 && centerValue<=maximumFrequencyHz && widthValue>=1 && widthValue<=maximumFrequencyHz
                && edgeValue>=0 && edgeValue<=widthValue/2
                && centerValue-widthValue/2>=information.first_hz && centerValue+widthValue/2<=information.last_hz
        }
        if (center.text.trim().length===0 || bandwidth.text.trim().length===0 || edge.text.trim().length===0 || !valid(centerHz,widthHz,edgeHz)) {return false}
        return valid(field===center ? Math.round(next*1e6):centerHz,
            field===bandwidth ? Math.round(next*1e6):widthHz,field===edge ? Math.round(next*1e6):edgeHz)
    }
    function stepNotch(field,delta) {
        if (canStepNotch(field,delta)) {field.text=String(Number(field.text)+delta);refresh()}
    }
    component StepButtons: ColumnLayout {
        id:stepper
        required property var theme
        required property string namePrefix
        required property string increaseDescription
        required property string decreaseDescription
        property bool canIncrease: false
        property bool canDecrease: false
        signal increaseRequested()
        signal decreaseRequested()
        spacing:2
        Layout.alignment:Qt.AlignVCenter
        WorkbenchActionButton {
            theme:stepper.theme;objectName:stepper.namePrefix+"Up";text:"▴";compact:true
            implicitWidth:26;implicitHeight:14;padding:0;font.pixelSize:10
            enabled:stepper.canIncrease;onClicked:stepper.increaseRequested()
            Accessible.name:stepper.increaseDescription
            ToolTip.visible:hovered || activeFocus;ToolTip.text:Accessible.name
        }
        WorkbenchActionButton {
            theme:stepper.theme;objectName:stepper.namePrefix+"Down";text:"▾";compact:true
            implicitWidth:26;implicitHeight:14;padding:0;font.pixelSize:10
            enabled:stepper.canDecrease;onClicked:stepper.decreaseRequested()
            Accessible.name:stepper.decreaseDescription
            ToolTip.visible:hovered || activeFocus;ToolTip.text:Accessible.name
        }
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
            randomVariation.checked=!!rules.random_variation
            randomDeviation.text=String(rules.random_variation ? rules.random_variation.standard_deviation_db:1.5)
            randomSeed.text=String(rules.random_variation ? rules.random_variation.seed:41)
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
    onClosed: { pnmExport.close(); if (working) { backend.designer_cancel() } }
    RxmerPnmExportDialog {
        id:pnmExport;theme:dialog.theme;backend:dialog.backend
        onContextHelpRequested:function(code){dialog.contextHelpRequested(code)}
    }
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
                id:effectsScroll;objectName:"designerEffectsScroll"
                Layout.preferredWidth:400;Layout.fillHeight:true;clip:true;enabled:!dialog.working
                contentWidth:availableWidth
                ColumnLayout {
                    width:effectsScroll.availableWidth;spacing:8
                    Label {text:"Starting variation";color:theme.accent;font.bold:true}
                    CheckBox {
                        id:randomVariation;objectName:"designerRandomVariation";text:"Random variation";enabled:source.currentIndex === 1
                        onToggled:dialog.refresh()
                        ToolTip.visible:hovered || activeFocus
                        ToolTip.text:"Independent normally distributed dB offsets across all subcarriers. Available only for a new flat sequence; applied before tilt, ripple and notch."
                    }
                    RowLayout {
                        enabled:source.currentIndex === 1 && randomVariation.checked;opacity:enabled ? 1:0.5
                        Label {text:"Std deviation (dB)";color:theme.text}
                        TextField {id:randomDeviation;objectName:"designerRandomDeviation";text:"1.5";Layout.preferredWidth:65;onEditingFinished:dialog.refresh();Accessible.name:"Random variation standard deviation in dB"}
                        StepButtons {
                            theme:dialog.theme;namePrefix:"designerRandomDeviation"
                            canIncrease:randomDeviation.text.trim().length>0 && Number.isFinite(Number(randomDeviation.text)) && Number(randomDeviation.text)>=0 && Number(randomDeviation.text)<=19.5
                            canDecrease:randomDeviation.text.trim().length>0 && Number.isFinite(Number(randomDeviation.text)) && Number(randomDeviation.text)>=0.5 && Number(randomDeviation.text)<=20
                            increaseDescription:"Increase standard deviation by 0.5 dB";decreaseDescription:"Decrease standard deviation by 0.5 dB"
                            onIncreaseRequested:{randomDeviation.text=String(Number(randomDeviation.text)+0.5);dialog.refresh()}
                            onDecreaseRequested:{randomDeviation.text=String(Number(randomDeviation.text)-0.5);dialog.refresh()}
                        }
                    }
                    RowLayout {
                        enabled:source.currentIndex === 1 && randomVariation.checked;opacity:enabled ? 1:0.5
                        Label {text:"Random seed";color:theme.text}
                        TextField {id:randomSeed;objectName:"designerRandomSeed";text:"41";Layout.preferredWidth:110;onEditingFinished:dialog.refresh();Accessible.name:"Designer random seed"
                            ToolTip.visible:hovered || activeFocus;ToolTip.text:"Use the same seed to reproduce the same pattern. Each capture has a different pattern. This is separate from simulation Noise Seed."}
                    }
                    Label {text:source.currentIndex === 1 ? "Same variation across the channel; no frequency weighting." : "Imported captures keep their existing starting values.";color:theme.muted;wrapMode:Text.WordWrap;Layout.fillWidth:true;font.pixelSize:11}
                    Rectangle {Layout.fillWidth:true;height:1;color:theme.line}
                    Label {text:"Frequency effects";color:theme.accent;font.bold:true}
                    CheckBox {id:tilt;objectName:"designerTilt";text:"RxMER tilt";onToggled:dialog.refresh()}
                    RowLayout {enabled:tilt.checked;opacity:enabled ? 1:0.5
                        WorkbenchActionButton {
                            theme:dialog.theme;objectName:"designerTiltDirection";text:Number(tiltDb.text)<0 ? "Low ← High":"Low → High";compact:true
                            enabled:dialog.canStepTilt(0) && Number(tiltDb.text)!==0
                            onClicked:{tiltDb.text=String(-Number(tiltDb.text));dialog.refresh()}
                            Accessible.name:"Reverse RxMER tilt direction"
                            ToolTip.visible:hovered || activeFocus
                            ToolTip.text:"The arrow points toward the higher MER edge. Click to reverse direction, keeping the same magnitude. Positive dB raises the high-frequency edge; negative dB lowers it."
                            Accessible.description:ToolTip.text
                        }
                        TextField {
                            id:tiltDb;objectName:"designerTiltDb";text:"-4";Layout.preferredWidth:80
                            Accessible.name:"High minus low edge tilt in dB"
                            onEditingFinished:dialog.refresh()
                            Keys.onUpPressed:dialog.stepTilt(1)
                            Keys.onDownPressed:dialog.stepTilt(-1)
                        }
                        Label {text:"dB";color:theme.text}
                        StepButtons {
                            theme:dialog.theme;namePrefix:"designerTilt"
                            canIncrease:dialog.canStepTilt(1);canDecrease:dialog.canStepTilt(-1)
                            increaseDescription:"Increase RxMER tilt by 1 dB"
                            decreaseDescription:"Decrease RxMER tilt by 1 dB"
                            onIncreaseRequested:dialog.stepTilt(1)
                            onDecreaseRequested:dialog.stepTilt(-1)
                        }

                    }
                    RxmerDesignScope {id:tiltScope;objectName:"designerTiltScope";enabled:tilt.checked;opacity:enabled ? 1:0.5;theme:dialog.theme;captureCount:dialog.captureCount;onEdited:dialog.refresh()}
                    Rectangle {Layout.fillWidth:true; height:1;color:theme.line}
                    CheckBox {id:ripple;objectName:"designerRipple";text:"Ripple";onToggled:dialog.refresh()}
                    RowLayout {
                        Label {text:"Peak-to-peak (dB)";color:theme.text}
                        TextField {
                            id:rippleDb;objectName:"designerRippleDb";text:"2";Layout.preferredWidth:80
                            Accessible.name:"Ripple peak-to-peak depth in dB"
                            onEditingFinished:dialog.refresh()
                            Keys.onUpPressed:dialog.stepRipple(rippleDb,1)
                            Keys.onDownPressed:dialog.stepRipple(rippleDb,-1)
                        }
                        StepButtons {
                            theme:dialog.theme;namePrefix:"designerRippleDepth"
                            canIncrease:dialog.canStepRipple(rippleDb,1);canDecrease:dialog.canStepRipple(rippleDb,-1)
                            increaseDescription:"Increase ripple peak-to-peak by 1 dB"
                            decreaseDescription:"Decrease ripple peak-to-peak by 1 dB"
                            onIncreaseRequested:dialog.stepRipple(rippleDb,1)
                            onDecreaseRequested:dialog.stepRipple(rippleDb,-1)
                        }
                    }
                    CheckBox {id:distanceMode;objectName:"designerRippleDistanceMode";text:"Use echo-distance spacing helper";onToggled:dialog.refresh();ToolTip.visible:hovered || activeFocus;ToolTip.text:"Assumes a round-trip cable cavity: crest spacing = c × velocity factor / (2 × distance). Depth is an independent MER edit; no RF phase is modeled."}
                    RowLayout {
                        visible:!distanceMode.checked
                        Label {text:"Crest spacing (MHz)";color:theme.text}
                        TextField {
                            id:spacing;objectName:"designerRippleSpacing";text:"20";Layout.preferredWidth:80
                            Accessible.name:"Ripple crest spacing in MHz"
                            onEditingFinished:dialog.refresh()
                            Keys.onUpPressed:dialog.stepRipple(spacing,1)
                            Keys.onDownPressed:dialog.stepRipple(spacing,-1)
                        }
                        StepButtons {
                            theme:dialog.theme;namePrefix:"designerRippleSpacing"
                            canIncrease:dialog.canStepRipple(spacing,1);canDecrease:dialog.canStepRipple(spacing,-1)
                            increaseDescription:"Increase ripple crest spacing by 1 MHz"
                            decreaseDescription:"Decrease ripple crest spacing by 1 MHz"
                            onIncreaseRequested:dialog.stepRipple(spacing,1)
                            onDecreaseRequested:dialog.stepRipple(spacing,-1)
                        }
                    }
                    RowLayout {visible:distanceMode.checked;Label {text:"Distance (m)";color:theme.text}
TextField {id:distance;text:"6.37";Layout.preferredWidth:70;onEditingFinished:dialog.refresh()}
Label {text:"VF";color:theme.text}
TextField {id:vf;text:"0.85";Layout.preferredWidth:65;onEditingFinished:dialog.refresh()} }
                    RxmerDesignScope {id:rippleScope;theme:dialog.theme;captureCount:dialog.captureCount;onEdited:dialog.refresh()}
                    Rectangle {Layout.fillWidth:true;height:1;color:theme.line}
                    CheckBox {id:notch;objectName:"designerNotch";text:"LTE / 5G-style notch";onToggled:dialog.refresh();ToolTip.visible:hovered || activeFocus;ToolTip.text:"Synthetic reduction in MER over a frequency band, not an LTE or 5G waveform simulation."}
                    GridLayout {
                        id:notchFields;objectName:"designerNotchFields"
                        Layout.fillWidth:true
                        // Native font metrics can make two groups exceed the editor's clip.
                        readonly property real pairedWidth: Math.max(notchCenterLabel.implicitWidth,notchDepthLabel.implicitWidth)
                            + center.Layout.preferredWidth + notchCenterSteps.implicitWidth
                            + Math.max(notchWidthLabel.implicitWidth,notchEdgeLabel.implicitWidth)
                            + bandwidth.Layout.preferredWidth + notchWidthSteps.implicitWidth + 5*columnSpacing
                        columns:effectsScroll.availableWidth>=pairedWidth ? 6:3
                        Label {id:notchCenterLabel;text:"Center MHz";color:theme.text}
                        TextField {
                            id:center;objectName:"designerNotchCenter";text:"";Layout.preferredWidth:75
                            Accessible.name:"Notch center in MHz"
                            onEditingFinished:dialog.refresh()
                            Keys.onUpPressed:dialog.stepNotch(center,1)
                            Keys.onDownPressed:dialog.stepNotch(center,-1)
                        }
                        StepButtons {
                            id:notchCenterSteps;theme:dialog.theme;namePrefix:"designerNotchCenter"
                            canIncrease:dialog.canStepNotch(center,1);canDecrease:dialog.canStepNotch(center,-1)
                            increaseDescription:"Increase notch center by 1 MHz"
                            decreaseDescription:"Decrease notch center by 1 MHz"
                            onIncreaseRequested:dialog.stepNotch(center,1)
                            onDecreaseRequested:dialog.stepNotch(center,-1)
                        }
                        Label {id:notchWidthLabel;text:"Width MHz";color:theme.text}
                        TextField {
                            id:bandwidth;objectName:"designerNotchWidth";text:"10";Layout.preferredWidth:65
                            Accessible.name:"Notch width in MHz"
                            onEditingFinished:dialog.refresh()
                            Keys.onUpPressed:dialog.stepNotch(bandwidth,1)
                            Keys.onDownPressed:dialog.stepNotch(bandwidth,-1)
                        }
                        StepButtons {
                            id:notchWidthSteps;theme:dialog.theme;namePrefix:"designerNotchWidth"
                            canIncrease:dialog.canStepNotch(bandwidth,1);canDecrease:dialog.canStepNotch(bandwidth,-1)
                            increaseDescription:"Increase notch width by 1 MHz"
                            decreaseDescription:"Decrease notch width by 1 MHz"
                            onIncreaseRequested:dialog.stepNotch(bandwidth,1)
                            onDecreaseRequested:dialog.stepNotch(bandwidth,-1)
                        }
                        Label {id:notchDepthLabel;text:"Depth dB";color:theme.text}
                        TextField {
                            id:depth;objectName:"designerNotchDepth";text:"18";Layout.preferredWidth:75
                            Accessible.name:"Notch depth in dB"
                            onEditingFinished:dialog.refresh()
                            Keys.onUpPressed:dialog.stepNotch(depth,1)
                            Keys.onDownPressed:dialog.stepNotch(depth,-1)
                        }
                        StepButtons {
                            theme:dialog.theme;namePrefix:"designerNotchDepth"
                            canIncrease:dialog.canStepNotch(depth,1);canDecrease:dialog.canStepNotch(depth,-1)
                            increaseDescription:"Increase notch depth by 1 dB"
                            decreaseDescription:"Decrease notch depth by 1 dB"
                            onIncreaseRequested:dialog.stepNotch(depth,1)
                            onDecreaseRequested:dialog.stepNotch(depth,-1)
                        }
                        Label {id:notchEdgeLabel;text:"Edge MHz";color:theme.text}
                        TextField {
                            id:edge;objectName:"designerNotchEdge";text:"1";Layout.preferredWidth:65
                            Accessible.name:"Notch edge in MHz"
                            onEditingFinished:dialog.refresh()
                            Keys.onUpPressed:dialog.stepNotch(edge,1)
                            Keys.onDownPressed:dialog.stepNotch(edge,-1)
                        }
                        StepButtons {
                            theme:dialog.theme;namePrefix:"designerNotchEdge"
                            canIncrease:dialog.canStepNotch(edge,1);canDecrease:dialog.canStepNotch(edge,-1)
                            increaseDescription:"Increase notch edge by 1 MHz"
                            decreaseDescription:"Decrease notch edge by 1 MHz"
                            onIncreaseRequested:dialog.stepNotch(edge,1)
                            onDecreaseRequested:dialog.stepNotch(edge,-1)
                        }
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
        WorkbenchActionButton {theme:dialog.theme;objectName:"designerExportPnm";text:"Save";enabled:!dialog.working && !!dialog.information.request && !dialog.errorMessage.length
            onClicked:{pnmExport.documentSnapshot=dialog.document();pnmExport.captureCount=dialog.captureCount;pnmExport.previewCapture=Math.round(preview.value);pnmExport.open()}}
        onCancelled:dialog.reject()
    }
}
