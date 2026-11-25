/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
import "../../shared/theme"

FocusScope {
    id: root
    required property var theme
    required property Component downstreamComponent
    required property Component upstreamComponent
    required property var presentationHost

    readonly property string downstreamDirection: "downstream"
    readonly property string upstreamDirection: "upstream"
    // Presentation spacing/typography, independent of all scientific limits.
    readonly property int frameSpacing: 8
    readonly property int tabSpacing: 4
    readonly property int tabPadding: 12
    readonly property int tabHeight: 42
    readonly property int labelPixelSize: 13
    readonly property int tabCornerRadius: 5
    readonly property int normalBorderWidth: 1
    readonly property int focusedBorderWidth: 2

    readonly property string selectedDirection: presentationHost
        && validDirection(presentationHost.selectedDirection)
        ? presentationHost.selectedDirection : ""
    readonly property bool selectionAvailable: validDirection(selectedDirection)
    readonly property bool scopedActionsAvailable: enabled && selectionAvailable
        && validOpaque(identityFor(selectedDirection))
    readonly property string executionDirection: presentationHost
        && validDirection(presentationHost.executionDirection)
        ? presentationHost.executionDirection : ""
    readonly property bool otherDirectionActive: selectionAvailable
        && validDirection(executionDirection) && executionDirection !== selectedDirection
    readonly property string executionStatus: presentationHost
        && typeof presentationHost.executionStatus === "string"
        ? presentationHost.executionStatus : ""
    readonly property string feedbackMessage: presentationHost
        && typeof presentationHost.feedbackMessage === "string"
        ? presentationHost.feedbackMessage : ""
    readonly property bool feedbackIsError: presentationHost
        ? presentationHost.feedbackIsError === true : false
    readonly property var downstreamItem: downstreamLoader.item
    readonly property var upstreamItem: upstreamLoader.item
    property Item downstreamFocusItem: null
    property Item upstreamFocusItem: null

    signal directionRequested(string direction)
    signal runActionRequested(string action, string direction, var identity)
    signal saveRequested(string direction, var identity, bool saveAs)
    signal openRequested()
    signal openTargetRequested(string direction, var guard)

    implicitWidth: tabRow.implicitWidth
    implicitHeight: content.implicitHeight

    /**
     * Requests presentation selection without changing execution ownership.
     * @param {string} direction Explicit downstream/upstream presentation key.
     * @returns {boolean} Whether a valid direction intent was emitted.
     */
    function requestDirection(direction) {
        if (!validDirection(direction)) {
            return false
        }
        directionRequested(direction)
        restoreSelectedFocus()
        return true
    }

    /**
     * Forwards an action to the selected direction's Rust-owned identity.
     * @param {string} action Existing host action name, such as start or pause.
     * @param {string|undefined} sourceDirection Explicit child origin; omit for global controls.
     * @param {object|string|undefined} sourceIdentity Identity captured with the child intent.
     * @returns {boolean} Whether intent was emitted, not whether execution succeeded.
     */
    function requestRunAction(action, sourceDirection, sourceIdentity) {
        const identity = resolveSourceIdentity(sourceDirection, sourceIdentity)
        if (!scopedActionsAvailable || !validOpaque(identity)
            || typeof action !== "string" || action.length === 0) {
            return false
        }
        runActionRequested(action, selectedDirection, identity)
        return true
    }

    /**
     * Requests Save for the selected direction without serializing any state.
     * @param {boolean} saveAs Whether the host should request a new destination.
     * @param {string|undefined} sourceDirection Explicit child origin; omit for global menus.
     * @param {object|string|undefined} sourceIdentity Identity captured with the child intent.
     * @returns {boolean} Whether scoped intent was emitted.
     */
    function requestSave(saveAs, sourceDirection, sourceIdentity) {
        const identity = resolveSourceIdentity(sourceDirection, sourceIdentity)
        if (!scopedActionsAvailable || !validOpaque(identity)) {
            return false
        }
        saveRequested(selectedDirection, identity, saveAs)
        return true
    }

    /**
     * Requests the host's native Open chooser before a target direction is known.
     * @returns {boolean} Whether enabled untargeted chooser intent was emitted.
     */
    function requestOpen() {
        if (!enabled) {
            return false
        }
        openRequested()
        return true
    }

    /**
     * Forwards a host-detected Open destination and unchanged Rust guard.
     * @param {string} direction Direction detected by public project admission.
     * @param {object|string} guard Opaque lossless target guard, rechecked by Rust.
     * @returns {boolean} Whether explicit target intent was emitted.
     */
    function requestOpenTarget(direction, guard) {
        if (!enabled || !validDirection(direction) || !validOpaque(guard)) {
            return false
        }
        openTargetRequested(direction, guard)
        return true
    }

    /**
     * Checks only presentation direction tags, with no scientific admission.
     * @param {string} direction Candidate presentation key.
     * @returns {boolean} Whether it names one of the two injected views.
     */
    function validDirection(direction) {
        return direction === downstreamDirection || direction === upstreamDirection
    }

    /**
     * Excludes absent or lossy primitive tokens without inspecting identity fields.
     * @param {object|string} value Rust-owned opaque token or Qt value object.
     * @returns {boolean} Whether a nonempty string or nonnull object was supplied.
     */
    function validOpaque(value) {
        return (typeof value === "string" && value.length > 0)
            || (typeof value === "object" && value !== null)
    }

    /**
     * Borrows an identity exactly as supplied by the host without conversion.
     * @param {string} direction Explicit presentation key.
     * @returns {object|string|undefined} The same opaque value, or unavailable.
     */
    function identityFor(direction) {
        if (!presentationHost) {
            return undefined
        }
        if (direction === downstreamDirection) {
            return presentationHost.downstreamIdentity
        }
        if (direction === upstreamDirection) {
            return presentationHost.upstreamIdentity
        }
        return undefined
    }

    /**
     * Checks captured child context or snapshots a global control's current context.
     * @param {string|undefined} direction Child origin, omitted with identity for global intent.
     * @param {object|string|undefined} identity Captured opaque origin, never replaced here.
     * @returns {object|string|undefined} Unchanged current origin, or unavailable/stale.
     */
    function resolveSourceIdentity(direction, identity) {
        if (direction === undefined && identity === undefined) {
            return identityFor(selectedDirection)
        }
        if (direction !== selectedDirection || !validOpaque(identity)
            || identity !== identityFor(direction)) {
            return undefined
        }
        return identity
    }

    /**
     * Tests visual ancestry without depending on a particular injected view type.
     * @param {Item} item Candidate focus item.
     * @param {Item} ancestor Root of a loaded direction view.
     * @returns {boolean} Whether item belongs to that root's visual subtree.
     */
    function belongsTo(item, ancestor) {
        if (!ancestor) {
            return false
        }
        for (let current = item; current; current = current.parent) {
            if (current === ancestor) {
                return true
            }
        }
        return false
    }

    /**
     * Remembers focused child items only in the selected available view.
     * @param {Item} item Current window focus item, possibly null or outside views.
     */
    function rememberFocus(item) {
        if (!scopedActionsAvailable || !item || !item.visible || !item.enabled) {
            return
        }
        if (selectedDirection === downstreamDirection && item !== downstreamItem
            && belongsTo(item, downstreamItem)) {
            downstreamFocusItem = item
        } else if (selectedDirection === upstreamDirection && item !== upstreamItem
            && belongsTo(item, upstreamItem)) {
            upstreamFocusItem = item
        }
    }

    /**
     * Restores the selected child's last valid focus after layout/visibility settles.
     * A later selection invalidates this queued request; hidden children stay disabled.
     */
    function restoreSelectedFocus() {
        const requested = selectedDirection
        Qt.callLater(function() {
            if (!root.scopedActionsAvailable || root.selectedDirection !== requested) {
                return
            }
            const child = requested === root.downstreamDirection
                ? root.downstreamItem : root.upstreamItem
            const remembered = requested === root.downstreamDirection
                ? root.downstreamFocusItem : root.upstreamFocusItem
            if (!child || !child.visible || !child.enabled) {
                return
            }
            if (remembered && remembered.visible && remembered.enabled
                && root.belongsTo(remembered, child)) {
                remembered.forceActiveFocus(Qt.TabFocusReason)
            } else {
                child.forceActiveFocus(Qt.TabFocusReason)
            }
        })
    }

    onSelectedDirectionChanged: restoreSelectedFocus()
    onScopedActionsAvailableChanged: restoreSelectedFocus()
    Component.onCompleted: restoreSelectedFocus()

    Connections {
        target: root.Window.window
        function onActiveFocusItemChanged() {
            if (root.Window.window) {
                root.rememberFocus(root.Window.window.activeFocusItem)
            }
        }
    }

    Rectangle { anchors.fill: parent; color: root.theme.background }
    ColumnLayout {
        id: content
        anchors.fill: parent
        spacing: 0
        Item {
            id: tabStrip
            objectName: "directionTabStrip"
            Layout.fillWidth: true
            implicitHeight: tabRow.implicitHeight
            Rectangle {
                objectName: "directionTabBaseline"
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                height: root.normalBorderWidth
                color: root.theme.line
            }
            RowLayout {
                id: tabRow
                anchors.fill: parent
                anchors.leftMargin: root.frameSpacing
                anchors.rightMargin: root.frameSpacing
                spacing: root.tabSpacing
                Repeater {
                    model: [root.downstreamDirection, root.upstreamDirection]
                    delegate: TabButton {
                        id: tab
                        required property string modelData
                        readonly property bool selected: root.selectedDirection === modelData
                        objectName: modelData + "DirectionTab"
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        Layout.maximumWidth: implicitWidth

                        // Prevent whole-pixel layout rounding from clipping fractional font widths.
                        implicitWidth: Math.ceil(contentItem.implicitWidth) + leftPadding + rightPadding
                        implicitHeight: Math.max(root.tabHeight,
                            contentItem.implicitHeight + topPadding + bottomPadding)
                        leftPadding: root.tabPadding
                        rightPadding: root.tabPadding
                        font.pixelSize: root.labelPixelSize
                        focusPolicy: Qt.StrongFocus
                        autoExclusive: false
                        checked: selected
                        text: modelData === root.downstreamDirection
                            ? "Downstream" : "Upstream"
                        Accessible.name: text
                        ThemePalette { targetControl: tab; theme: root.theme }
                        onClicked: {
                            root.requestDirection(modelData)
                            // The host may accept asynchronously or decline selection.
                            checked = Qt.binding(function() { return tab.selected })
                        }
                        Keys.onLeftPressed: function(event) {
                            root.requestDirection(root.downstreamDirection)
                            event.accepted = true
                        }
                        Keys.onRightPressed: function(event) {
                            root.requestDirection(root.upstreamDirection)
                            event.accepted = true
                        }
                        background: Rectangle {
                            objectName: tab.modelData + "DirectionTabSurface"
                            topLeftRadius: root.tabCornerRadius
                            topRightRadius: root.tabCornerRadius
                            color: tab.selected ? root.theme.background
                                : tab.hovered ? root.theme.button : root.theme.panel
                            border.width: tab.visualFocus
                                ? root.focusedBorderWidth : root.normalBorderWidth
                            border.color: tab.visualFocus
                                ? root.theme.accent : root.theme.line
                            Rectangle {
                                objectName: tab.modelData + "DirectionTabJoin"
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.bottom: parent.bottom
                                height: parent.border.width
                                visible: tab.selected
                                color: parent.color
                            }
                        }
                        contentItem: Label {
                            text: tab.text
                            font: tab.font
                            color: tab.selected ? root.theme.accent : root.theme.text
                            wrapMode: Text.NoWrap
                            elide: Text.ElideRight
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                    }
                }
                Item { Layout.fillWidth: true }
            }
        }
        Label {
            objectName: "otherDirectionExecutionStatus"
            Layout.fillWidth: true
            Layout.leftMargin: root.frameSpacing
            Layout.rightMargin: root.frameSpacing
            Layout.topMargin: root.frameSpacing
            visible: root.otherDirectionActive && text.length > 0
            text: root.executionStatus
            color: root.theme.statusAttention
            wrapMode: Text.WordWrap
            Accessible.name: text
        }
        Label {
            objectName: "directionActionFeedback"
            Layout.fillWidth: true
            Layout.leftMargin: root.frameSpacing
            Layout.rightMargin: root.frameSpacing
            Layout.topMargin: root.frameSpacing
            visible: text.length > 0
            text: root.feedbackMessage
            color: root.feedbackIsError ? root.theme.statusError : root.theme.muted
            wrapMode: Text.WordWrap
            Accessible.name: text
        }
        Label {
            objectName: "directionSelectionUnavailable"
            Layout.fillWidth: true
            visible: !root.selectionAvailable
            text: "Select a direction"
            color: root.theme.muted
            horizontalAlignment: Text.AlignHCenter
        }
        Item {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Loader {
                id: downstreamLoader
                anchors.fill: parent
                active: true
                sourceComponent: root.downstreamComponent
                visible: root.selectedDirection === root.downstreamDirection
                enabled: visible && root.scopedActionsAvailable
                focus: visible && enabled
                onLoaded: root.restoreSelectedFocus()
            }
            Loader {
                id: upstreamLoader
                anchors.fill: parent
                active: true
                sourceComponent: root.upstreamComponent
                visible: root.selectedDirection === root.upstreamDirection
                enabled: visible && root.scopedActionsAvailable
                focus: visible && enabled
                onLoaded: root.restoreSelectedFocus()
            }
        }
    }
}
