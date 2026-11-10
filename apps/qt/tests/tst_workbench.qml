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
    QtObject {
        id: backend
        property string accepted: JSON.stringify({occupied_bandwidth_hz: 192000000, subcarrier_spacing: "25000", cyclic_prefix_samples: 512, roll_off_samples: 128, time_interleaver_depth: 16})
        function registry() {
            return JSON.stringify({fields: [{id: "cyclic_prefix_samples", group: "Timing", label: "Cyclic prefix", choices: [{value: 512, label: "512 samples"}, {value: 1024, label: "1024 samples"}], application_default: 512, description: "Test prefix editor", source: {document: "Test metadata", section: "test"}}], profile_fields: [{id: "profile_id", label: "Profile ID", description: "No profile selected"}, {id: "source_file", label: "Source file", description: "No source selected"}, {id: "subcarrier_assignments", label: "Subcarrier assignments", description: "No assignments selected"}], default_note: "Test adapter"})
        }
        function current() { return accepted }
        function defaults() { return accepted }
        function describe(document) { return JSON.stringify({summary: "Timing tested in Rust"}) }
        function apply(document) { accepted = document; return "" }
    }
    QtObject {
        id: profileBackend
        property int acceptedId: -1
        property bool loaded: false
        function view(id) {
            return JSON.stringify({source_name: loaded ? "test.bin" : "", format: loaded ? "Binary PNM" : "", profile_ids: loaded ? [0, 1] : [], selected_profile_id: loaded ? id : -1, rows: loaded ? [{key: "profiles", label: "Profiles", value: "2 available", depth: 0, ancestors: [], branch: true, description: "Test capture"}, {key: "profile_0", label: "Profile 0", value: "Test assignments", depth: 1, ancestors: ["profiles"], branch: true, profile_id: 0, description: "Test profile"}] : []})
        }
        function begin() { loaded = acceptedId >= 0; return view(acceptedId) }
        function demo() { loaded = true; return view(0) }
        function preview(id) { return view(id) }
        function stage(url) { return JSON.stringify({error: "Invalid test input"}) }
        function clear_draft() { loaded = false; return view(-1) }
        function apply(id, settings) { acceptedId = loaded ? id : -1; return "" }
        function cancel() { loaded = acceptedId >= 0 }
        function state(settings) {
            return JSON.stringify({selected: acceptedId >= 0, profile_status: acceptedId >= 0 ? "complete" : "attention", ofdm_status: acceptedId >= 0 ? "complete" : "attention", reason: acceptedId >= 0 ? "Profile and settings valid" : "A validated modulation-profile input is required.", summary: acceptedId >= 0 ? {profile_id: String(acceptedId), source_file: "test.bin", subcarrier_assignments: "Test assignments"} : {}})
        }
    }
    Workbench {
        id: workbench
        anchors.fill: parent; theme: theme; settingsBackend: backend; profileBackend: profileBackend
        iconRoot: "../../../docs/images/logo/"
    }
    Component {
        id: statusBlockComponent
        WorkbenchBlock { width: 170; height: 90 }
    }
    TestCase {
        name: "Workbench"
        when: windowShown
        function test_sections_data() {
            return [
                {tag: "ofdm", block: "ofdmBlock", title: "OFDM Channel Descriptor (OCD)"},
                {tag: "source", block: "sourceBlock", title: "PRBS-TX"},
                {tag: "tx", block: "transmitterBlock", title: "Transmitter"},
                {tag: "channel", block: "channelBlock", title: "Channel Simulator"},
                {tag: "rx", block: "receiverBlock", title: "Receiver"},
                {tag: "verify", block: "verificationBlock", title: "PRBS-RX"},
                {tag: "profile", block: "profileBlock", title: "Downstream Profile Descriptor (DPD)"},
                {tag: "ce", block: "channelEstimateBlock", title: "DS Channel Est."},
                {tag: "mer", block: "rxmerBlock", title: "RxMER"}
            ]
        }
        function test_sections(data) {
            const block = findChild(workbench, data.block)
            verify(block !== null)
            mouseClick(block)
            const dialog = data.block === "ofdmBlock" ? workbench.ofdmPopup : data.block === "profileBlock" ? workbench.profilePopup : workbench.sectionPopup
            verify(dialog !== null)
            tryCompare(dialog, "opened", true)
            compare(dialog.title, data.title)
            if (data.block !== "ofdmBlock" && data.block !== "profileBlock") { verify(workbench.sectionDescription.length > 30) }
            keyClick(Qt.Key_Escape)
            tryCompare(dialog, "visible", false)
        }
        function test_ofdm_cancel_and_reopen() {
            const dialog = workbench.ofdmPopup
            const original = backend.current()
            mouseClick(findChild(workbench, "ofdmBlock"))
            tryCompare(dialog, "opened", true)
            tryVerify(() => findChild(dialog.contentItem, "cyclic_prefix_samplesEditor") !== null)
            const editor = findChild(dialog.contentItem, "cyclic_prefix_samplesEditor")
            verify(editor !== null)
            editor.forceActiveFocus()
            keyClick(Qt.Key_Down)
            compare(dialog.draft.cyclic_prefix_samples, 1024)
            keyClick(Qt.Key_Escape)
            tryCompare(dialog, "visible", false)
            compare(backend.current(), original)
            mouseClick(findChild(workbench, "ofdmBlock"))
            tryCompare(dialog, "opened", true)
            compare(dialog.draft.cyclic_prefix_samples, JSON.parse(original).cyclic_prefix_samples)
            dialog.draft = Object.assign({}, dialog.draft, {cyclic_prefix_samples: 1024})
            mouseClick(findChild(dialog, "ofdmApply"))
            tryCompare(dialog, "visible", false)
            compare(JSON.parse(backend.current()).cyclic_prefix_samples, 1024)
            compare(findChild(workbench, "ofdmBlock").blockStatus, "attention")
            verify(findChild(workbench, "ofdmBlock").statusReason.indexOf("modulation-profile") >= 0)
            backend.accepted = original
        }
        function test_ofdm_tree_and_blank_profile() {
            const dialog = workbench.ofdmPopup
            mouseClick(findChild(workbench, "ofdmBlock"))
            tryCompare(dialog, "opened", true)
            for (const id of ["profile_id", "source_file", "subcarrier_assignments"]) {
                tryVerify(() => findChild(dialog.contentItem, id + "Value") !== null)
                compare(findChild(dialog.contentItem, id + "Value").text, "")
            }
            compare(findChild(dialog.contentItem, "occupied_bandwidth_hzEditor"), null)
            compare(findChild(dialog.contentItem, "subcarrier_spacingEditor"), null)
            const branch = findChild(dialog.contentItem, "TimingBranch")
            const editor = findChild(dialog.contentItem, "cyclic_prefix_samplesEditor")
            mouseClick(branch)
            compare(editor.visible, false)
            mouseClick(branch)
            compare(editor.visible, true)
            dialog.close()
            tryCompare(dialog, "visible", false)
        }
        function test_block_readiness_colors_and_reasons() {
            const block = createTemporaryObject(statusBlockComponent, window, {
                theme: theme, heading: "Status test", blockStatus: "attention", statusReason: "Missing required input"
            })
            verify(block !== null)
            compare(findChild(block, "statusStripe").color, theme.statusAttention)
            compare(block.Accessible.description, "Needs attention: Missing required input")
            block.blockStatus = "complete"
            block.statusReason = "Required input accepted"
            compare(findChild(block, "statusStripe").color, theme.statusComplete)
            compare(block.Accessible.description, "Complete: Required input accepted")
            block.blockStatus = "error"
            block.statusReason = "Input validation failed"
            compare(findChild(block, "statusStripe").color, theme.statusError)
            compare(block.Accessible.description, "Error: Input validation failed")
            theme.dark = false
            compare(findChild(block, "statusStripe").color, theme.statusError)
            theme.dark = true
        }
        function test_preview_blocks_need_attention() {
            for (const name of ["ofdmBlock", "sourceBlock", "transmitterBlock", "channelBlock", "receiverBlock", "verificationBlock", "profileBlock", "channelEstimateBlock", "rxmerBlock"]) {
                const block = findChild(workbench, name)
                compare(block.blockStatus, "attention")
                compare(findChild(block, "statusStripe").color, theme.statusAttention)
                verify(block.statusReason.length > 20)
                verify(block.Accessible.description.indexOf(block.statusReason) >= 0)
            }
        }
        function test_start_requires_every_block_complete() {
            const blocks = workbench.diagramBlocks
            const previous = blocks.map(block => block.blockStatus)
            try {
                for (const block of blocks) { block.blockStatus = "complete" }
                compare(workbench.allBlocksComplete, true)
                // Readiness must not enable a button with no run dispatch.
                compare(workbench.runIntegrationAvailable, false)
                compare(findChild(workbench, "startButton").enabled, false)
                for (const block of blocks) {
                    for (const status of ["attention", "error"]) {
                        block.blockStatus = status
                        compare(workbench.allBlocksComplete, false)
                        compare(findChild(workbench, "startButton").enabled, false)
                    }
                    block.blockStatus = "complete"
                    compare(workbench.allBlocksComplete, true)
                }
            } finally {
                for (let index = 0; index < blocks.length; ++index) { blocks[index].blockStatus = previous[index] }
            }
        }
        function test_profile_staging_apply_cancel_and_remove() {
            const dialog = workbench.profilePopup
            mouseClick(findChild(workbench, "profileBlock"))
            tryCompare(dialog, "opened", true)
            compare(dialog.previewModel.profile_ids.length, 0)
            mouseClick(findChild(dialog, "profileLoadDemo"))
            compare(dialog.previewModel.format, "Binary PNM")
            compare(profileBackend.acceptedId, -1)
            keyClick(Qt.Key_Escape)
            tryCompare(dialog, "visible", false)
            compare(profileBackend.acceptedId, -1)
            mouseClick(findChild(workbench, "profileBlock"))
            tryCompare(dialog, "opened", true)
            mouseClick(findChild(dialog, "profileLoadDemo"))
            mouseClick(findChild(dialog, "profileApply"))
            tryCompare(dialog, "visible", false)
            compare(profileBackend.acceptedId, 0)
            compare(workbench.profileState.profile_status, "complete")
            compare(workbench.profileState.ofdm_status, "complete")
            compare(findChild(workbench, "startButton").enabled, false)
            mouseClick(findChild(workbench, "profileBlock"))
            tryCompare(dialog, "opened", true)
            dialog.showResult(profileBackend.stage("file:///bad.bin"))
            verify(dialog.errorMessage.length > 0)
            compare(dialog.previewModel.selected_profile_id, 0)
            mouseClick(findChild(dialog, "profileRemove"))
            compare(profileBackend.acceptedId, 0)
            mouseClick(findChild(dialog, "profileApply"))
            tryCompare(dialog, "visible", false)
            compare(profileBackend.acceptedId, -1)
            compare(workbench.profileState.profile_status, "attention")
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
            const dialog = workbench.ofdmPopup
            tryCompare(dialog, "opened", true)
            grabImage(window.contentItem).save(Qt.resolvedUrl("../../../docs/development/local/visual-workspace/workbench-dialog.png").toString().replace("file://", ""))
            dialog.close()
            tryCompare(dialog, "visible", false)
            window.width = 1240; window.height = 780; theme.dark = true
        }
    }
}
