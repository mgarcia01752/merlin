/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import "../shared/components"

WorkbenchDialog {
    id: dialog
    required property var host
    signal viewRequested(var request, var context)
    signal grantRequested(var grantId, var context)
    signal cellInspected(var cell, var context)
    property alias gridView: resourceView
    readonly property real preferredDialogWidth: 1120
    readonly property real preferredDialogHeight: 820
    readonly property real windowMargin: 24
    title: "Upstream Resource Grid"
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(preferredDialogWidth, parent ? parent.width - windowMargin * 2 : preferredDialogWidth)
    height: Math.min(preferredDialogHeight, parent ? parent.height - windowMargin * 2 : preferredDialogHeight)
    modal: true
    focus: true
    standardButtons: Dialog.Close
    closePolicy: Popup.CloseOnEscape
    background: Rectangle {
        color: dialog.theme.panel
        border.color: dialog.theme.line
    }
    contentItem: ScrollView {
        id: scroller
        clip: true
        contentWidth: availableWidth
        ResourceGridView {
            id: resourceView
            width: scroller.availableWidth
            theme: dialog.theme
            host: dialog.host
            onViewRequested: function (request, context) {
                dialog.viewRequested(request, context);
            }
            onGrantRequested: function (grantId, context) {
                dialog.grantRequested(grantId, context);
            }
            onCellInspected: function (cell, context) {
                dialog.cellInspected(cell, context);
            }
        }
    }
}
