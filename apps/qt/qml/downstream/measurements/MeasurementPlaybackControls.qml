/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../shared/components"
import "../../shared/presentation"
// Adapted from PNM Viewer RxMerPlaybackControls; original-capture preview only.
RowLayout {
    id: controls
    required property var theme
    property string namePrefix:"rxmerPreview"
    property int frameIndex: 0
    property int frameCount: 0
    property bool suspended: false
    property bool playing: false
    property bool presentationReady: false
    readonly property bool labelsCurrent: !!presentation.value
        && presentation.value.frameIndex === frameIndex && presentation.value.frameCount === frameCount
    readonly property bool effectiveSuspended: suspended || !labelsCurrent || presentation.rejected
    PresentationOwner {
        id: presentation
        theme: controls.theme
        initialValue: ({speeds: controls.speedChoices(), frameIndex: controls.frameIndex,
            frameCount: controls.frameCount, frameText: controls.frameText()})
    }
    Component.onCompleted: {
        presentationReady = true
        Qt.callLater(controls.refreshLabels)
    }
    Component.onDestruction: {
        presentationReady = false
        if (presentation) {
            presentation.clear()
        }
    }
    function speedChoices() { return ["1","2","4","8"] }
    function frameText() { return frameCount>0 ? (frameIndex+1)+" / "+frameCount : "—" }
    function refreshLabels() {
        if (!presentationReady || !presentation) {
            return false
        }
        const accepted = presentation.replace({speeds: presentation.value ? presentation.value.speeds : speedChoices(),
            frameIndex: frameIndex, frameCount: frameCount, frameText: frameText()})
        if (!accepted) {
            playing = false
        }
        return accepted
    }
    readonly property int framesPerSecond: Number(speed.currentText) || 2
    signal frameRequested(int frameIndex)
    onSuspendedChanged: {
        if (suspended) {
            playing=false
        }
    }
    onFrameCountChanged: {
        playing = false
        refreshLabels()
    }
    onFrameIndexChanged: {
        refreshLabels()
        if (frameIndex>=frameCount-1) {
            playing=false
        }
    }
    function jump(index) {
        playing=false
        if (!effectiveSuspended && index>=0 && index<frameCount) {
            frameRequested(index)
        }
    }
    Timer {
        interval: 1000/controls.framesPerSecond
        repeat: true
        running: controls.playing && !controls.effectiveSuspended && controls.frameCount>1
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
        enabled: !controls.effectiveSuspended && controls.frameIndex>0
        onClicked: controls.jump(controls.frameIndex-1)
    }
    WorkbenchActionButton {
        objectName: controls.namePrefix+"PlayPause"
        Accessible.name: text+" capture preview"
        theme: controls.theme
        text: controls.playing ? "Pause" : "Play"
        enabled: !controls.effectiveSuspended && controls.frameCount>1
        primary: controls.playing
        onClicked: {
            if (!controls.playing) {
                controls.frameRequested(controls.frameIndex===controls.frameCount-1 ? 0 : controls.frameIndex)
            }
            if (!controls.effectiveSuspended) {
                controls.playing = !controls.playing
            }
        }
    }
    WorkbenchActionButton {
        objectName: controls.namePrefix+"NextFrame"
        Accessible.name: "Next capture"
        theme: controls.theme
        text: "Next"
        enabled: !controls.effectiveSuspended && controls.frameIndex<controls.frameCount-1
        onClicked: controls.jump(controls.frameIndex+1)
    }
    Label { text: "Frames/s"; color: controls.theme.muted }
    ComboBox { id: speed; objectName: controls.namePrefix+"PlaybackSpeed"; Accessible.name: "Playback frames per second"; model: presentation.value ? presentation.value.speeds : []; currentIndex: 1 }
    Label { objectName:"previewFrameCount"; text: presentation.rejected ? presentation.message : presentation.value ? presentation.value.frameText : ""; color: controls.theme.text }
}
