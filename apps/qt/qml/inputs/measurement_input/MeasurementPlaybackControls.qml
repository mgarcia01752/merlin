/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
// Adapted from PNM Viewer RxMerPlaybackControls; original-capture preview only.
RowLayout {
    id: controls
    required property var theme
    property string namePrefix:"rxmerPreview"
    property int frameIndex: 0
    property int frameCount: 0
    property bool suspended: false
    property bool playing: false
    readonly property int framesPerSecond: Number(speed.currentText) || 2
    signal frameRequested(int frameIndex)
    onSuspendedChanged: {
        if (suspended) {
            playing=false
        }
    }
    onFrameCountChanged: playing=false
    onFrameIndexChanged: {
        if (frameIndex>=frameCount-1) {
            playing=false
        }
    }
    function jump(index) {
        playing=false
        if (!suspended && index>=0 && index<frameCount) {
            frameRequested(index)
        }
    }
    Timer {
        interval: 1000/controls.framesPerSecond
        repeat: true
        running: controls.playing && !controls.suspended && controls.frameCount>1
        onTriggered: {
            if (controls.frameIndex<controls.frameCount-1) {
                controls.frameRequested(controls.frameIndex+1)
            } else {
                controls.playing=false
            }
        }
    }
    WorkbenchActionButton {
        objectName: controls.namePrefix+"PreviousFrame"
        Accessible.name: "Previous capture"
        theme: controls.theme
        text: "Previous"
        enabled: !controls.suspended && controls.frameIndex>0
        onClicked: controls.jump(controls.frameIndex-1)
    }
    WorkbenchActionButton {
        objectName: controls.namePrefix+"PlayPause"
        Accessible.name: text+" capture preview"
        theme: controls.theme
        text: controls.playing ? "Pause" : "Play"
        enabled: !controls.suspended && controls.frameCount>1
        primary: controls.playing
        onClicked: {
            if (!controls.playing) {
                controls.frameRequested(controls.frameIndex===controls.frameCount-1 ? 0 : controls.frameIndex)
            }
            controls.playing=!controls.playing
        }
    }
    WorkbenchActionButton {
        objectName: controls.namePrefix+"NextFrame"
        Accessible.name: "Next capture"
        theme: controls.theme
        text: "Next"
        enabled: !controls.suspended && controls.frameIndex<controls.frameCount-1
        onClicked: controls.jump(controls.frameIndex+1)
    }
    Label { text: "Frames/s"; color: controls.theme.muted }
    ComboBox { id: speed; objectName: controls.namePrefix+"PlaybackSpeed"; Accessible.name: "Playback frames per second"; model: ["1","2","4","8"]; currentIndex: 1 }
    Label { objectName:"previewFrameCount"; text: controls.frameCount>0 ? (controls.frameIndex+1)+" / "+controls.frameCount : "—"; color: controls.theme.text }
}
