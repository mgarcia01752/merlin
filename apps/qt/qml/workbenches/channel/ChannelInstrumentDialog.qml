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
    property bool output: false
    signal spectrumRequested()
    signal qamRequested()
    title: output ? "Channel Output · Instruments" : "Channel Input · Instruments"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(440,parent ? parent.width-40 : 440)
    modal:true;standardButtons:Dialog.Close
    contentItem: RowLayout {
        spacing:16
        ReceiverChip { objectName:"channelSpectrum";theme:dialog.theme;text:"Spectrum";onClicked:{dialog.close();dialog.spectrumRequested()} }
        ReceiverChip { objectName:"channelQam";theme:dialog.theme;text:"QAM";onClicked:{dialog.close();dialog.qamRequested()} }
    }
}
