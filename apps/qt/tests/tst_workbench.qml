/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml"

Pane {
    id: window
    width: 1240; height: 780; visible: true
    Theme { id: theme }
    ThemePalette { targetControl: window; theme: theme }
    Workbench {
        id: workbench
        anchors.fill: parent; theme: theme
        iconRoot: "../../../docs/images/logo/"
    }
    TestCase {
        name: "Workbench"
        when: windowShown
        function test_sections_data() {
            return [
                {tag: "ofdm", block: "ofdmBlock", title: "OFDM Settings"},
                {tag: "source", block: "sourceBlock", title: "PRBS-TX"},
                {tag: "tx", block: "transmitterBlock", title: "Transmitter"},
                {tag: "channel", block: "channelBlock", title: "Channel Simulator"},
                {tag: "rx", block: "receiverBlock", title: "Receiver"},
                {tag: "verify", block: "verificationBlock", title: "PRBS-RX"},
                {tag: "profile", block: "profileBlock", title: "Modulation Profile"},
                {tag: "ce", block: "channelEstimateBlock", title: "DS Channel Est."},
                {tag: "mer", block: "rxmerBlock", title: "RxMER"}
            ]
        }
        function test_sections(data) {
            const block = findChild(workbench, data.block)
            verify(block !== null)
            mouseClick(block)
            const dialog = workbench.sectionPopup
            verify(dialog !== null)
            tryCompare(dialog, "opened", true)
            compare(dialog.title, data.title)
            verify(workbench.sectionDescription.length > 30)
            keyClick(Qt.Key_Escape)
            tryCompare(dialog, "visible", false)
        }
        function test_controls_and_theme() {
            for (const name of ["startButton", "stopButton", "pauseButton"]) {
                compare(findChild(workbench, name).enabled, false)
            }
            const previous = theme.dark
            mouseClick(findChild(workbench, "themeToggle"))
            compare(theme.dark, !previous)
            theme.dark = previous
        }
        function test_render() {
            theme.dark = true
            waitForRendering(workbench)
            compare(grabImage(workbench).width, workbench.width)
            wait(100)
            grabImage(workbench).save(Qt.resolvedUrl("../../../docs/development/local/visual-workspace/workbench-dark.png").toString().replace("file://", ""))
            theme.dark = false
            wait(100)
            grabImage(workbench).save(Qt.resolvedUrl("../../../docs/development/local/visual-workspace/workbench-light.png").toString().replace("file://", ""))
            window.width = 960; window.height = 640
            wait(100)
            const block = findChild(workbench, "ofdmBlock")
            mouseClick(block)
            const dialog = workbench.sectionPopup
            tryCompare(dialog, "opened", true)
            grabImage(window.contentItem).save(Qt.resolvedUrl("../../../docs/development/local/visual-workspace/workbench-dialog.png").toString().replace("file://", ""))
            dialog.close()
            tryCompare(dialog, "visible", false)
            window.width = 1240; window.height = 780; theme.dark = true
        }
    }
}
