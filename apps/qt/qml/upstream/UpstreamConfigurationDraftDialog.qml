/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
import "../shared/theme"
import "../shared/presentation"

// Upstream configuration windows share this presentation-only draft form.
// The host retains/merges the complete scientific document and admits all edits.
WorkbenchDialog {
    id: dialog
    required property var host
    required property string sectionId
    required property var approvedFields
    property var presentation: ({})
    property var stages: []
    property string purpose: ""
    property string referencePlane: ""
    property string captureFamily: ""
    signal captureRequested(string family)
    signal draftRequested(string sectionId)
    signal draftApplyRequested(string sectionId, string token, var values)
    signal draftCancelled(string token)
    signal configurationLoadRequested
    readonly property var hostState: host && host.draftState ? host.draftState : ({})
    property PresentationOwner draftCoverage: PresentationOwner {
        id: draftOwner
        theme: dialog.theme
    }
    property PresentationOwner observationCoverage: PresentationOwner {
        id: observationOwner
        theme: dialog.theme
    }
    property PresentationOwner editorTextCoverage: PresentationOwner {
        id: textOwner
        theme: dialog.theme
    }
    readonly property var draft: draftOwner.value || ({})
    readonly property string token: draft.token || ""
    readonly property string baseSession: draft.baseSession || ""
    readonly property string baseGeneration: draft.baseGeneration || ""
    readonly property var fields: draft.fields || []
    readonly property var originalValues: draft.originalValues || ({})
    readonly property var changes: draft.changes || ({})
    readonly property var displayedPresentation: observationOwner.value ? observationOwner.value.presentation : ({})
    readonly property var displayedRows: observationOwner.value ? observationOwner.value.rows : []
    readonly property bool presentationRejected: draftOwner.rejected || observationOwner.rejected || textOwner.rejected
    property bool draftReady: false
    property bool valuesLoaded: false
    property bool submitting: false
    property bool cancellationRequested: false
    property bool cancellationSent: false
    property bool terminalReceived: false
    property bool textCapacityCurrent: false
    readonly property bool textAdmitted: textCapacityCurrent && textOwner.textAdmitted
    readonly property bool matchingState: matchesEnvelope()
    readonly property bool pending: submitting || cancellationRequested || (matchingState && hostState.pending === true)
    readonly property bool editable: valuesLoaded && textAdmitted && !draftOwner.rejected && matchingState && !pending && (hostState.outcome === "editing" || hostState.outcome === "rejected")
    readonly property bool configurationBusy: host && host.upstreamState ? host.upstreamState.busy === true || host.upstreamState.application_busy === true : false
    readonly property bool canLoadConfiguration: !pending && hostState.pending !== true && !configurationBusy
    readonly property bool canApply: editable && hostState.can_apply === true && Object.keys(changes).length > 0
    readonly property string diagnostic: draft.diagnostic || ""
    readonly property string admissionMessage: presentationRejected || (valuesLoaded && !textAdmitted) ? "Presentation limits reached; previous view preserved." : ""
    readonly property real preferredDialogWidth: 700
    readonly property real preferredDialogHeight: 650
    readonly property real windowMargin: 24
    readonly property real sectionSpacing: 16
    readonly property real fieldSpacing: 6
    readonly property real headerSpacing: 8
    readonly property real headerInset: 6
    readonly property real captionFontPixels: 12
    readonly property real panelRadius: 8
    readonly property real closeButtonWidth: 36
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(preferredDialogWidth, parent ? parent.width - windowMargin * 2 : preferredDialogWidth)
    height: Math.min(preferredDialogHeight, parent ? parent.height - windowMargin * 2 : preferredDialogHeight)
    modal: true
    focus: true
    closePolicy: Popup.NoAutoClose

    /**
     * Identify a fresh host draft using exact strings, never numeric conversion.
     * @returns Whether the host supplied a new envelope for this opened section.
     */
    function newEnvelope() {
        return hostState.section === sectionId && typeof hostState.token === "string" && hostState.token.length > 0 && hostState.token !== draft.previousToken && typeof hostState.baseSession === "string" && hostState.baseSession.length > 0 && typeof hostState.baseGeneration === "string" && hostState.baseGeneration.length > 0;
    }

    /**
     * Check identity directly before consuming a response or enabling an edit.
     * @returns Exact token, section and base equality without binding-order races.
     */
    function matchesEnvelope() {
        return token.length > 0 && hostState.section === sectionId && hostState.token === token && hostState.baseSession === baseSession && hostState.baseGeneration === baseGeneration;
    }

    /**
     * Compare presentation values without parsing, sorting or normalization.
     * @param left Original or changed string/selection.
     * @param right Original or changed string/selection.
     * @returns Exact scalar or ordered-array equality.
     */
    function sameValue(left, right) {
        if (Array.isArray(left) && Array.isArray(right)) {
            return left.length === right.length && left.every((value, index) => value === right[index]);
        }
        return left === right;
    }

    /**
     * Read one guarded value without modifying the host snapshot.
     * @param key Opaque adapter field key.
     * @returns Local change or the original supplied value.
     */
    function valueFor(key) {
        return Object.prototype.hasOwnProperty.call(changes, key) ? changes[key] : originalValues[key];
    }

    /**
     * Require authoritative metadata before enabling a control.
     * @param field Whitelisted host descriptor; limits remain host-owned.
     * @returns Whether exact string values and finite transport bounds exist.
     */
    function fieldAvailable(field) {
        if (!editable || field.editable !== true) {
            return false;
        }
        if (field.kind === "integer" || field.kind === "decimal") {
            return Number.isInteger(field.max_length) && field.max_length > 0 && typeof originalValues[field.key] === "string";
        }
        if (!Array.isArray(field.choices) || field.choices.length === 0 || !field.choices.every(choice => typeof choice.value === "string")) {
            return false;
        }
        if (field.kind === "selection") {
            return Number.isInteger(field.max_items) && field.max_items > 0 && Array.isArray(originalValues[field.key]) && originalValues[field.key].every(value => typeof value === "string");
        }
        return field.kind === "choice" && typeof originalValues[field.key] === "string";
    }

    /**
     * Pre-admit the coherent changed-value bundle before publishing a user edit.
     * @param field Host descriptor with its original opaque destination key.
     * @param value Exact string or ordered string array.
     * @returns False retains the previous draft and freezes Apply on rejection.
     */
    function changeField(field, value) {
        if (!fieldAvailable(field)) {
            return false;
        }
        const changed = Object.assign({}, changes);
        if (sameValue(value, originalValues[field.key])) {
            delete changed[field.key];
        } else {
            changed[field.key] = Array.isArray(value) ? value.slice() : value;
        }
        return draftOwner.replace(Object.assign({}, draft, {
            changes: changed
        }));
    }

    /**
     * Change an explicit choice without deriving allocation or physical geometry.
     * @param field Host selection descriptor and selection-count policy.
     * @param value Reported choice string.
     * @param selected Whether the user checked this option.
     * @returns Whether the guarded ordered selection was published.
     */
    function selectValue(field, value, selected) {
        if (!fieldAvailable(field)) {
            return false;
        }
        const values = valueFor(field.key).slice();
        const index = values.indexOf(value);
        if (selected && index < 0 && values.length < field.max_items) {
            values.push(value);
        } else if (!selected && index >= 0) {
            values.splice(index, 1);
        }
        return changeField(field, values);
    }

    /**
     * Reserve one complement for all actual descriptor-bound integer/decimal TextFields.
     * No field references survive this call. A rejected new capacity keeps all
     * new fields blank, even if an older reservation remains until disposal.
     * @returns True only after every existing numeric-text editor has text coverage.
     */
    function refreshTextAdmission() {
        if (!draftReady || !valuesLoaded) {
            textCapacityCurrent = false;
            return false;
        }
        const editors = [];
        for (let index = 0; index < fieldRepeater.count; index++) {
            const row = fieldRepeater.itemAt(index);
            if (!row || !row.textReady) {
                textCapacityCurrent = false;
                return false;
            }
            if (row.field.kind === "integer" || row.field.kind === "decimal") {
                editors.push(row.numericEditor);
            }
        }
        textCapacityCurrent = textOwner.reserveTextFields(editors);
        return textCapacityCurrent;
    }

    /**
     * Guard accepted display aliases together with independently derived labels.
     * Closed and never-opened dialogs do not copy incoming observation data.
     * @returns False preserves the prior admitted observation display.
     */
    function adoptPresentation() {
        if (!draftReady || !visible) {
            return false;
        }
        const rows = (presentation.rows || []).map(row => row.label + ": " + row.value);
        return observationOwner.replace({
            presentation: presentation,
            rows: rows
        });
    }

    /**
     * Consume only this draft's envelope. Admission precedes persistent copies;
     * terminal control transitions still complete if optional diagnostics reject.
     */
    function receiveState() {
        if (!draftReady || !visible || !draftOwner.value) {
            return;
        }
        if (!token.length && newEnvelope() && hostState.outcome === "editing") {
            if (!draftOwner.replace(Object.assign({}, draft, {
                token: hostState.token,
                baseSession: hostState.baseSession,
                baseGeneration: hostState.baseGeneration
            }))) {
                return;
            }
        }
        if (!matchesEnvelope()) {
            return;
        }
        const candidate = Object.assign({}, draft);
        let loadingValues = false;
        if (!valuesLoaded && !hostState.pending && hostState.outcome === "editing") {
            const supplied = hostState.options && Array.isArray(hostState.options.fields) ? hostState.options.fields : [];
            candidate.fields = supplied.filter(field => field && approvedFields.indexOf(field.field) >= 0 && typeof field.key === "string" && field.key.length > 0).map(field => {
                const descriptor = Object.assign({}, field, {
                    editorObjectName: "draftField_" + field.key
                });
                if (Array.isArray(field.choices)) {
                    descriptor.choices = field.choices.map(choice => Object.assign({}, choice, {
                            selectionObjectName: "draftSelection_" + field.key + "_" + choice.value
                        }));
                }
                return descriptor;
            });
            candidate.originalValues = {};
            for (const field of candidate.fields) {
                if (Object.prototype.hasOwnProperty.call(hostState.values || ({}), field.key)) {
                    const value = hostState.values[field.key];
                    candidate.originalValues[field.key] = Array.isArray(value) ? value.slice() : value;
                }
            }
            loadingValues = true;
        }
        candidate.diagnostic = hostState.error || (hostState.outcome === "stale" ? "The accepted configuration changed. Close and reopen this draft." : hostState.outcome === "rejected" ? "The draft was rejected. Review the supplied settings." : "");
        if (loadingValues || candidate.diagnostic !== draft.diagnostic) {
            if (draftOwner.replace(candidate) && loadingValues) {
                valuesLoaded = true;
                refreshTextAdmission();
            }
        }
        receiveControl(hostState);
    }

    /**
     * Consume only exact-token/base control flags from a transient native poll.
     * No incoming strings, fields or diagnostics survive this call. This path
     * remains live when the outer host cannot admit a replacement presentation.
     * @param state Native control envelope, or the already admitted host state.
     */
    function receiveControl(state) {
        if (!visible || !token.length || state.section !== sectionId || state.token !== token
                || state.baseSession !== baseSession || state.baseGeneration !== baseGeneration
                || state.pending !== false) {
            return;
        }
        if (state.outcome === "accepted" || state.outcome === "cancelled") {
            terminalReceived = true;
            submitting = false;
            if (state.outcome === "accepted") {
                dialog.accept();
            } else {
                dialog.reject();
            }
        } else if (state.outcome === "rejected" || state.outcome === "stale") {
            submitting = false;
            if (cancellationRequested) {
                terminalReceived = true;
                dialog.reject();
            }
        }
    }

    /**
     * Submit changed strings only, then wait for exact-token native adoption.
     * A queued return never establishes accepted scientific configuration.
     */
    function applyDraft() {
        if (!canApply) {
            return;
        }
        submitting = true;
        draftApplyRequested(sectionId, token, Object.assign({}, changes));
    }

    /**
     * Request native configuration acquisition without changing this draft.
     * The host owns its OS picker, bounded read and actual acceptance/refresh.
     */
    function loadConfiguration() {
        if (canLoadConfiguration) {
            configurationLoadRequested();
        }
    }

    /**
     * Request cancellation once for this token; never call global Stop.
     * If local identity admission failed, a fresh matching host token is used
     * only during this call so closing cannot abandon that native draft owner.
     */
    function requestCancellation() {
        const target = token.length ? token : draftOwner.value && newEnvelope() ? hostState.token : "";
        if (target.length && !cancellationSent && !terminalReceived) {
            cancellationSent = true;
            draftCancelled(target);
        }
    }

    /**
     * Discard idle edits, or wait for targeted pending suppression acknowledgement.
     */
    function cancelDraft() {
        receiveState();
        if (!visible || terminalReceived || cancellationRequested) {
            return;
        }
        const mustWait = pending;
        cancellationRequested = mustWait;
        requestCancellation();
        if (!mustWait) {
            dialog.reject();
        }
    }

    onHostStateChanged: receiveState()
    onPresentationChanged: adoptPresentation()
    onAboutToShow: {
        valuesLoaded = false;
        textCapacityCurrent = false;
        submitting = false;
        cancellationRequested = false;
        cancellationSent = false;
        terminalReceived = false;
        if (draftOwner.replace({
            previousToken: typeof hostState.token === "string" ? hostState.token : "",
            token: "",
            baseSession: "",
            baseGeneration: "",
            fields: [],
            originalValues: {},
            changes: {},
            diagnostic: ""
        })) {
            draftRequested(sectionId);
        }
    }
    onOpened: {
        adoptPresentation();
        receiveState();
    }
    onClosed: {
        requestCancellation();
        valuesLoaded = false;
        textCapacityCurrent = false;
        submitting = false;
        cancellationRequested = false;
        draftOwner.clear();
        observationOwner.clear();
    }
    Component.onCompleted: draftReady = true
    Component.onDestruction: {
        requestCancellation();
        draftReady = false;
        if (draftOwner) {
            draftOwner.clear();
        }
        if (observationOwner) {
            observationOwner.clear();
        }
    }
    property ThemePalette dialogPalette: ThemePalette {
        targetControl: dialog
        theme: dialog.theme
    }
    property Shortcut cancelShortcut: Shortcut {
        sequence: "Escape"
        enabled: dialog.visible
        onActivated: dialog.cancelDraft()
    }
    background: Rectangle {
        color: dialog.theme.panel
        border.color: dialog.theme.line
        radius: dialog.panelRadius
    }
    // The shared header's immediate reject cannot wait for pending suppression.
    header: RowLayout {
        spacing: dialog.headerSpacing
        Label {
            Layout.fillWidth: true
            Layout.margins: dialog.headerInset
            text: dialog.title
            color: dialog.theme.text
            font.bold: true
            elide: Text.ElideRight
        }
        WorkbenchActionButton {
            objectName: "popupCloseButton"
            theme: dialog.theme
            compact: true
            Layout.preferredWidth: dialog.closeButtonWidth
            text: "×"
            Accessible.name: "Close " + dialog.title
            ToolTip.visible: hovered
            ToolTip.text: "Close"
            onClicked: dialog.cancelDraft()
        }
    }
    contentItem: ScrollView {
        id: scroller
        clip: true
        contentWidth: availableWidth
        ColumnLayout {
            width: scroller.availableWidth
            spacing: dialog.sectionSpacing
            Label {
                Layout.fillWidth: true
                text: dialog.purpose
                color: dialog.theme.text
                wrapMode: Text.WordWrap
            }
            Label {
                objectName: "upstreamReferencePlane"
                Layout.fillWidth: true
                text: dialog.referencePlane
                color: dialog.theme.accent
                font.pixelSize: dialog.captionFontPixels
                wrapMode: Text.WordWrap
            }
            Label {
                Layout.fillWidth: true
                visible: dialog.stages.length > 0
                text: "Processing path"
                color: dialog.theme.text
                font.bold: true
            }
            Repeater {
                model: dialog.stages
                Label {
                    required property string modelData
                    Layout.fillWidth: true
                    text: modelData
                    textFormat: Text.PlainText
                    color: dialog.theme.muted
                    wrapMode: Text.WordWrap
                }
            }
            Label {
                objectName: "upstreamObservationSummary"
                Layout.fillWidth: true
                text: dialog.displayedPresentation.summary || "Unavailable. Configure this direction to obtain accepted state and observations."
                textFormat: Text.PlainText
                color: dialog.theme.muted
                wrapMode: Text.WordWrap
            }
            Repeater {
                model: dialog.displayedRows
                Label {
                    required property var modelData
                    Layout.fillWidth: true
                    text: modelData
                    textFormat: Text.PlainText
                    color: dialog.theme.muted
                    wrapMode: Text.WrapAnywhere
                }
            }
            Label {
                objectName: "upstreamDraftStatus"
                Layout.fillWidth: true
                text: dialog.cancellationRequested ? "Waiting for draft cancellation." : dialog.pending ? "Waiting for configuration admission." : !dialog.valuesLoaded ? "Editable draft unavailable. Waiting for the configuration adapter." : dialog.fields.length === 0 ? "Editable fields are unavailable for this configuration." : "Only Apply submits these draft changes."
                color: dialog.theme.muted
                wrapMode: Text.WordWrap
            }
            Repeater {
                id: fieldRepeater
                model: dialog.fields
                onItemAdded: dialog.refreshTextAdmission()
                onItemRemoved: dialog.refreshTextAdmission()
                ColumnLayout {
                    id: row
                    required property int index
                    readonly property var field: dialog.fields[index] || ({})
                    readonly property bool textReady: field.kind !== "integer" && field.kind !== "decimal" || editorLoader.status === Loader.Ready
                    readonly property var numericEditor: (field.kind === "integer" || field.kind === "decimal") && textReady ? editorLoader.item : null
                    onTextReadyChanged: dialog.refreshTextAdmission()
                    Layout.fillWidth: true
                    spacing: dialog.fieldSpacing
                    Label {
                        Layout.fillWidth: true
                        text: row.field.label || row.field.field
                        textFormat: Text.PlainText
                        color: dialog.theme.text
                        wrapMode: Text.WordWrap
                    }
                    Loader {
                        id: editorLoader
                        Layout.fillWidth: true
                        sourceComponent: row.field.kind === "choice" ? choiceEditor : (row.field.kind === "integer" || row.field.kind === "decimal") ? numericEditor : row.field.kind === "selection" ? selectionEditor : unavailableEditor
                        Component {
                            id: choiceEditor
                            ComboBox {
                                objectName: row.field.editorObjectName || ""
                                model: Array.isArray(row.field.choices) ? row.field.choices : []
                                textRole: "label"
                                enabled: dialog.editable && dialog.fieldAvailable(row.field)
                                currentIndex: (row.field.choices || []).findIndex(choice => choice.value === dialog.valueFor(row.field.key))
                                Accessible.name: row.field.label || row.field.field
                                onActivated: function (index) {
                                    if (!dialog.changeField(row.field, row.field.choices[index].value)) {
                                        currentIndex = row.field.choices.findIndex(choice => choice.value === dialog.valueFor(row.field.key));
                                    }
                                }
                            }
                        }
                        Component {
                            id: numericEditor
                            TextField {
                                id: numericField
                                readonly property bool textAdmitted: dialog.textAdmitted
                                Component.onCompleted: dialog.refreshTextAdmission()
                                objectName: row.field.editorObjectName || ""
                                text: textAdmitted && typeof dialog.valueFor(row.field.key) === "string" ? dialog.valueFor(row.field.key) : ""
                                enabled: dialog.editable && dialog.fieldAvailable(row.field)
                                maximumLength: Number.isInteger(row.field.max_length) && row.field.max_length > 0 ? row.field.max_length : 0
                                selectByMouse: true
                                Accessible.name: row.field.label || row.field.field
                                onTextEdited: {
                                    if (!dialog.changeField(row.field, text)) {
                                        text = dialog.valueFor(row.field.key) || "";
                                    }
                                }
                            }
                        }
                        Component {
                            id: selectionEditor
                            ColumnLayout {
                                spacing: dialog.fieldSpacing
                                Repeater {
                                    model: Array.isArray(row.field.choices) ? row.field.choices : []
                                    CheckBox {
                                        id: option
                                        required property var modelData
                                        objectName: modelData.selectionObjectName
                                        Layout.fillWidth: true
                                        text: modelData.label
                                        checked: Array.isArray(dialog.valueFor(row.field.key)) && dialog.valueFor(row.field.key).indexOf(modelData.value) >= 0
                                        enabled: dialog.editable && dialog.fieldAvailable(row.field) && (checked || dialog.valueFor(row.field.key).length < row.field.max_items)
                                        onToggled: {
                                            if (!dialog.selectValue(row.field, modelData.value, checked)) {
                                                checked = Array.isArray(dialog.valueFor(row.field.key)) && dialog.valueFor(row.field.key).indexOf(modelData.value) >= 0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Component {
                            id: unavailableEditor
                            Label {
                                text: "Editor metadata unavailable."
                                color: dialog.theme.muted
                            }
                        }
                    }
                    Label {
                        Layout.fillWidth: true
                        visible: text.length > 0
                        text: row.field.help || ""
                        textFormat: Text.PlainText
                        color: dialog.theme.muted
                        font.pixelSize: dialog.captionFontPixels
                        wrapMode: Text.WordWrap
                    }
                }
            }
            Label {
                objectName: "upstreamDraftAdmissionError"
                Layout.fillWidth: true
                visible: text.length > 0
                text: dialog.admissionMessage
                textFormat: Text.PlainText
                color: dialog.theme.statusError
                wrapMode: Text.WrapAnywhere
            }
            Label {
                objectName: "upstreamDraftError"
                Layout.fillWidth: true
                visible: text.length > 0
                text: dialog.diagnostic
                textFormat: Text.PlainText
                color: dialog.theme.statusError
                wrapMode: Text.WrapAnywhere
            }
        }
    }
    footer: ApplyCancelFooter {
        theme: dialog.theme
        applyObjectName: "upstreamDraftApply"
        cancelObjectName: "upstreamDraftCancel"
        applyEnabled: dialog.canApply
        onApplied: dialog.applyDraft()
        onCancelled: dialog.cancelDraft()
        WorkbenchActionButton {
            objectName: "upstreamCaptureInspect"
            theme: dialog.theme
            text: "View Capture"
            visible: dialog.captureFamily.length > 0
            enabled: dialog.canLoadConfiguration
            onClicked: {
                dialog.cancelDraft();
                dialog.captureRequested(dialog.captureFamily);
            }
        }
        WorkbenchActionButton {
            objectName: "upstreamConfigurationLoad"
            theme: dialog.theme
            text: "Load Configuration"
            enabled: dialog.canLoadConfiguration
            onClicked: dialog.loadConfiguration()
        }
    }
}
