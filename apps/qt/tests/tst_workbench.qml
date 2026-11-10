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
        function describe(document) { return JSON.stringify({summary: "Timing configuration"}) }
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
    QtObject {
        id: sourceBackend
        property bool acceptedSource: false
        property var acceptedOptions: ({mode: "prbs15", seed: 32767, append: false, target_bits: 0, fill_seed: 32767})
        property bool fileSelected: false
        property bool acceptedFile: false
        function fileView() { return {selected: fileSelected, name: fileSelected ? "test.bin" : "", bytes: fileSelected ? 2 : null, bits: fileSelected ? 16 : null, sha256: fileSelected ? "test-hash" : ""} }
        function begin() { fileSelected = acceptedFile; return JSON.stringify({options: acceptedOptions, file: fileView()}) }
        function choose_file() { fileSelected = true; return JSON.stringify(fileView()) }
        function describe(document) {
            const options = JSON.parse(document)
            let error = ""
            if (options.mode === "prbs15" && (options.seed < 1 || options.seed > 32767)) { error = "Invalid PRBS15 seed" }
            if (options.mode === "file" && !fileSelected) { error = "Select a file" }
            if (options.mode === "file" && options.append && (options.target_bits < 16 || options.target_bits > 134217728 || options.fill_seed < 1 || options.fill_seed > 32767)) { error = "Invalid fill" }
            return JSON.stringify({error: error, fill_bits: error ? null : options.mode === "file" && options.append ? options.target_bits - 16 : 0})
        }
        function apply(document) {
            const result = JSON.parse(describe(document))
            if (result.error) { return result.error }
            acceptedOptions = JSON.parse(document); acceptedSource = true
            acceptedFile = acceptedOptions.mode === "file" && fileSelected
            return ""
        }
        function cancel() { fileSelected = acceptedFile }
        function state() { return JSON.stringify({status: acceptedSource ? "complete" : "attention", reason: acceptedSource ? "Validated source accepted for the next run" : "Apply a validated PRBS15 or file source before starting."}) }
    }
    QtObject {
        id: transmitterBackend
        property bool busy: false
        property bool resultAvailable: false
        property string token: "0"
        property bool ready: false
        function configure(settings) { ready = sourceBackend.acceptedSource && profileBackend.acceptedId === 0; resultAvailable = false; return poll() }
        function generate() { if (ready) { busy = true } }
        function cancel() { busy = false }
        function finish() { busy = false; resultAvailable = true; token = String(Number(token) + 1) }
        function poll() { return JSON.stringify({status: ready ? "complete" : "attention", reason: ready ? "Accepted source, profile and settings form a transmitter" : "Accept compatible OCD, DPD and PRBS-TX inputs to configure the transmitter.", tap_name: "TX Output", tap_id: "tx.overlap_add.samples", tap_description: "Complex baseband I/Q before channel", busy: busy, can_generate: ready && !busy, has_result: resultAvailable, result_token: token, progress: busy ? 0.5 : 0, message: ""}) }
        function timing(velocity) {
            if (!resultAvailable) { return JSON.stringify({points:[],ticks:[],error:"Run simulation"}) }
            return JSON.stringify({points:[{x:0,coefficient:0},{x:0.2,coefficient:1},{x:1,coefficient:1}],ticks:[{x:0,label:"0"},{x:1,label:"3.125 µs"}],prefix_fraction:0.8,roll_off_fraction:0.2,overview:"CP 512 · FFT 8192 samples",roll_label:"δ 0.625 µs",guard_label:"Flat guard 1.875 µs",budget_label:"Ideal guard budget: 1.875 µs",extra_path:isNaN(velocity) ? "Unknown VF" : velocity === 0.87 ? "1604 ft" : "1512 ft",reflection_spacing:isNaN(velocity) ? "Unknown VF" : velocity === 0.87 ? "802 ft" : "756 ft"})
        }
        function view(start, count, captureAxis) {
            return JSON.stringify({points: [{x:0,y:0.7,minimum_y:0.8,readout:"Test low bin"},{x:0.5,y:0.2,minimum_y:0.4,readout:"Test center bin"},{x:1,y:0.6,minimum_y:0.9,readout:"Test high bin"}],x_ticks:[{x:0,label:captureAxis ? "631.100" : "-102.400"},{x:1,label:captureAxis ? "835.875" : "102.375"}],y_ticks:[{y:0,label:"-60"},{y:1,label:"-160"}],start:start,count:count || 8192,total:8192,axis_label:"Normalized PSD",frequency_label:captureAxis ? "Capture mapping" : "Baseband frequency offset (MHz)",summary:"Test FFT metadata",detail:"Bounded fixture",trace_label:"Test envelope"})
        }
    }
    QtObject {
        id: measurementBackend
        property bool acceptedRx: false
        property bool draftRx: false
        property bool flat: true
        property bool draftFlat: true
        function fileRows() { return draftRx ? [{index:0,name:"rxmer.bin",format:"Binary PNM",bytes:7508,capture_time:1380970,mac:"00:ca:00:00:00:01",channel:34,frequency:"640–827 MHz",sha256:"test",compatible:profileBackend.acceptedId >= 0,reason:"Test exact grid"}] : [] }
        function metadata(rxmer) { return JSON.stringify({files:rxmer ? fileRows() : [],flat:draftFlat,options:{mode:"single",interval_symbols:"1000",tolerance_seconds:90000}}) }
        function begin(rxmer) { if (rxmer) { draftRx = acceptedRx } else { draftFlat = flat }; return metadata(rxmer) }
        function choose_files(rxmer) { return demo(rxmer,0) }
        function demo(rxmer,variant) { if (rxmer) { draftRx = true } else { draftFlat = false }; return metadata(rxmer) }
        function edit(rxmer,index,delta) { if (rxmer) { draftRx = false } else { draftFlat = true }; return metadata(rxmer) }
        function describe(rxmer,options,settings) { return JSON.stringify({error:profileBackend.acceptedId < 0 ? "Apply DPD" : rxmer && !draftRx ? "Choose RxMER" : "",summary:"Interval 42.500 ms · final capture held"}) }
        function view(rxmer,request,settings) {
            const ready = profileBackend.acceptedId >= 0 && (!rxmer || draftRx)
            const selection = JSON.parse(request)
            return JSON.stringify({points:ready ? [{x:0,y:0.5,minimum_y:0.5,gap:false,readout:"Test"},{x:1,y:0.5,minimum_y:0.5,gap:false,readout:"Test"}] : [],x_ticks:[],y_ticks:[],total:7480,start:selection.start,count:selection.count || 7480,axis_label:rxmer ? "RxMER (dB)" : "Channel gain (dB)",summary:"Test capture"})
        }
        function apply(rxmer,options,settings) {
            const error=JSON.parse(describe(rxmer,options,settings)).error
            if (error) { return error }
            if (rxmer) { acceptedRx=draftRx } else { flat=draftFlat }
            return ""
        }
        function state(settings) {
            const response=profileBackend.acceptedId >= 0
            const rx=response && acceptedRx
            return JSON.stringify({rxmer:{status:rx ? "complete":"attention",reason:rx ? "RxMER configured and accepted":"Apply compatible RxMER inputs before running."},response:{status:response ? "complete":"attention",reason:response ? "Flat/unity channel configured":"Apply compatible DPD to define the channel grid."},channel:{status:rx ? "complete":"attention",reason:rx ? "Both channel inputs configured":"Configure both inputs"},flat:flat})
        }
    }
    QtObject {
        id: aboutBackend
        function version() { return "0.1.0" }
        function license_document(key) { return JSON.stringify({title:key === "gpl" ? "GNU General Public License v3" : "Apache License 2.0",text:key === "gpl" ? "GNU GENERAL PUBLIC LICENSE" : "Apache License"}) }
    }
    QtObject {
        id: receiverBackend
        property bool retainedCells:false
        property var lastQuery: ({})
        property bool lastDecoderInput:false
        function qam_selected(document) {
            lastQuery=JSON.parse(document)
            if (!retainedCells) { return qam(lastQuery.signal,lastQuery.view,lastQuery.part) }
            const available=(lastQuery.first_bin<0 || lastQuery.first_bin<=320) && (lastQuery.last_bin<0 || lastQuery.last_bin>=320)
            return JSON.stringify({available:available,reason:available ? "Actual test observations" : "No cells match",points:available ? [{raw_x:0.4,raw_y:0.4,eq_x:0.5,eq_y:0.5}] : [],profiles:[0],orders:[256],first_bin:320,last_bin:320,first_symbol:10,last_symbol:10,retained:1,detail:"Retained test result"})
        }
        function response(document) { lastQuery=JSON.parse(document);return JSON.stringify({available:retainedCells,points:[],x_ticks:[],y_ticks:[],snapshots:[10],reason:"Response test",summary:"Actual response query"}) }
        function slicer(document,decoderInput) { lastQuery=JSON.parse(document);lastDecoderInput=decoderInput;return JSON.stringify({available:retainedCells,rows:[],llr:{count:retainedCells ? 8 : 0},histogram:{points:[],x_ticks:[],y_ticks:[]},detail:"Actual soft query"}) }
        function fec() {return JSON.stringify({available:retainedCells,status:"complete",words:[],verification_charts:{recovery:{points:[]},word_errors:{points:[]},omitted:0,uncompared_retained:0},histogram:{points:[],x_ticks:[],y_ticks:[]}})}
        function qam(signal, view, part) { return JSON.stringify({available:false,reason:"No retained receiver results",detail:signal === 1 ? "Matched received NCP I/Q collection is pending" : "Receiver collector not connected",points:[],profiles:[],raw_tap:"rx.fft.bins",equalized_tap:"rx.equalized.iq"}) }
        function spectrum(tap) { return JSON.stringify({tap_name:["RX Input / Channel Output","RX Framed Samples","RX Useful Samples","RX FFT Bin Power"][tap],tap_id:["channel.output","rx.framed.samples","rx.cp.samples","rx.fft.bins"][tap],tap_description:"Selected receiver test point",reason:"No retained receiver results",has_result:false,result_token:"",can_generate:false,busy:false}) }
        function cancel() {}
    }
    QtObject {
        id: runBackend
        property bool configured:false
        property bool verified:false
        property bool busy:false
        property bool paused:false
        property var lastResult: null
        function data() { return {receiver_status:configured ? "complete":"attention",verify_status:verified ? "complete":"attention",reason:"Configure accepted receiver and independent source comparison before running.",busy:busy,paused:paused,can_start:configured&&verified&&!busy,has_result:!!lastResult,result:lastResult,result_token:lastResult ? "1":"0",options:{receiver_mode:"supplied_channel",duration_ms:3,max_iterations:50,max_runtime_ms:300000,noise_seed:41},verified:verified} }
        function configure(settings) { return JSON.stringify(data()) }
        function poll() { return JSON.stringify(data()) }
        function apply(options,settings) { configured=true;return "" }
        function verify() { verified=true }
        function start() { busy=true }
        function stop() { busy=false;paused=false }
        function pause() { paused=!paused }
        function restore(options,verify,settings) { configured=options!=="null";verified=verify }
    }
    QtObject {
        id: fileBackend
        function open() { return JSON.stringify({cancelled:true}) }
        function save(saveAs,settings,receiver,verified,dark) { return JSON.stringify({cancelled:true}) }
    }
    WorkbenchMenuBar {
        id: appMenu
        anchors.top: parent.top; width: parent.width; theme: theme
        onGlobalSettingsRequested: workbench.settingsPopup.open()
        onAboutRequested: workbench.aboutPopup.open()
    }
    Workbench {
        id: workbench
        anchors { top: appMenu.bottom; bottom: parent.bottom; left: parent.left; right: parent.right }
        theme: theme; settingsBackend: backend; profileBackend: profileBackend; sourceBackend: sourceBackend; transmitterBackend: transmitterBackend; measurementBackend: measurementBackend; aboutBackend: aboutBackend; receiverBackend: receiverBackend; runBackend: runBackend; fileBackend: fileBackend
        iconRoot: "../../../docs/images/logo/"
    }
    Component {
        id: statusBlockComponent
        WorkbenchBlock { width: 170; height: 90 }
    }
    Component {
        id: gatingWorkbenchComponent
        Workbench { width: 1240; height: 780; visible: false }
    }
    TestCase {
        name: "Workbench"
        when: windowShown
        function closeWithX(dialog) {
            const button = findChild(dialog.header, "popupCloseButton")
            verify(button !== null)
            verify(button.visible && button.enabled)
            compare(button.Accessible.name, "Close " + dialog.title)
            mouseClick(button)
            tryCompare(dialog, "visible", false)
        }
        function test_sections_data() {
            return [
                {tag: "ofdm", block: "ofdmBlock", title: "OFDM Channel Descriptor (OCD)"},
                {tag: "source", block: "sourceBlock", title: "Transmit Source (PRBS-TX)"},
                {tag: "tx", block: "transmitterBlock", title: "Spectrum Analyzer · TX Output"},
                {tag: "channel", block: "channelBlock", title: "Channel Simulator · Configuration"},
                {tag: "rx", block: "receiverBlock", title: "Receiver Workbench"},
                {tag: "verify", block: "verificationBlock", title: "PRBS-RX / Recovered Data"},
                {tag: "profile", block: "profileBlock", title: "Downstream Profile Descriptor (DPD)"},
                {tag: "ce", block: "channelEstimateBlock", title: "DS Channel Estimate · Response Configuration"},
                {tag: "mer", block: "rxmerBlock", title: "RxMER · Capture Configuration"}
            ]
        }
        function test_sections(data) {
            const block = findChild(workbench, data.block)
            verify(block !== null)
            mouseClick(block)
            const dialog = data.block === "ofdmBlock" ? workbench.ofdmPopup : data.block === "profileBlock" ? workbench.profilePopup : data.block === "sourceBlock" ? workbench.sourcePopup : data.block === "transmitterBlock" ? workbench.transmitterPopup : data.block === "channelBlock" ? workbench.channelPopup : data.block === "channelEstimateBlock" ? workbench.responsePopup : data.block === "rxmerBlock" ? workbench.rxmerPopup : data.block === "receiverBlock" ? workbench.receiverPopup : data.block === "verificationBlock" ? workbench.verificationPopup : workbench.sectionPopup
            verify(dialog !== null)
            tryCompare(dialog, "opened", true)
            compare(dialog.title, data.title)
            if (data.block !== "ofdmBlock" && data.block !== "profileBlock" && data.block !== "sourceBlock" && data.block !== "transmitterBlock" && data.block !== "channelBlock" && data.block !== "channelEstimateBlock" && data.block !== "rxmerBlock" && data.block !== "receiverBlock" && data.block !== "verificationBlock") { verify(workbench.sectionDescription.length > 30) }
            closeWithX(dialog)
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
            closeWithX(dialog)
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
            waitForRendering(branch)
            wait(100)
            mouseClick(branch)
            tryCompare(editor, "visible", false)
            mouseClick(branch)
            tryCompare(editor, "visible", true)
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
            const isolated = createTemporaryObject(gatingWorkbenchComponent, window, {
                theme: theme, settingsBackend: backend, profileBackend: profileBackend,
                sourceBackend: sourceBackend, transmitterBackend: transmitterBackend, measurementBackend: measurementBackend, aboutBackend: aboutBackend, receiverBackend: receiverBackend, runBackend: runBackend, fileBackend: fileBackend,
                iconRoot: "../../../docs/images/logo/"
            })
            verify(isolated !== null)
            const blocks = isolated.diagramBlocks
            const previous = blocks.map(block => block.blockStatus)
            try {
                for (const block of blocks) { block.blockStatus = "complete" }
                compare(isolated.allBlocksComplete, true)
                // All blocks still require a valid frozen request and idle worker.
                compare(isolated.runIntegrationAvailable, true)
                compare(findChild(isolated, "startButton").enabled, false)
                for (const block of blocks) {
                    for (const status of ["attention", "error"]) {
                        block.blockStatus = status
                        compare(isolated.allBlocksComplete, false)
                        compare(findChild(isolated, "startButton").enabled, false)
                    }
                    block.blockStatus = "complete"
                    compare(isolated.allBlocksComplete, true)
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
            compare(findChild(workbench, "themeToggle"), null)
            mouseClick(appMenu.itemAt(0))
            compare(findChild(appMenu, "openWorkbenchConfig").enabled, true)
            compare(findChild(appMenu, "saveWorkbenchConfig").enabled, true)
            keyClick(Qt.Key_Escape)
            mouseClick(appMenu.itemAt(1))
            mouseClick(findChild(appMenu, "globalSettingsAction"))
            tryCompare(workbench.settingsPopup, "opened", true)
            const mode = findChild(workbench.settingsPopup, "globalThemeMode")
            mode.currentIndex = previous ? 1 : 0
            mode.activated(mode.currentIndex)
            compare(theme.dark, !previous)
            keyClick(Qt.Key_Escape)
            tryCompare(workbench.settingsPopup, "visible", false)
            theme.dark = previous
            mouseClick(appMenu.itemAt(2))
            mouseClick(findChild(appMenu, "aboutAction"))
            tryCompare(workbench.aboutPopup, "opened", true)
            compare(workbench.aboutPopup.title, "About MERLIN")
            verify(findChild(workbench.aboutPopup, "aboutIcon") !== null)
            compare(findChild(workbench.aboutPopup, "aboutVersion").text, "Version 0.1.0")
            workbench.aboutPopup.licensesPopup.showDocument("gpl")
            tryCompare(workbench.aboutPopup.licensesPopup, "opened", true)
            verify(findChild(workbench.aboutPopup.licensesPopup, "licenseDocumentText").text.indexOf("GNU") >= 0)
            closeWithX(workbench.aboutPopup.licensesPopup)
            compare(workbench.aboutPopup.opened, true)
            closeWithX(workbench.aboutPopup)
        }
        function test_source_options_and_accepted_readiness() {
            const dialog = workbench.sourcePopup
            mouseClick(findChild(workbench, "sourceBlock"))
            tryCompare(dialog, "opened", true)
            compare(dialog.draft.mode, "prbs15")
            compare(dialog.draft.seed, 32767)
            dialog.edit("seed", 0)
            mouseClick(findChild(dialog, "sourceApply"))
            compare(dialog.visible, true)
            verify(dialog.errorMessage.length > 0)
            compare(workbench.sourceState.status, "attention")
            dialog.edit("seed", 1)
            mouseClick(findChild(dialog, "sourceApply"))
            tryCompare(dialog, "visible", false)
            compare(workbench.sourceState.status, "complete")
            compare(findChild(workbench, "sourceBlock").blockStatus, "complete")
            compare(findChild(workbench, "startButton").enabled, false)
            mouseClick(findChild(workbench, "sourceBlock"))
            tryCompare(dialog, "opened", true)
            dialog.edit("mode", "file")
            compare(findChild(dialog, "sourceSeed").visible, false)
            verify(findChild(dialog, "sourceOpenFile").visible)
            compare(findChild(dialog, "sourceTarget").enabled, false)
            dialog.chooseFile()
            compare(dialog.fileInfo.bytes, 2)
            dialog.edit("append", true)
            compare(findChild(dialog, "sourceTarget").enabled, true)
            dialog.edit("target_bits", 19)
            compare(dialog.summary.fill_bits, 3)
            mouseClick(findChild(dialog, "sourceCancel"))
            tryCompare(dialog, "visible", false)
            compare(sourceBackend.acceptedOptions.mode, "prbs15")
            mouseClick(findChild(workbench, "sourceBlock"))
            tryCompare(dialog, "opened", true)
            compare(dialog.fileInfo.selected, false)
            dialog.edit("mode", "file")
            dialog.chooseFile()
            mouseClick(findChild(dialog, "sourceApply"))
            tryCompare(dialog, "visible", false)
            compare(sourceBackend.acceptedOptions.mode, "file")
            sourceBackend.acceptedSource = false; sourceBackend.acceptedFile = false
            sourceBackend.acceptedOptions = ({mode: "prbs15", seed: 32767, append: false, target_bits: 0, fill_seed: 32767})
            workbench.refreshSourceState()
        }
        function test_transmitter_readiness_analyzer_zoom_and_invalidation() {
            sourceBackend.acceptedSource = true
            profileBackend.acceptedId = 0
            workbench.refreshSourceState()
            workbench.refreshProfileState()
            compare(findChild(workbench, "transmitterBlock").blockStatus, "complete")
            compare(findChild(workbench, "startButton").enabled, false)
            mouseClick(findChild(workbench, "transmitterBlock"))
            const dialog = workbench.transmitterPopup
            tryCompare(dialog, "opened", true)
            compare(transmitterBackend.busy, true)
            transmitterBackend.finish()
            tryVerify(() => dialog.viewport.points.length === 3)
            compare(findChild(dialog, "saReadout"), null)
            compare(findChild(dialog, "saStatus"), null)
            const timingPanel = findChild(dialog, "saTimingPanel")
            verify(timingPanel !== null)
            tryVerify(() => timingPanel.timing.points.length === 3)
            verify(findChild(timingPanel, "saEchoSpacing").text.indexOf("756 ft") >= 0)
            const cable = findChild(timingPanel, "saCableModel")
            cable.currentIndex = 1
            tryVerify(() => findChild(timingPanel, "saEchoSpacing").text.indexOf("802 ft") >= 0)
            cable.currentIndex = 2
            tryVerify(() => findChild(timingPanel, "saEchoSpacing").text.indexOf("Unknown VF") >= 0)
            cable.currentIndex = 0
            compare(dialog.captureFrequencyAxis, false)
            compare(dialog.viewport.count, 8192)
            compare(findChild(dialog, "saZoomIn"), null)
            compare(findChild(dialog, "saZoomOut"), null)
            const navigation = findChild(dialog, "chartNavigation")
            const x = navigation.leftMargin + navigation.plotWidth * 0.25
            const y = navigation.topMargin + 20
            mouseWheel(navigation, x, y, 0, 240)
            compare(dialog.viewport.count, 4096)
            compare(dialog.viewport.start, 1024)
            mouseWheel(navigation, x, y, 0, -240)
            compare(dialog.viewport.count, 8192)
            compare(dialog.viewport.start, 0)
            mouseDrag(navigation, navigation.leftMargin + navigation.plotWidth * 0.25, y, navigation.plotWidth * 0.5, 0)
            verify(dialog.viewport.count >= 4096 && dialog.viewport.count <= 4098)
            verify(dialog.viewport.start >= 2047 && dialog.viewport.start <= 2048)
            mouseClick(findChild(dialog, "saFullSpan"))
            compare(dialog.viewport.count, 8192)
            mouseWheel(navigation, 5, y, 0, 240)
            compare(dialog.viewport.count, 8192)
            for (let index = 0; index < 14; ++index) { mouseWheel(navigation, x, y, 0, 240) }
            compare(dialog.viewport.count, 2)
            verify(dialog.viewport.start >= 0 && dialog.viewport.start + dialog.viewport.count <= dialog.viewport.total)
            mouseClick(navigation, x, y)
            keyClick(Qt.Key_Home)
            compare(dialog.viewport.count, 8192)
            keyClick(Qt.Key_Minus)
            compare(dialog.viewport.count, 8192)
            keyClick(Qt.Key_Escape)
            tryCompare(dialog, "visible", false)
            sourceBackend.acceptedSource = false
            profileBackend.acceptedId = -1
            workbench.refreshSourceState()
            workbench.refreshProfileState()
            compare(workbench.transmitterState.status, "attention")
            compare(workbench.transmitterState.has_result, false)
            compare(dialog.viewport.points.length, 0)
            compare(navigation.canZoom, false)
            compare(timingPanel.timing.points.length, 0)
        }
        function test_shared_measurement_viewers_and_joint_channel_readiness() {
            profileBackend.acceptedId = 0
            workbench.refreshProfileState()
            compare(findChild(workbench, "channelEstimateBlock").blockStatus, "complete")
            compare(findChild(workbench, "channelBlock").blockStatus, "attention")
            mouseClick(findChild(workbench, "channelEstimateBlock"))
            tryCompare(workbench.responsePopup, "opened", true)
            const viewer = findChild(workbench.responsePopup, "sharedMeasurementViewer")
            verify(viewer !== null)
            const navigation = findChild(viewer, "chartNavigation")
            mouseWheel(navigation, navigation.leftMargin + navigation.plotWidth / 2, navigation.topMargin + 20, 0, 240)
            compare(viewer.viewport.count, 3740)
            mouseClick(findChild(viewer, "measurementFullSpan"))
            compare(viewer.viewport.count, 7480)
            keyClick(Qt.Key_Escape)
            tryCompare(workbench.responsePopup, "visible", false)
            mouseClick(findChild(workbench, "rxmerBlock"))
            tryCompare(workbench.rxmerPopup, "opened", true)
            verify(findChild(workbench.rxmerPopup, "sharedMeasurementViewer") !== null)
            compare(findChild(workbench.rxmerPopup, "measurementDemo").visible, false)
            compare(findChild(workbench.rxmerPopup, "measurementNotchDemo"), null)
            mouseClick(findChild(workbench.rxmerPopup, "measurementOpen"))
            mouseClick(findChild(workbench.rxmerPopup, "measurementApply"))
            tryCompare(workbench.rxmerPopup, "visible", false)
            compare(findChild(workbench, "rxmerBlock").blockStatus, "complete")
            compare(findChild(workbench, "channelBlock").blockStatus, "complete")
            compare(findChild(workbench, "startButton").enabled, false)
            mouseClick(findChild(workbench, "rxmerBlock"))
            tryCompare(workbench.rxmerPopup, "opened", true)
            measurementBackend.draftRx = false
            keyClick(Qt.Key_Escape)
            tryCompare(workbench.rxmerPopup, "visible", false)
            compare(measurementBackend.acceptedRx, true)
            measurementBackend.acceptedRx = false
            profileBackend.acceptedId = -1
            workbench.refreshProfileState()
            compare(findChild(workbench, "channelBlock").blockStatus, "attention")
        }
        function test_receiver_signal_views_and_independent_spectrum_taps() {
            const receiver = workbench.receiverPopup
            mouseClick(findChild(workbench, "receiverBlock"))
            tryCompare(receiver, "opened", true)
            mouseClick(findChild(receiver, "receiverNcpQam"))
            const qam = receiver.qamPopup
            tryCompare(qam, "opened", true)
            compare(qam.signalIndex, 1)
            verify(qam.observation.detail.indexOf("NCP") >= 0)
            const signal = findChild(qam, "receiverQamSignal")
            mouseClick(signal); keyClick(Qt.Key_Down); keyClick(Qt.Key_Return)
            compare(qam.signalIndex, 2)
            verify(findChild(qam, "receiverPlcPart").visible)
            const view = findChild(qam, "receiverQamView")
            mouseClick(view); keyClick(Qt.Key_End); keyClick(Qt.Key_Return)
            compare(qam.viewIndex, 2)
            compare(qam.observation.points.length, 0)
            closeWithX(qam)
            compare(receiver.opened, true)
            waitForRendering(receiver.contentItem)
            wait(100)
            mouseClick(findChild(receiver, "receiverInputTap"))
            const sa = receiver.spectrumPopup
            tryCompare(sa, "opened", true)
            compare(sa.state.tap_id, "channel.output")
            compare(findChild(sa, "saGenerate").enabled, false)
            sa.viewport = ({points:[{x:0,y:0,readout:"Previous result"}],x_ticks:[],y_ticks:[]})
            const tap = findChild(sa, "saTestPoint")
            tap.forceActiveFocus(); keyClick(Qt.Key_End); keyClick(Qt.Key_Return)
            compare(sa.state.tap_id, "rx.fft.bins")
            compare(sa.viewport.points.length, 0)
            compare(qam.signalIndex, 2)
            compare(findChild(sa, "saTimingPanel").visible, false)
            closeWithX(sa)
            mouseClick(findChild(receiver, "receiverDataQam"))
            tryCompare(qam, "opened", true)
            compare(qam.signalIndex, 0)
            compare(receiver.selectedTap, 3)
            compare(findChild(qam, "receiverProfile").enabled, false)
            keyClick(Qt.Key_Escape); tryCompare(qam, "visible", false)
            keyClick(Qt.Key_Escape); tryCompare(receiver, "visible", false)
            compare(findChild(workbench, "receiverBlock").blockStatus, "attention")
        }
        function test_receiver_configuration_comparison_and_run_controls() {
            mouseClick(findChild(workbench,"receiverBlock"))
            tryCompare(workbench.receiverPopup,"opened",true)
            mouseClick(findChild(workbench.receiverPopup,"receiverConfigure"))
            tryCompare(workbench.receiverConfigPopup,"opened",true)
            mouseClick(findChild(workbench.receiverConfigPopup,"receiverConfigApply"))
            tryCompare(workbench.receiverConfigPopup,"visible",false)
            compare(workbench.runState.receiver_status,"complete")
            keyClick(Qt.Key_Escape);tryCompare(workbench.receiverPopup,"visible",false)
            mouseClick(findChild(workbench,"verificationBlock"))
            tryCompare(workbench.verificationPopup,"opened",true)
            mouseClick(findChild(workbench.verificationPopup,"verificationApply"))
            compare(workbench.runState.verify_status,"complete")
            keyClick(Qt.Key_Escape);tryCompare(workbench.verificationPopup,"visible",false)
            runBackend.start();workbench.pollRun()
            compare(findChild(workbench,"pauseButton").enabled,true)
            mouseClick(findChild(workbench,"pauseButton"))
            compare(workbench.runState.paused,true)
            mouseClick(findChild(workbench,"pauseButton"))
            compare(workbench.runState.paused,false)
            mouseClick(findChild(workbench,"stopButton"));workbench.pollRun()
            compare(workbench.runState.busy,false)
            runBackend.configured=false;runBackend.verified=false;workbench.pollRun()
        }
        function test_verification_partial_results_running_and_empty_state() {
            const dialog = workbench.verificationPopup
            runBackend.lastResult = {status:"cancelled",bits_tested:0,bit_errors:0,ber:null,recovered_payloads:0,unrecovered:2}
            receiverBackend.retainedCells = true
            workbench.pollRun()
            dialog.open();tryCompare(dialog,"opened",true)
            compare(dialog.statusText,"Cancelled · Partial Result")
            verify(dialog.hasResult)
            verify(findChild(dialog,"verificationWordErrors") !== null)
            runBackend.busy = true;workbench.pollRun()
            compare(dialog.statusText,"Running")
            compare(dialog.hasResult,false)
            compare(dialog.statistics.available,false)
            runBackend.paused = true;workbench.pollRun()
            compare(dialog.statusText,"Paused")
            runBackend.busy = false;runBackend.paused = false
            runBackend.lastResult = null;workbench.pollRun()
            compare(dialog.hasResult,false)
            compare(dialog.statistics.available,false)
            mouseClick(findChild(dialog,"popupCloseButton"));tryCompare(dialog,"visible",false)
            receiverBackend.retainedCells = false
        }
        function test_receiver_retained_filters_response_soft_and_fec_navigation() {
            const receiver=workbench.receiverPopup
            receiverBackend.retainedCells=true
            receiver.open();tryCompare(receiver,"opened",true)
            mouseClick(findChild(receiver,"receiverDataQam"));tryCompare(receiver.qamPopup,"opened",true)
            const qam=receiver.qamPopup
            compare(qam.observation.points.length,1)
            verify(findChild(qam,"receiverProfile").enabled)
            const first=findChild(qam,"receiverFirstBin")
            first.value=400;first.valueModified()
            compare(receiverBackend.lastQuery.first_bin,400)
            compare(qam.observation.points.length,0)
            qam.selectionControls.resetFilters()
            compare(qam.observation.points.length,1)
            closeWithX(qam);compare(receiver.opened,true)
            receiver.responsePopup.open();tryCompare(receiver.responsePopup,"opened",true)
            const view=findChild(receiver.responsePopup,"receiverResponseView")
            view.currentIndex=4;view.activated(4)
            compare(receiverBackend.lastQuery.view,4)
            closeWithX(receiver.responsePopup)
            receiver.openSlicer();tryCompare(receiver.slicerPopup,"opened",true)
            const boundary=findChild(receiver.slicerPopup,"receiverSoftBoundary")
            boundary.currentIndex=1;boundary.activated(1)
            compare(receiverBackend.lastDecoderInput,true)
            closeWithX(receiver.slicerPopup)
            mouseClick(findChild(receiver,"receiverFecShortcut"));tryCompare(receiver.fecPopup,"opened",true)
            verify(findChild(receiver.fecPopup,"receiverFecStatus").text.indexOf("complete")>=0)
            closeWithX(receiver.fecPopup);closeWithX(receiver)
            receiverBackend.retainedCells=false
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
