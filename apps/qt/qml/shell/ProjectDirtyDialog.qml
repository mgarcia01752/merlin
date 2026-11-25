/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../shared/components"
import "../shared/presentation"

// Presentation only. The host owns the canonical native revision and file intent.
WorkbenchDialog {
    id: dialog
    property string direction: ""
    property string action: ""
    property bool saveAllowed: false
    signal saveRequested
    signal discardRequested
    signal cancelled

    readonly property real preferredDialogWidth: 560
    readonly property real viewportEdgeInset: 20
    readonly property real contentSpacing: 12
    readonly property real footerPadding: 10
    readonly property real actionSpacing: 10
    readonly property int rowActionColumns: 3
    readonly property int stackedActionColumns: 1
    readonly property bool contextCurrent: !!presentation.value && direction === presentation.value.direction && action === presentation.value.action && validTags()
    readonly property bool decisionPending: lifecycle.awaiting && lifecycle.choice === ""
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: presentation
        theme: dialog.theme
    }
    property QtObject outcomeState: QtObject {
        id: lifecycle
        property bool awaiting: false
        property bool emissionPending: false
        property string choice: ""
    }

    /** Accept only the compiled shell direction/action tags. */
    function validTags() {
        return (direction === "downstream" || direction === "upstream") && (action === "open" || action === "new" || action === "close");
    }

    /** Build only compiled presentation labels; no native context is copied. */
    function labels() {
        const name = direction === "downstream" ? "Downstream" : "Upstream";
        const lowerName = direction === "downstream" ? "downstream" : "upstream";
        const purpose = action === "open" ? "opening another project" : action === "new" ? "creating a new workbench" : "closing";
        return {
            direction: direction,
            action: action,
            title: "Unsaved " + name + " Workbench",
            message: "Save changes to the " + lowerName + " workbench before " + purpose + "?",
            unavailable: "Saving is unavailable for this workbench.",
            save: "Save",
            discard: "Discard",
            cancel: "Cancel",
            saveDescription: "Save changes before continuing.",
            discardDescription: "Continue without saving changes.",
            cancelDescription: "Cancel this operation and keep the workbench."
        };
    }

    /** Mark the existing shell feedback when bounded admission rejects a prompt. */
    function admissionRejected() {
        if (typeof theme.presentationAdmissionRejected === "boolean") {
            theme.presentationAdmissionRejected = true;
        }
    }

    /**
     * Validate/admit compiled content before reusing WorkbenchDialog visibility.
     * @returns False leaves the native file intent untouched and emits no outcome.
     */
    function openChecked() {
        if (!validTags() || visible || lifecycle.awaiting || lifecycle.emissionPending) {
            return false;
        }
        if (!presentation.replace(labels())) {
            admissionRejected();
            return false;
        }
        if (!visibilityCoverage.beginOpen()) {
            presentation.clear();
            admissionRejected();
            return false;
        }
        if (typeof theme.presentationAdmissionRejected === "boolean") {
            theme.presentationAdmissionRejected = false;
        }
        lifecycle.awaiting = true;
        lifecycle.choice = "";
        dialog.open();
        return true;
    }

    /** Resolve one compiled outcome; repeated/disabled/stale actions do nothing. */
    function resolve(choice) {
        if (!decisionPending || !visible || (choice !== "save" && choice !== "discard" && choice !== "cancel")) {
            return false;
        }
        if (choice !== "cancel" && !contextCurrent) {
            return false;
        }
        if (choice === "save" && !saveAllowed) {
            return false;
        }
        lifecycle.choice = choice;
        dialog.close();
        return true;
    }

    /** Emit only after all inherited closed handlers released their open slot. */
    function publishOutcome() {
        if (!lifecycle.emissionPending) {
            return;
        }
        const choice = lifecycle.choice;
        lifecycle.choice = "";
        lifecycle.emissionPending = false;
        if (choice === "save") {
            saveRequested();
        } else if (choice === "discard") {
            discardRequested();
        } else {
            cancelled();
        }
    }

    onAboutToHide: {
        if (lifecycle.awaiting && lifecycle.choice === "") {
            lifecycle.choice = "cancel";
        }
    }
    onClosed: {
        if (lifecycle.awaiting) {
            lifecycle.awaiting = false;
            if (lifecycle.choice === "") {
                lifecycle.choice = "cancel";
            }
            presentation.clear();
            lifecycle.emissionPending = true;
            Qt.callLater(dialog.publishOutcome);
        }
    }
    onOpened: cancelButton.forceActiveFocus(Qt.PopupFocusReason)
    Component.onDestruction: {
        if (presentation) {
            presentation.clear();
        }
    }

    title: presentation.value ? presentation.value.title : "Unsaved Workbench"
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(preferredDialogWidth, parent ? Math.max(0, parent.width - viewportEdgeInset * 2) : preferredDialogWidth)
    modal: true
    focus: true
    closePolicy: Popup.CloseOnEscape
    standardButtons: Dialog.NoButton
    contentItem: ColumnLayout {
        spacing: dialog.contentSpacing
        Label {
            objectName: "projectDirtyMessage"
            Layout.fillWidth: true
            text: presentation.value ? presentation.value.message : ""
            textFormat: Text.PlainText
            color: dialog.theme.text
            wrapMode: Text.WordWrap
        }
        Label {
            visible: !dialog.saveAllowed
            Layout.fillWidth: true
            text: presentation.value ? presentation.value.unavailable : ""
            textFormat: Text.PlainText
            color: dialog.theme.muted
            wrapMode: Text.WordWrap
        }
    }
    footer: Pane {
        padding: dialog.footerPadding
        background: Item {}
        contentItem: GridLayout {
            columns: width >= saveButton.implicitWidth + discardButton.implicitWidth + cancelButton.implicitWidth + dialog.actionSpacing * (dialog.rowActionColumns - 1) ? dialog.rowActionColumns : dialog.stackedActionColumns
            rowSpacing: dialog.actionSpacing
            columnSpacing: dialog.actionSpacing
            WorkbenchActionButton {
                id: saveButton
                theme: dialog.theme
                objectName: "projectDirtySave"
                text: presentation.value ? presentation.value.save : ""
                enabled: dialog.decisionPending && dialog.contextCurrent && dialog.saveAllowed
                Layout.fillWidth: true
                Layout.preferredWidth: 1
                Accessible.description: presentation.value ? presentation.value.saveDescription : ""
                onClicked: dialog.resolve("save")
                Keys.onReturnPressed: dialog.resolve("save")
                Keys.onEnterPressed: dialog.resolve("save")
            }
            WorkbenchActionButton {
                id: discardButton
                theme: dialog.theme
                objectName: "projectDirtyDiscard"
                text: presentation.value ? presentation.value.discard : ""
                enabled: dialog.decisionPending && dialog.contextCurrent
                Layout.fillWidth: true
                Layout.preferredWidth: 1
                Accessible.description: presentation.value ? presentation.value.discardDescription : ""
                onClicked: dialog.resolve("discard")
                Keys.onReturnPressed: dialog.resolve("discard")
                Keys.onEnterPressed: dialog.resolve("discard")
            }
            WorkbenchActionButton {
                id: cancelButton
                theme: dialog.theme
                objectName: "projectDirtyCancel"
                text: presentation.value ? presentation.value.cancel : ""
                primary: true
                enabled: dialog.decisionPending
                Layout.fillWidth: true
                Layout.preferredWidth: 1
                Accessible.description: presentation.value ? presentation.value.cancelDescription : ""
                onClicked: dialog.resolve("cancel")
                Keys.onReturnPressed: dialog.resolve("cancel")
                Keys.onEnterPressed: dialog.resolve("cancel")
            }
        }
    }
}
