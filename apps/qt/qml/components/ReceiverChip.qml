/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

WorkbenchActionButton {
    id: chip
    implicitWidth: 140
    Layout.fillWidth: true
    Layout.minimumWidth: 0
    Layout.preferredWidth: 1
    contentItem: Label {
        text: chip.text
        font: chip.font
        color: chip.enabled ? chip.theme.text : chip.theme.muted
        opacity: chip.enabled ? 1 : 0.55
        wrapMode: Text.WordWrap
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
}
