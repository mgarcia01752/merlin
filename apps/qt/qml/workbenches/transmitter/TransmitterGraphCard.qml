/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import "../../components"
import "../../instruments/stages"

WorkbenchActionButton {
    id: card
    property var observation: ({available:false})
    property string caption: ""
    property bool spectrum: false
    property var spectrumView: ({points:[]})
    Accessible.name:text+" · "+caption
    ToolTip.visible:hovered || visualFocus
    ToolTip.text:observation.available ? "Open "+text : "Open "+text+" · Use Start to capture transmitter stages"
    contentItem: Column {
        spacing:4
        Label { objectName:"txGraphTitle";width:parent.width;text:card.text;color:card.theme.text;font.bold:true;font.pixelSize:12;minimumPixelSize:8;fontSizeMode:Text.HorizontalFit;wrapMode:Text.NoWrap;horizontalAlignment:Text.AlignHCenter }
        TransmitterStagePlot { width:parent.width;height:67;theme:card.theme;miniature:true;observation:card.observation;graph:card.spectrum ? {kind:"spectrum",points:card.spectrumView.points || []} : card.observation.graph || ({}) }
        Label { width:parent.width;text:card.caption;color:card.theme.muted;font.pixelSize:10;horizontalAlignment:Text.AlignHCenter;elide:Text.ElideRight }
    }
}
