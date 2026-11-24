/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
// Adapted from RxMerCollectionTree: virtualized expandable rows, original file
// metadata and explicit selection. The adapter supplies compatibility and order.
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../shared/presentation"
Item {
    id: tree
    required property var theme
    property var files: []
    property int selectedIndex: -1
    property bool selectionEnabled: false
    property bool singleShot: false
    property var selectedCaptures: null
    signal selectionToggled(string identity, bool checked)
    readonly property var expanded: presentation.value ? presentation.value.expanded : ({})
    readonly property var displayedContext: presentation.value ? presentation.value.context : null
    readonly property bool contextCurrent: !!displayedContext
        && displayedContext.files === files
        && displayedContext.selectedIndex === selectedIndex
        && displayedContext.selectionEnabled === selectionEnabled
        && displayedContext.singleShot === singleShot
        && displayedContext.selectedCaptures === selectedCaptures
    property bool presentationReady: false
    signal selected(int index)
    PresentationOwner {
        id: presentation
        theme: tree.theme
        initialValue: ({expanded: {}, rows: [], context: {files: [], selectedIndex: tree.selectedIndex,
            selectionEnabled: false, singleShot: false, selectedCaptures: null}})
    }
    onFilesChanged: scheduleRefresh()
    onSelectedIndexChanged: scheduleRefresh()
    onSelectionEnabledChanged: scheduleRefresh()
    onSingleShotChanged: scheduleRefresh()
    onSelectedCapturesChanged: scheduleRefresh()
    Component.onCompleted: {
        presentationReady = true
        scheduleRefresh()
    }
    Component.onDestruction: {
        presentationReady = false
        if (presentation) {
            presentation.clear()
        }
    }

    // Coalesce related parent bindings before admitting one coherent row context.
    function scheduleRefresh() {
        if (presentationReady) {
            Qt.callLater(tree.refreshPresentation)
        }
    }

    // Keep the actual parent aliases with the admitted rows and index context.
    function rowContext() {
        return {files: files, selectedIndex: selectedIndex, selectionEnabled: selectionEnabled,
            singleShot: singleShot, selectedCaptures: selectedCaptures}
    }
    function refreshPresentation() {
        if (!presentationReady || !presentation) {
            return false
        }
        const sameFiles = displayedContext && displayedContext.files === files
        const nextExpanded = sameFiles ? expanded : ({})
        const nextRows = sameFiles ? presentation.value.rows : rows(files, nextExpanded)
        return presentation.replace({expanded: nextExpanded, rows: nextRows, context: rowContext()})
    }
    function toggleExpanded(index) {
        if (!contextCurrent) {
            return
        }
        const next = Object.assign({}, expanded)
        next[index] = !next[index]
        presentation.replace({expanded: next, rows: rows(files, next), context: rowContext()})
    }
    function captureChecked(identity, single) {
        if (!displayedContext) {
            return false
        }
        if (displayedContext.selectedCaptures === null) {
            return !single || displayedContext.files.length === 1
        }
        return displayedContext.selectedCaptures.indexOf(identity) >= 0
    }
    function rows(sourceFiles = files, expansion = expanded) {
        const rows = []
        for (const file of sourceFiles) {
            rows.push({kind: "file", index: file.index, label: file.name, detail: file.frequency, compatible: file.compatible, identity: file.sha256})
            if (expansion[file.index]) {
                for (const detail of [file.mac + " · channel " + file.channel, file.frequency, (file.synthetic_time ? "Synthetic association epoch " : "Capture epoch ") + file.capture_time, file.format + " · " + file.bytes + " bytes", file.reason, "SHA-256 " + file.sha256]) {
                    rows.push({kind: "detail", index: file.index, label: detail, detail: detail})
                }
            }
        }
        return rows
    }
    ListView {
        id: nodes; objectName: "measurementCaptureTree"
        anchors.fill: parent; anchors.bottomMargin: feedback.visible ? feedback.implicitHeight : 0
        clip: true; model: presentation.value ? presentation.value.rows : []
        ScrollBar.vertical: ScrollBar {}
        delegate: Rectangle {
            required property var modelData
            width: nodes.width; height: modelData.kind === "file" ? 34 : 24
            color: modelData.kind === "file" && tree.displayedContext && tree.displayedContext.selectedIndex === modelData.index ? tree.theme.accentSoft : "transparent"
            RowLayout {
                anchors.fill: parent; spacing: 2
                ToolButton {
                    visible: modelData.kind === "file"; Layout.preferredWidth: 24
                    text: tree.expanded[modelData.index] ? "▾" : "▸"
                    enabled: tree.contextCurrent
                    onClicked: tree.toggleExpanded(modelData.index)
                    Accessible.name: "Expand capture metadata"
                }
                CheckBox {
                    objectName: "captureCheck"+modelData.index
                    visible: !!tree.displayedContext && tree.displayedContext.selectionEnabled && !tree.displayedContext.singleShot && modelData.kind === "file"
                    checked: tree.captureChecked(modelData.identity, false)
                    Accessible.name: "Use capture "+(modelData.index+1)
                    enabled: tree.contextCurrent
                    onClicked: {
                        if (tree.contextCurrent) {
                            tree.selectionToggled(modelData.identity, checked)
                        }
                        checked = Qt.binding(function() { return tree.captureChecked(modelData.identity, false) })
                    }
                }
                RadioButton {
                    objectName: "captureSingle"+modelData.index
                    visible: !!tree.displayedContext && tree.displayedContext.selectionEnabled && tree.displayedContext.singleShot && modelData.kind === "file"
                    autoExclusive:false
                    checked: tree.captureChecked(modelData.identity, true)
                    Accessible.name: "Single Shot capture "+(modelData.index+1)
                    enabled: tree.contextCurrent
                    onClicked: {
                        if (tree.contextCurrent) {
                            tree.selectionToggled(modelData.identity, true)
                        }
                        checked = Qt.binding(function() { return tree.captureChecked(modelData.identity, true) })
                    }
                }
                Label { visible: modelData.kind === "file"; text: "●"; font.pixelSize: 10; color: modelData.compatible === true ? tree.theme.statusComplete : modelData.compatible === false ? tree.theme.statusError : tree.theme.statusAttention }
                ItemDelegate {
                    Layout.fillWidth: true
                    text: modelData.label
                    contentItem: Label { text: parent.text; color: modelData.kind === "file" ? tree.theme.text : tree.theme.muted; font.pixelSize: 11; elide: Text.ElideRight; verticalAlignment: Text.AlignVCenter; leftPadding: modelData.kind === "file" ? 0 : 24 }
                    enabled: tree.contextCurrent
                    onClicked: {
                        if (tree.contextCurrent) {
                            tree.selected(modelData.index)
                        }
                    }
                    ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered; ToolTip.text: modelData.detail || modelData.label
                }
            }
        }
    }
    Label {
        id: feedback
        anchors.left: parent.left; anchors.right: parent.right; anchors.bottom: parent.bottom
        visible: presentation.rejected
        text: presentation.message
        color: tree.theme.statusAttention
        wrapMode: Text.WordWrap
    }
}
