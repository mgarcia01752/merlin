/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
Dialog {
    required property var theme
    property string message: ""
    title:"MERLIN Workbench File"
    parent:Overlay.overlay; anchors.centerIn:parent
    width:Math.min(650,parent ? parent.width-40 : 650)
    modal:true; standardButtons:Dialog.Close
    contentItem:Label { text:message; wrapMode:Text.WordWrap; color:theme.text }
}
