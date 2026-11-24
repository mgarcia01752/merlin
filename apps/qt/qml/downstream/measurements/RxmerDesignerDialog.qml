/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import "../../shared/instruments"
import QtQuick.Controls
import QtQuick.Layouts
import "../../shared/components"
import "../../shared/presentation"
WorkbenchDialog {
    id: dialog
    required property var backend
    property var parentOptions: ({})
    readonly property var information: presentation.value ? presentation.value.information : ({})
    property var stagedAdopter: null
    property string stagedAdoptionMessage: ""
    property bool stagedAdoptionRejected: false
    property bool exportOpenRejected: false
    readonly property string actionLimitMessage: "Presentation limits reached; previous view preserved."
    readonly property int firstPreviewCapture: 1
    readonly property int fullSpanStart: 0
    readonly property int fullSpanCount: 0

    // Scientific domains come from the same public Rust owner that admits recipes.
    readonly property var domainLimits: information.limits || ({})
    readonly property bool limitsAvailable: !!information.limits
    readonly property bool editorTextAdmitted: presentation.textAdmitted && tiltScope.textAdmitted
        && rippleScope.textAdmitted && notchScope.textAdmitted && editScope.textAdmitted
    readonly property int maximumCaptures: Number.isInteger(domainLimits.maximum_captures) ? domainLimits.maximum_captures : 0

    // Existing UI increment, independent of the backend standard-deviation domain.
    readonly property real randomDeviationStepDb: 0.5
    readonly property var viewport: presentation.value ? presentation.value.viewport : ({})
    property int viewStart: 0
    property int viewCount: 0
    readonly property var valueEdits: presentation.value ? presentation.value.valueEdits : []
    property bool initializing: false
    property bool working: false
    readonly property string errorMessage: presentation.value ? presentation.value.errorMessage : ""
    readonly property var diagnostic: presentation.value ? presentation.value.diagnostic : null
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
        initialValue: ({information: {}, viewport: {}, valueEdits: [], errorMessage: "", diagnostic: null,display:{sources:["Imported captures","New flat sequence"],captures:"0 captures",edits:[]}})
    }
    property PresentationOwner progressCoverage: PresentationOwner {id:progressLabels;theme:dialog.theme}
    /** Admit display-only progress; worker flags remain live if details reject. */
    function refreshProgressLabels() {
        return progressLabels.replace({preview:"Preview capture "+Math.round(preview.value)+" / "+captureCount,
            progress:"Designing captures: "+completed+" / "+captureCount})
    }
    onCaptureCountChanged: if (progressLabels) {Qt.callLater(dialog.refreshProgressLabels)}
    onCompletedChanged: if (progressLabels) {Qt.callLater(dialog.refreshProgressLabels)}
    Component.onCompleted: {Qt.callLater(dialog.admitTextFields);Qt.callLater(dialog.refreshProgressLabels)}
    Component.onDestruction: {
        if (presentation) {
            presentation.clear()
        }
    }
    function admitTextFields() {
        return presentation.reserveTextFields([count, baseline, randomDeviation, randomSeed, tiltDb,
            rippleDb, spacing, distance, vf, center, bandwidth, depth, edge, editFirst, editLast, editDb])
            && tiltScope.reserveText() && rippleScope.reserveText() && notchScope.reserveText() && editScope.reserveText()
    }
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
            notch:notch.checked ? {scope:notchScope.value(),center_hz:Math.round(Number(center.text)*1e6),bandwidth_hz:Math.round(Number(bandwidth.text)*1e6),depth_db:Number(depth.text),edge_hz:Math.round(Number(edge.text)*1e6)} : null,edits:valueEdits.map(editValue)}
    }
    function canStepTilt(delta) {
        const value=Number(tiltDb.text)
        return limitsAvailable && tilt.checked && tiltDb.text.trim().length>0 && Number.isFinite(value) && value>=-domainLimits.maximum_absolute_tilt_db && value<=domainLimits.maximum_absolute_tilt_db && value+delta>=-domainLimits.maximum_absolute_tilt_db && value+delta<=domainLimits.maximum_absolute_tilt_db
    }
    function stepTilt(delta) {
        if (canStepTilt(delta)) {tiltDb.text=String(Number(tiltDb.text)+delta);refresh()}
    }
    // Mirrors the existing positive u32-Hz input domain for button guards only.
    readonly property real maximumFrequencyHz: 4294967295
    function canStepRipple(field,delta) {
        const value=Number(field.text), next=value+delta
        if (!limitsAvailable || !ripple.checked || field.text.trim().length===0 || !Number.isFinite(value) || !Number.isFinite(next)) {return false}
        if (field===rippleDb) {return value>=0 && value<=domainLimits.maximum_ripple_peak_to_peak_db && next>=0 && next<=domainLimits.maximum_ripple_peak_to_peak_db}
        return !distanceMode.checked && Math.round(value*1e6)>=1 && Math.round(value*1e6)<=maximumFrequencyHz
            && Math.round(next*1e6)>=1 && Math.round(next*1e6)<=maximumFrequencyHz
    }
    function stepRipple(field,delta) {
        if (canStepRipple(field,delta)) {field.text=String(Number(field.text)+delta);refresh()}
    }
    function canStepNotch(field,delta) {
        const value=Number(field.text), next=value+delta
        if (!limitsAvailable || !notch.checked || field.text.trim().length===0 || !Number.isFinite(value) || !Number.isFinite(next)) {return false}
        if (field===depth) {return value>=0 && value<=domainLimits.maximum_notch_depth_db && next>=0 && next<=domainLimits.maximum_notch_depth_db}
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
            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus;ToolTip.text:Accessible.name
        }
        WorkbenchActionButton {
            theme:stepper.theme;objectName:stepper.namePrefix+"Down";text:"▾";compact:true
            implicitWidth:26;implicitHeight:14;padding:0;font.pixelSize:10
            enabled:stepper.canDecrease;onClicked:stepper.decreaseRequested()
            Accessible.name:stepper.decreaseDescription
            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus;ToolTip.text:Accessible.name
        }
    }

    // Numeric text retains NaN/Infinity draft spelling without changing Rust wire values.
    function editText(edit) {
        const scope = Object.assign({}, edit.scope)
        if (Object.prototype.hasOwnProperty.call(scope, "first")) {
            scope.first = String(scope.first)
        }
        if (Object.prototype.hasOwnProperty.call(scope, "last")) {
            scope.last = String(scope.last)
        }
        return {scope: scope, first_hz: String(edit.first_hz), last_hz: String(edit.last_hz), value_db: String(edit.value_db)}
    }
    function editValue(edit) {
        const scope = Object.assign({}, edit.scope)
        if (Object.prototype.hasOwnProperty.call(scope, "first")) {
            scope.first = Number(scope.first)
        }
        if (Object.prototype.hasOwnProperty.call(scope, "last")) {
            scope.last = Number(scope.last)
        }
        return {scope: scope, first_hz: Number(edit.first_hz), last_hz: Number(edit.last_hz), value_db: Number(edit.value_db)}
    }
    function replaceState(nextInformation, nextViewport, nextEdits, nextError, nextDiagnostic) {
        return presentation.replace({information: nextInformation, viewport: nextViewport,
            valueEdits: nextEdits, errorMessage: nextError, diagnostic: nextDiagnostic,display:{
                sources:["Imported captures","New flat sequence"],captures:(nextInformation.imported_count || 0)+" captures",
                edits:nextEdits.map(function(edit){return (edit.first_hz/1e6)+"–"+(edit.last_hz/1e6)+" MHz: "+edit.value_db+" dB"})}})
    }
    function document() { return JSON.stringify({source:source.currentIndex === 0 ? "imported":"flat",capture_count:captureCount,baseline_db:Number(baseline.text),recipe:recipe()}) }
    function refresh() {
        if (initializing || !visible || working || !editorTextAdmitted) {
            return false
        }
        const nextViewport = JSON.parse(backend.designer_preview(document(),Math.round(preview.value),viewStart,viewCount))
        const accepted = replaceState(information, nextViewport, valueEdits, nextViewport.error || "", nextViewport.diagnostic || null)
        if (accepted) {
            stagedAdoptionRejected = false
            exportOpenRejected = false
        }
        return accepted
    }
    function setWindow(start,count) {viewStart=start;viewCount=count;return refresh()}
    function restore() {
        if (!admitTextFields()) {
            return false
        }
        initializing = true
        try {
            const nextInformation = JSON.parse(backend.designer_begin())
            const nextEdits = nextInformation.request ? (nextInformation.request.recipe.edits || []).map(editText) : valueEdits
            const nextViewport = nextInformation.request
                ? JSON.parse(backend.designer_preview(JSON.stringify(nextInformation.request),firstPreviewCapture,fullSpanStart,fullSpanCount)) : viewport
            const nextError = nextInformation.request ? nextViewport.error || "" : nextInformation.error || ""
            const nextDiagnostic = nextInformation.request ? nextViewport.diagnostic || null : diagnostic
            if (!replaceState(nextInformation, nextViewport, nextEdits, nextError, nextDiagnostic)) {
                return false
            }
            viewStart = fullSpanStart
            viewCount = fullSpanCount
            if (information.request) {
                const request=information.request, rules=request.recipe
                source.currentIndex=request.source === "flat" ? 1:0;count.text=String(request.capture_count);baseline.text=String(request.baseline_db)
                if (!tiltScope.restore(rules.tilt ? rules.tilt.scope:{mode:"all"})
                        || !rippleScope.restore(rules.ripple ? rules.ripple.scope:{mode:"all"})
                        || !notchScope.restore(rules.notch ? rules.notch.scope:{mode:"all"})
                        || !editScope.restore({mode:"all"})) {
                    return false
                }
                randomVariation.checked=!!rules.random_variation
                randomDeviation.text=String(rules.random_variation ? rules.random_variation.standard_deviation_db:1.5)
                randomSeed.text=String(rules.random_variation ? rules.random_variation.seed:41)
                tilt.checked=!!rules.tilt;tiltDb.text=String(rules.tilt ? rules.tilt.tilt_db:-4)
                ripple.checked=!!rules.ripple;rippleDb.text=String(rules.ripple ? rules.ripple.peak_to_peak_db:2)
                const period=rules.ripple ? rules.ripple.spacing:{mode:"frequency",spacing_hz:20000000}
                distanceMode.checked=period.mode === "echo_distance";spacing.text=String((period.spacing_hz || 20000000)/1e6);distance.text=String(period.distance_metres || 6.37);vf.text=String(period.velocity_factor || 0.85)
                notch.checked=!!rules.notch
                center.text=String((rules.notch ? rules.notch.center_hz:(information.first_hz+information.last_hz)/2)/1e6)
                bandwidth.text=String((rules.notch ? rules.notch.bandwidth_hz:10000000)/1e6);depth.text=String(rules.notch ? rules.notch.depth_db:18);edge.text=String((rules.notch ? rules.notch.edge_hz:1000000)/1e6)
                editFirst.text=String(information.first_hz/1e6);editLast.text=editFirst.text;preview.value=firstPreviewCapture
            }
            stagedAdoptionRejected = false
            exportOpenRejected = false
            return true
        } finally {
            initializing = false
        }
    }
    function addValueEdit() {
        if (!editorTextAdmitted) {
            return false
        }
        const next = valueEdits.concat([editText({scope:editScope.value(),first_hz:Math.round(Number(editFirst.text)*1e6),last_hz:Math.round(Number(editLast.text)*1e6),value_db:Number(editDb.text)})])
        if (replaceState(information, viewport, next, errorMessage, diagnostic)) {
            refresh()
        }
    }
    function removeValueEdit(index) {
        const next = valueEdits.slice()
        next.splice(index, 1)
        if (replaceState(information, viewport, next, errorMessage, diagnostic)) {
            refresh()
        }
    }
    function applyDesign() {
        if (!editorTextAdmitted) {
            return false
        }
        const result = JSON.parse(backend.designer_stage(document(),JSON.stringify(parentOptions)))
        completed = 0
        working = !!result.running
        stagedAdoptionRejected = false
        replaceState(information, viewport, valueEdits, result.error || "", result.diagnostic || null)
    }
    function publishStaged(metadata) {
        if (typeof stagedAdopter === "function" && stagedAdopter(metadata) !== true) {
            stagedAdoptionRejected = true
            return false
        }
        stagedAdoptionRejected = false
        staged(metadata)
        close()
        return true
    }
    function openExport() {
        if (!editorTextAdmitted) {
            return false
        }
        exportOpenRejected = !pnmExport.showSnapshot(document(),captureCount,Math.round(preview.value))
        return !exportOpenRejected
    }
    onOpened: restore()
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
                if (status.staged) {dialog.publishStaged(status.metadata)}
                else {dialog.replaceState(dialog.information, dialog.viewport, dialog.valueEdits, status.error || "", status.diagnostic || null)}
            }
        }
    }
    contentItem:ColumnLayout {
        spacing:10
        RowLayout {
            enabled:!dialog.working
            Label {text:"Baseline";color:theme.text}
            ComboBox {id:source;objectName:"designerSource";model:presentation.value ? presentation.value.display.sources : [];Layout.preferredWidth:200;onActivated:dialog.refresh()}
            Label {text:source.currentIndex === 0 && presentation.value ? presentation.value.display.captures : "Captures";color:theme.muted}
            TextField {id:count;objectName:"designerCount";visible:source.currentIndex === 1;text:presentation.textAdmitted ? "50" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:65;validator:IntValidator {bottom:1;top:dialog.maximumCaptures}
 onEditingFinished:dialog.refresh()}
            Label {visible:source.currentIndex === 1;text:"Flat RxMER (dB)";color:theme.text}
            TextField {id:baseline;visible:source.currentIndex === 1;text:presentation.textAdmitted ? "48" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:65;onEditingFinished:dialog.refresh()}
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
                        ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus
                        ToolTip.text:"Independent normally distributed dB offsets across all subcarriers. Available only for a new flat sequence; applied before tilt, ripple and notch."
                    }
                    RowLayout {
                        enabled:source.currentIndex === 1 && randomVariation.checked;opacity:enabled ? 1:0.5
                        Label {text:"Std deviation (dB)";color:theme.text}
                        TextField {id:randomDeviation;objectName:"designerRandomDeviation";text:presentation.textAdmitted ? "1.5" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:65;onEditingFinished:dialog.refresh();Accessible.name:"Random variation standard deviation in dB"}
                        StepButtons {
                            theme:dialog.theme;namePrefix:"designerRandomDeviation"
                            canIncrease:dialog.limitsAvailable && randomDeviation.text.trim().length>0 && Number.isFinite(Number(randomDeviation.text)) && Number(randomDeviation.text)>=0 && Number(randomDeviation.text)<=dialog.domainLimits.maximum_random_standard_deviation_db-dialog.randomDeviationStepDb
                            canDecrease:dialog.limitsAvailable && randomDeviation.text.trim().length>0 && Number.isFinite(Number(randomDeviation.text)) && Number(randomDeviation.text)>=dialog.randomDeviationStepDb && Number(randomDeviation.text)<=dialog.domainLimits.maximum_random_standard_deviation_db
                            increaseDescription:"Increase standard deviation by 0.5 dB";decreaseDescription:"Decrease standard deviation by 0.5 dB"
                            onIncreaseRequested:{randomDeviation.text=String(Number(randomDeviation.text)+dialog.randomDeviationStepDb);dialog.refresh()}
                            onDecreaseRequested:{randomDeviation.text=String(Number(randomDeviation.text)-dialog.randomDeviationStepDb);dialog.refresh()}
                        }
                    }
                    RowLayout {
                        enabled:source.currentIndex === 1 && randomVariation.checked;opacity:enabled ? 1:0.5
                        Label {text:"Random seed";color:theme.text}
                        TextField {id:randomSeed;objectName:"designerRandomSeed";text:presentation.textAdmitted ? "41" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:110;onEditingFinished:dialog.refresh();Accessible.name:"Designer random seed"
                            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || (activeFocus && (focusReason === Qt.TabFocusReason || focusReason === Qt.BacktabFocusReason || focusReason === Qt.ShortcutFocusReason));ToolTip.text:"Use the same seed to reproduce the same pattern. Each capture has a different pattern. This is separate from simulation Noise Seed."}
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
                            ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus
                            ToolTip.text:"The arrow points toward the higher MER edge. Click to reverse direction, keeping the same magnitude. Positive dB raises the high-frequency edge; negative dB lowers it."
                            Accessible.description:ToolTip.text
                        }
                        TextField {
                            id:tiltDb;objectName:"designerTiltDb";text:presentation.textAdmitted ? "-4" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:80
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
                    RxmerDesignScope {id:tiltScope;objectName:"designerTiltScope";enabled:tilt.checked;opacity:enabled ? 1:0.5;theme:dialog.theme;captureCount:dialog.captureCount;maximumCaptures:dialog.maximumCaptures;onEdited:dialog.refresh()}
                    Rectangle {Layout.fillWidth:true; height:1;color:theme.line}
                    CheckBox {id:ripple;objectName:"designerRipple";text:"Ripple";onToggled:dialog.refresh()}
                    RowLayout {
                        Label {text:"Peak-to-peak (dB)";color:theme.text}
                        TextField {
                            id:rippleDb;objectName:"designerRippleDb";text:presentation.textAdmitted ? "2" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:80
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
                    CheckBox {id:distanceMode;objectName:"designerRippleDistanceMode";text:"Use echo-distance spacing helper";onToggled:dialog.refresh();ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus;ToolTip.text:"Assumes a round-trip cable cavity: crest spacing = c × velocity factor / (2 × distance). Depth is an independent MER edit; no RF phase is modeled."}
                    RowLayout {
                        visible:!distanceMode.checked
                        Label {text:"Crest spacing (MHz)";color:theme.text}
                        TextField {
                            id:spacing;objectName:"designerRippleSpacing";text:presentation.textAdmitted ? "20" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:80
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
TextField {id:distance;text:presentation.textAdmitted ? "6.37" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:70;onEditingFinished:dialog.refresh()}
Label {text:"VF";color:theme.text}
TextField {id:vf;text:presentation.textAdmitted ? "0.85" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:65;onEditingFinished:dialog.refresh()} }
                    RxmerDesignScope {id:rippleScope;theme:dialog.theme;captureCount:dialog.captureCount;maximumCaptures:dialog.maximumCaptures;onEdited:dialog.refresh()}
                    Rectangle {Layout.fillWidth:true;height:1;color:theme.line}
                    CheckBox {id:notch;objectName:"designerNotch";text:"LTE / 5G-style notch";onToggled:dialog.refresh();ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus;ToolTip.text:"Synthetic reduction in MER over a frequency band, not an LTE or 5G waveform simulation."}
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
                            id:center;objectName:"designerNotchCenter";text:"";enabled:presentation.textAdmitted;Layout.preferredWidth:75
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
                            id:bandwidth;objectName:"designerNotchWidth";text:presentation.textAdmitted ? "10" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:65
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
                            id:depth;objectName:"designerNotchDepth";text:presentation.textAdmitted ? "18" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:75
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
                            id:edge;objectName:"designerNotchEdge";text:presentation.textAdmitted ? "1" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:65
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
                    RxmerDesignScope {id:notchScope;objectName:"designerNotchScope";theme:dialog.theme;captureCount:dialog.captureCount;maximumCaptures:dialog.maximumCaptures;onEdited:dialog.refresh()}
                    Rectangle {Layout.fillWidth:true;height:1;color:theme.line}
                    Label {text:"Replace subcarrier values";color:theme.accent;font.bold:true}
                    Label {text:"Use exact carrier-center frequencies. Equal endpoints edit one subcarrier. Missing values stay missing.";color:theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11}
                    RowLayout {Label {text:"First / last MHz";color:theme.text}
TextField {id:editFirst;enabled:presentation.textAdmitted;Layout.preferredWidth:85}
TextField {id:editLast;enabled:presentation.textAdmitted;Layout.preferredWidth:85} }
                    RowLayout {Label {text:"RxMER dB";color:theme.text}
TextField {id:editDb;text:presentation.textAdmitted ? "30" : "";enabled:presentation.textAdmitted;Layout.preferredWidth:75} }
                    RxmerDesignScope {id:editScope;theme:dialog.theme;captureCount:dialog.captureCount;maximumCaptures:dialog.maximumCaptures}
                    WorkbenchActionButton {theme:dialog.theme;objectName:"designerAddValueEdit";text:"Add Value Edit";enabled:dialog.limitsAvailable && dialog.valueEdits.length<dialog.domainLimits.maximum_value_edits;onClicked:dialog.addValueEdit() }
                    Repeater {model:dialog.valueEdits;delegate:RowLayout {required property var modelData;required property int index
                        Label {text:presentation.value ? presentation.value.display.edits[index] || "" : "";color:theme.text;Layout.fillWidth:true;elide:Text.ElideRight}
                        WorkbenchActionButton {theme:dialog.theme;text:"Remove";compact:true;onClicked:dialog.removeValueEdit(index) }
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
                    Label {text:progressLabels.value ? progressLabels.value.preview : "";color:theme.text}
                    Slider {id:preview;objectName:"designerPreviewCapture";Layout.fillWidth:true;from:1;to:Math.max(1,dialog.captureCount);stepSize:1;enabled:!dialog.working;onMoved:dialog.refresh();onValueChanged:if(progressLabels){Qt.callLater(dialog.refreshProgressLabels)}}
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
        ContextualStatus {theme:dialog.theme;diagnostic:presentation.rejected || dialog.stagedAdoptionRejected || dialog.exportOpenRejected ? null : dialog.diagnostic;onHelpRequested:function(code){dialog.contextHelpRequested(code)};text:dialog.working ? (progressLabels.value ? progressLabels.value.progress : dialog.actionLimitMessage) : !dialog.editorTextAdmitted ? dialog.actionLimitMessage : presentation.rejected ? presentation.message : dialog.stagedAdoptionRejected ? (dialog.stagedAdoptionMessage || dialog.actionLimitMessage) : dialog.exportOpenRejected ? dialog.actionLimitMessage : dialog.errorMessage;visible:text.length>0;color:dialog.errorMessage.length ? theme.statusError:theme.accent;Layout.fillWidth:true;wrapMode:Text.WordWrap}
    }
    footer:ApplyCancelFooter {
        theme:dialog.theme;applyObjectName:"designerApply";applyEnabled:dialog.editorTextAdmitted && !presentation.rejected && dialog.limitsAvailable && !dialog.working && !!dialog.information.request && !dialog.errorMessage.length
        onApplied: dialog.applyDesign()
        WorkbenchActionButton {theme:dialog.theme;objectName:"designerExportPnm";text:"Save";enabled:dialog.editorTextAdmitted && !presentation.rejected && dialog.limitsAvailable && !dialog.working && !!dialog.information.request && !dialog.errorMessage.length
            onClicked:dialog.openExport()}
        onCancelled:dialog.reject()
    }
}
