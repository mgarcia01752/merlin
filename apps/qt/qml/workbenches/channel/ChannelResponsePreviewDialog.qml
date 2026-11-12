/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
import "../../instruments/spectrum"
WorkbenchDialog {
    id:dialog
    property var viewport:({points:[],x_ticks:[],y_ticks:[]})
    signal contextHelpRequested(string code)
    title:"Combined Channel Response"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1060,parent ? parent.width-40 : 1060)
    height:Math.min(650,parent ? parent.height-40 : 650)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        Label {text:"Accepted configuration · before receiver correction";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        SpectrumPlot {theme:dialog.theme;viewport:dialog.viewport;Layout.fillWidth:true;Layout.fillHeight:true}
        ContextualStatus {theme:dialog.theme;text:dialog.viewport.error || "";diagnostic:dialog.viewport.diagnostic || null;Layout.fillWidth:true;onHelpRequested:function(code){dialog.contextHelpRequested(code)}}
    }
}
