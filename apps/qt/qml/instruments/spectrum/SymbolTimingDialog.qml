/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"

WorkbenchDialog {
    id: dialog
    objectName: "symbolTimingDialog"
    property var timing: ({})
    readonly property var windowData: timing.window || ({})
    readonly property real maximumWidth: parent ? parent.width-32 : 1080
    readonly property real maximumHeight: parent ? parent.height-32 : 700
    onMaximumWidthChanged: if (opened && maximumWidth>0 && width>maximumWidth) width=maximumWidth
    onMaximumHeightChanged: if (opened && maximumHeight>0 && height>maximumHeight) height=maximumHeight
    onOpened: {
        width=Math.min(maximumWidth,Math.max(640,width))
        height=Math.min(maximumHeight,Math.max(520,height))
    }
    title: "Symbol Timing"
    parent: Overlay.overlay; anchors.centerIn:parent
    width:Math.min(1080,maximumWidth);height:Math.min(700,maximumHeight)
    modal:true;standardButtons:Dialog.Close
    contentItem: ColumnLayout {
        spacing:10
        Label { objectName:"symbolTimingSource";text:dialog.timing.source || "No timing configuration";color:dialog.theme.accent;font.bold:true }
        Label {text:dialog.timing.overview || dialog.timing.error || "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        RowLayout {
            spacing:18
            Label {text:"━ Symbol 1";color:dialog.theme.accent}
            Label {text:"┄ Symbol 2";color:dialog.theme.statusAttention}
            Label {text:"·· Overlap sum";color:dialog.theme.txScatteredPilot}
            Item {Layout.fillWidth:true}
        }
        Label {text:"Full duration · shared time scale";color:dialog.theme.text;font.bold:true}
        SymbolTimingPlot {objectName:"symbolTimingOverview";theme:dialog.theme;graph:dialog.windowData.overview || ({});Layout.fillWidth:true;Layout.fillHeight:true;Layout.minimumHeight:100}
        Label {text:"CP and usable guard · enlarged";color:dialog.theme.text;font.bold:true}
        SymbolTimingPlot {objectName:"symbolTimingOverlap";theme:dialog.theme;graph:dialog.windowData.detail || ({});detail:true;Layout.fillWidth:true;Layout.fillHeight:true;Layout.minimumHeight:100}
        Label {text:dialog.windowData.overlap_label || "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap}
    }
    footer: Pane {
        padding:10
        contentItem:RowLayout {
            Item {Layout.fillWidth:true}
            WorkbenchActionButton {theme:dialog.theme;text:"Close";onClicked:dialog.close()}
            MouseArea {
                objectName:"symbolTimingResize"
                Layout.preferredWidth:22;Layout.preferredHeight:22
                cursorShape:Qt.SizeFDiagCursor
                property point origin
                property real initialWidth
                property real initialHeight
                Accessible.name:"Resize Symbol Timing"
                onPressed:function(mouse) {origin=mapToItem(dialog.parent,mouse.x,mouse.y);initialWidth=dialog.width;initialHeight=dialog.height}
                onPositionChanged:function(mouse) {
                    if (!pressed) return
                    const position=mapToItem(dialog.parent,mouse.x,mouse.y)
                    dialog.width=Math.max(Math.min(640,dialog.maximumWidth),Math.min(dialog.maximumWidth,initialWidth+2*(position.x-origin.x)))
                    dialog.height=Math.max(Math.min(520,dialog.maximumHeight),Math.min(dialog.maximumHeight,initialHeight+2*(position.y-origin.y)))
                }
                Label {anchors.centerIn:parent;text:"◢";color:dialog.theme.muted;font.pixelSize:12}
            }
        }
    }
}
