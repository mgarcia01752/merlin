/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml/components"
import "../qml/shell"
import "../qml/theme"

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
        function reload() {}
        function begin() { loaded = acceptedId >= 0; return view(acceptedId) }
        function demo() { loaded = true; return view(0) }
        function preview(id) { return view(id) }
        function plot(id, start, count) {
            return JSON.stringify({profile_id:id,total:100,start:count ? start : 0,count:count || 100,points:[{x:0,y:0.5,minimum_y:0.5,role:"data",readout:"Test carrier"}],roles:[{kind:"data",label:"Data"},{kind:"continuous_pilot",label:"Pilots"}],x_ticks:[],y_ticks:[],axis_label:"Bit Loading (Bits/Symbol)"})
        }
        function inspect(request) {
            const r = JSON.parse(request), v = JSON.parse(plot(0,r.start,r.count))
            v.series = r.profiles.map(id => ({id:id,points:[{x:0,y:0.5,minimum_y:0.5,gap:false,readout:"Test carrier"}]}))
            v.profiles = [0,1].map(id => ({id:id,bandwidth_text:"100.000 Mbit/s",bandwidth_bps:100000000}))
            v.roles = [{kind:"data",label:"Data",count:90},{kind:"continuous_pilot",label:"Pilots",count:10}]
            v.lanes = ["continuous_pilot"]; v.markers = []; v.channel=34; v.mac="00:ca:00:00:00:01"
            return JSON.stringify(v)
        }
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
        function reload() {}
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
        function stage(index,offset) { return JSON.stringify({available:true,rows:[{index:offset,value:"1010"}],points:[{x:0.5,y:0.5,role:"data"},{x:0.6,y:0.6,role:"ncp"}],role_counts:{data:1,ncp:1},symbol:128,phase:"Observation",retained:4096,total:4096,offset:offset,end:Math.min(4096,offset+2048),iq_count:1,missing:0}) }
        function messages() {
            const packet = {name:"OCD",type:49,channel:34,profile:null,change_count:0,source:"00:ca:00:00:00:01",bytes:40,symbol:0,completed_frame:true,status:"Carried in completed TX PLC frame",integrity:"HCS / FCS valid",tlvs:[{name:"Cyclic Prefix",value:"512 samples",kind:1,length:1,offset:28,hex:"02",children:[{name:"Encoded Value",value:"512 samples",offset:30,length:1,hex:"02"},{name:"Value Bytes",value:"02",offset:30,length:1,hex:"02"}]}],hex_rows:[{offset:"0000",hex:"C2 00"}]}
            const dpd = Object.assign({}, packet, {name:"DPD",type:50,profile:0,tlvs:[{name:"Subcarrier Assignment",value:"0–8191 · 256-QAM",kind:5,length:5,offset:29,hex:"28 00 00 1F FF",children:[{name:"First Subcarrier",value:"0",offset:32,length:2,hex:"00 00"}]}]})
            return JSON.stringify({messages:[packet,dpd],reason:"",pending_bytes:0})
        }
        function channel_qam(first,last) {return JSON.stringify({available:false,points:[],reason:"No TX bins retained"})}
        function spectrum_tap(tap) { return JSON.stringify({has_result:false,can_generate:false,busy:false,tap_name:["TX IFFT","TX Cyclic Prefix","TX Window","TX Output"][tap],tap_id:["tx.ifft.samples","tx.cp.samples","tx.window.samples","tx.overlap_add.samples"][tap],result_token:"",reason:"Use Start to capture TX samples"}) }
        function view_tap(tap,start,count,captureAxis) { return view(start,count,captureAxis) }
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
        property var loss: ({mode:"off"})
        property var power: ({amplifier_gain_db:0,attenuation_db:0,input_data_level_dbmv_per_6mhz:null,amplifier_target_dbmv_per_6mhz:null,receiver_noise_dbmv_hz:null})
        function power_current(){return JSON.stringify(power)}
        function power_apply(document){const draft=JSON.parse(document);if(draft.amplifier_gain_db<0 || draft.attenuation_db<0){return "Invalid gain"};power=draft;return ""}
        function tap_state(tap){return JSON.stringify({has_result:false,can_generate:false,tap_name:["Before Amplifier","After Amplifier","Channel Output / RX Input"][tap],reason:"No retained data",result_token:""})}
        function tap_spectrum(tap,start,count,axis){return JSON.stringify({points:[],total:8192,start:start,count:count || 8192,channel_power_text:"Channel Power: Test"})}
        function tap_qam(tap,query){return JSON.stringify({available:false,points:[],reason:"No retained data"})}

        function loss_current() {return JSON.stringify(loss)}
        function loss_preview(document,settings) {
            const options=JSON.parse(document)
            const valid=options.mode!=="cable" || (options.length>=0 && options.length<=1000)
            return JSON.stringify({error:valid ? "" : "Invalid length",points:[],chart:[],band:"640–827 MHz",low_gain:"−4 dB",high_gain:"−5 dB",tilt:"−1 dB"})
        }
        function loss_apply(document,settings) {const error=JSON.parse(loss_preview(document,settings)).error;if(!error){loss=JSON.parse(document)};return error}
        function reload() {}
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
        function faq_document(code) { return JSON.stringify({title:code ? "RxMER playback needs a longer run":"MERLIN FAQ",text:"## RxMER playback needs a longer run\nChoose a duration covering all captures."}) }
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
        property var fecOverride: null
        function fec_profile(id) {
            if (fecOverride !== null) { return JSON.stringify(fecOverride) }
            const selected = id < 0 ? 3 : id
            const total = selected === 3 ? 122 : 21
            const allocation = {total:total,segments:[{key:"clean",label:"Clean",count:0,fraction:0},{key:"corrected",label:"Corrected",count:total,fraction:1},{key:"failed",label:"Uncorrectable",count:0,fraction:0}]}
            return JSON.stringify({available:retainedCells,status:"complete",profile_id:selected,profiles:[{id:0,status:"complete"},{id:3,status:"complete"}],fec:{total_cw:total,ccw:total,ufec:0},outcomes:allocation,recovery:{total:125,segments:[{key:"clean",label:"Recovered",count:125,fraction:1}]},recovered:125,generated:125,compared_bits:1779000,bit_errors:0,ber:0,mean_iterations:1.032,iterations:{maximum:121,points:[{iteration:1,count:121,fraction:1},{iteration:2,count:4,fraction:4/121}]},words:[{receive_sequence:0,first_source_symbol:1,last_source_symbol:1,ldpc_success:true,ldpc_iterations:1,bch_success:true,bch_corrected_bits:0,comparison:{bit_errors:0}},{receive_sequence:1,first_source_symbol:2,last_source_symbol:2,ldpc_success:false,ldpc_iterations:100,bch_success:false,bch_corrected_bits:0,comparison:{bit_errors:7}}],omitted_words:61})
        }
        function fec() {return JSON.stringify({available:retainedCells,status:"complete",words:[],verification_charts:{recovery:{points:[]},word_errors:{points:[]},omitted:0,uncompared_retained:0},histogram:{points:[],x_ticks:[],y_ticks:[]}})}
        function performance(id,start,count) {
            if (!retainedCells) return JSON.stringify({available:false,profiles:[],reason:"Run the simulation first."})
            return JSON.stringify({available:true,profiles:[{id:0,status:"complete"},{id:3,status:"cancelled"}],profile_id:id < 0 ? 3 : id,channel:34,status:id === 0 ? "complete":"cancelled",fec:{total_cw:10,ccw:3,ufec:1,clean_cw:6,undetected_errors:0},total:100,start:count ? start : 0,count:count || 100,points:[{x:0},{x:1}],series:[],lanes:[],markers:[],x_ticks:[],y_ticks:[]})
        }

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
        property var acceptedRunOptions: ({receiver_mode:"supplied_channel",all_profiles:false,selected_profile_ids:null,duration_ms:3,max_iterations:50,max_runtime_ms:300000,noise_seed:41})
        property var lastResult: null
        property var timing: null
        property var diagnostic: null
        function data() { return {diagnostic:diagnostic,timing:timing,completed_symbols:23,profile_id:3,profile_index:4,profile_count:4,receiver_status:configured ? "complete":"attention",verify_status:verified ? "complete":"attention",reason:"Configure accepted receiver and independent source comparison before running.",busy:busy,paused:paused,can_start:configured&&verified&&!busy,has_result:!!lastResult,result:lastResult,result_token:lastResult ? "1":"0",available_profile_ids:profileBackend.acceptedId>=0 ? [0,1]:[],applied_profile_id:profileBackend.acceptedId,options:acceptedRunOptions,accepted_options:configured ? acceptedRunOptions:null,verified:verified} }
        function configure(settings) { return JSON.stringify(data()) }
        function poll() { return JSON.stringify(data()) }
        function apply(options,settings) { acceptedRunOptions=JSON.parse(options);configured=true;return "" }
        function verify() { verified=true }
        function start() { busy=true }
        function stop() { busy=false;paused=false }
        function pause() { paused=!paused }
        function restore(options,verify,settings) { configured=options!=="null";verified=verify;if(configured) acceptedRunOptions=JSON.parse(options);else acceptedRunOptions={receiver_mode:"supplied_channel",all_profiles:false,selected_profile_ids:null,duration_ms:3,max_iterations:50,max_runtime_ms:300000,noise_seed:41} }
    }
    QtObject {
        id: fileBackend
        property var openResponse: ({cancelled:true})
        function new_workbench() {
            profileBackend.acceptedId=-1;sourceBackend.acceptedSource=false;measurementBackend.acceptedRx=false
            return JSON.stringify({created:true,transmission:JSON.parse(backend.defaults()),receiver:null,verified:false,dark:null})
        }
        function open() { return JSON.stringify(openResponse) }
        property var saveResponse: ({cancelled:true})
        function save(saveAs,settings,receiver,verified,dark) { return JSON.stringify(saveResponse) }
    }
    WorkbenchMenuBar {
        id: appMenu
        property url openedTerminology: ""
        onTerminologyRequested: function(target) { openedTerminology=target }
        anchors.top: parent.top; width: parent.width; theme: theme
        onNewRequested: workbench.requestNewProject()
        onGlobalSettingsRequested: workbench.settingsPopup.open()
        onAboutRequested: workbench.aboutPopup.open()
        onFaqRequested: workbench.openHelp("")
    }
    Workbench {
        id: workbench
        anchors { top: appMenu.bottom; bottom: parent.bottom; left: parent.left; right: parent.right }
        theme: theme; settingsBackend: backend; profileBackend: profileBackend; sourceBackend: sourceBackend; transmitterBackend: transmitterBackend; measurementBackend: measurementBackend; aboutBackend: aboutBackend; receiverBackend: receiverBackend; runBackend: runBackend; fileBackend: fileBackend
        iconRoot: Qt.resolvedUrl("../../../docs/images/logo/").toString()
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
                {tag: "tx", block: "transmitterBlock", title: "Transmitter Workbench"},
                {tag: "channel", block: "channelBlock", title: "Channel Simulator Workbench"},
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
                iconRoot: Qt.resolvedUrl("../../../docs/images/logo/").toString()
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
            mouseClick(findChild(dialog, "profileView"))
            tryCompare(dialog.plotPopup, "opened", true)
            compare(dialog.plotPopup.selectedProfile,0)
            compare(profileBackend.acceptedId,-1)
            dialog.plotPopup.refresh(10,20)
            compare(dialog.plotPopup.viewport.start,10)
            compare(dialog.plotPopup.viewport.count,20)
            mouseClick(findChild(dialog.plotPopup,"profilePlotFullSpan"))
            compare(dialog.plotPopup.viewport.count,100)
            mouseClick(findChild(dialog.plotPopup,"modulationAllProfiles"))
            compare(dialog.plotPopup.selectedIds.length,2)
            wait(50)
            mouseClick(findChild(dialog.plotPopup,"profileToggleRepeater").itemAt(1))
            compare(dialog.plotPopup.selectedIds.length,1)
            wait(50)
            mouseClick(findChild(dialog.plotPopup,"carrierToggleRepeater").itemAt(0))
            compare(dialog.plotPopup.selectedRoles.indexOf("data"),-1)
            mouseClick(findChild(dialog.plotPopup,"modulationAllCarriers"))
            compare(dialog.plotPopup.selectedRoles.length,7)
            dialog.plotPopup.metric=1
            dialog.plotPopup.refreshRange()
            compare(profileBackend.acceptedId,-1)
            mouseClick(findChild(dialog.plotPopup,"profileBandwidthChip"))
            const bw = findChild(dialog.plotPopup,"profileBandwidthDialog")
            tryCompare(bw,"opened",true)
            bw.close()
            dialog.plotPopup.close()
            tryCompare(dialog.plotPopup,"visible",false)
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
            mouseClick(appMenu.itemAt(2))
            mouseClick(findChild(appMenu,"terminologyAction"))
            compare(String(appMenu.openedTerminology),"https://github.com/mgarcia01752/merlin/blob/master/docs/user/terminology.md")
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
        function test_contextual_help_hover_keyboard_and_offline_faq() {
            runBackend.diagnostic={code:"PlaybackObservationTooShort",tooltip:"Current duration: 3 ms. Set duration to at least 43 ms."}
            workbench.pollRun()
            const status=findChild(workbench,"runProgress")
            compare(status.hasHelp,true)
            mouseMove(status,10,status.height/2)
            const tooltip=findChild(status,"contextualStatusTooltip")
            tryCompare(tooltip,"visible",true)
            verify(tooltip.text.indexOf("43 ms")>=0)
            mouseClick(status,10,status.height/2)
            tryCompare(workbench.faqPopup,"opened",true)
            verify(workbench.faqPopup.contextText.indexOf("3 ms")>=0)
            verify(workbench.faqPopup.documentText.indexOf("all captures")>=0)
            workbench.faqPopup.close()
            tryCompare(workbench.faqPopup,"opened",false)
            status.forceActiveFocus()
            keyClick(Qt.Key_Return)
            tryCompare(workbench.faqPopup,"opened",true)
            workbench.faqPopup.close()
            tryCompare(workbench.faqPopup,"opened",false)
            runBackend.diagnostic=null
            workbench.pollRun()
            compare(status.hasHelp,false)
            workbench.openHelp("")
            tryCompare(workbench.faqPopup,"opened",true)
            compare(workbench.faqPopup.contextText,"")
            workbench.faqPopup.close()
        }
        function test_transmitter_readiness_analyzer_zoom_and_invalidation() {
            sourceBackend.acceptedSource = true
            profileBackend.acceptedId = 0
            workbench.refreshSourceState()
            workbench.refreshProfileState()
            compare(findChild(workbench, "transmitterBlock").blockStatus, "complete")
            compare(findChild(workbench, "startButton").enabled, false)
            mouseClick(findChild(workbench, "transmitterBlock"))
            const transmitter = workbench.transmitterPopup
            tryCompare(transmitter, "opened", true)
            compare(transmitterBackend.busy, false)
            mouseClick(transmitter.stageRepeater.itemAt(6).outputSaButton)
            const dialog = transmitter.spectrumPopup
            tryCompare(dialog, "opened", true)
            compare(transmitterBackend.busy, true)
            verify(findChild(dialog.footer,"saGenerate")!==null)
            compare(findChild(dialog.footer,"saCancel").text,"Cancel preview")
            tryCompare(findChild(dialog.footer,"saCancel"),"visible",true)
            transmitterBackend.finish()
            tryVerify(() => dialog.viewport.points.length === 3)
            tryCompare(findChild(dialog.footer,"saCancel"),"visible",false)
            const details=findChild(dialog,"saAcquisitionDetails")
            verify(details!==null)
            compare(details.visible,true)
            const canvas=findChild(dialog,"saCanvas")
            verify(details.mapToItem(window,0,0).y >= canvas.mapToItem(window,0,canvas.height).y)
            const toggle=findChild(dialog,"saDetailsToggle")
            mouseClick(toggle)
            compare(details.visible,false)
            compare(dialog.viewport.points.length,3)
            mouseClick(toggle)
            compare(details.visible,true)
            compare(findChild(dialog, "saReadout"), null)
            compare(findChild(dialog, "saStatus"), null)
            const timingPanel = findChild(dialog, "saTimingPanel")
            verify(timingPanel !== null)
            verify(findChild(timingPanel,"saCableGrid")!==null)
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
            compare(findChild(dialog,"saFrequencyAxis"),null)
            const basebandAxis=findChild(dialog,"saBasebandAxis")
            const captureAxis=findChild(dialog,"saCaptureAxis")
            mouseClick(captureAxis)
            compare(dialog.captureFrequencyAxis,true)
            compare(captureAxis.checked,true)
            compare(basebandAxis.checked,false)
            compare(dialog.viewport.x_ticks[0].label,"631.100")
            compare(dialog.viewport.count,8192)
            compare(transmitterBackend.busy,false)
            mouseClick(basebandAxis)
            compare(dialog.captureFrequencyAxis,false)
            compare(basebandAxis.checked,true)
            compare(captureAxis.checked,false)
            compare(dialog.viewport.x_ticks[0].label,"-102.400")
            compare(findChild(dialog, "saZoomIn"), null)
            compare(findChild(dialog, "saZoomOut"), null)
            const navigation = findChild(dialog, "chartNavigation")
            const x = navigation.leftMargin + navigation.plotWidth * 0.25
            const y = navigation.topMargin + 20
            mouseWheel(navigation, x, y, 0, 240)
            compare(dialog.viewport.count, 4096)
            compare(dialog.viewport.start, 1024)
            mouseClick(captureAxis)
            compare(dialog.viewport.start,1024)
            compare(dialog.viewport.count,4096)
            mouseClick(basebandAxis)
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
            closeWithX(transmitter)
            sourceBackend.acceptedSource = false
            profileBackend.acceptedId = -1
            workbench.refreshSourceState()
            workbench.refreshProfileState()
            compare(workbench.transmitterState.status, "attention")
            compare(workbench.transmitterState.has_result, false)
            compare(dialog.viewport.points.length, 0)
            compare(navigation.canZoom, false)
            compare(timingPanel.timing.points.length, 0)
            compare(basebandAxis.enabled,false)
            compare(captureAxis.enabled,false)
        }
        function test_transmitter_stage_message_and_qam_navigation() {
            mouseClick(findChild(workbench,"transmitterBlock"))
            const tx=workbench.transmitterPopup
            tryCompare(tx,"opened",true)
            compare(tx.stageRepeater.itemAt(0).modelData,"Source\nPRBS-TX")
            verify(findChild(tx,"txMessages").width>=200)
            verify(findChild(tx,"txProtected").width>=200)
            mouseClick(tx.stageRepeater.itemAt(1).tapButton)
            tryCompare(tx.stagePopup,"opened",true)
            compare(tx.stagePopup.stageIndex,1)
            compare(findChild(tx.stagePopup,"txStageSelection").model[0],"Source Bits · PRBS-TX")
            mouseClick(findChild(tx.stagePopup,"txNextPage"))
            compare(tx.stagePopup.pageOffset,2048)
            closeWithX(tx.stagePopup)
            mouseClick(findChild(tx,"txQam"))
            tryCompare(tx.qamPopup,"opened",true)
            verify(tx.qamPopup.observation.points.length>0)
            compare(findChild(tx.qamPopup,"txQamLegend").visible,true)
            const scatter=findChild(tx.qamPopup,"txQamScatter")
            compare(scatter.roleColor("data"),findChild(tx.qamPopup,"txQamDataLegend").color)
            compare(scatter.roleColor("ncp"),findChild(tx.qamPopup,"txQamNcpLegend").color)
            verify(scatter.roleColor("data")!==scatter.roleColor("ncp"))
            theme.dark=false
            compare(scatter.roleColor("data"),theme.qamData)
            compare(scatter.roleColor("ncp"),theme.qamNcp)
            verify(scatter.roleColor("data")!==scatter.roleColor("ncp"))
            theme.dark=true
            const signal=findChild(tx.qamPopup,"txQamSignal")
            signal.currentIndex=2;signal.activated(2)
            compare(findChild(tx.qamPopup,"txQamLegend").visible,false)
            signal.currentIndex=0;signal.activated(0)
            closeWithX(tx.qamPopup)
            mouseClick(findChild(tx,"txMessages"))
            tryCompare(tx.messagePopup,"opened",true)
            compare(tx.messagePopup.packet.channel,34)
            const tree = findChild(tx.messagePopup,"txTlvTree")
            compare(tree.rowCount,1)
            mouseClick(findChild(tree,"tlvFold0"))
            compare(tree.rowCount,3)
            verify(tree.list.activeFocus)
            mouseClick(findChild(tree,"tlvFold0"))
            compare(tree.rowCount,1)
            mouseClick(findChild(tree,"tlvExpandAll"))
            compare(tree.rowCount,3)
            compare(tree.visibleRows[1].node.offset,30)
            compare(tree.visibleRows[1].depth,1)
            mouseClick(findChild(tree,"tlvCollapseAll"))
            compare(tree.rowCount,1)
            tree.list.currentIndex=0
            tree.list.forceActiveFocus()
            keyClick(Qt.Key_Right)
            compare(tree.rowCount,3)
            tree.list.currentIndex=1
            keyClick(Qt.Key_Left)
            compare(tree.list.currentIndex,0)
            keyClick(Qt.Key_Left)
            compare(tree.rowCount,1)
            mouseClick(findChild(tree,"tlvExpandAll"))
            mouseClick(findChild(tx.messagePopup,"txMessageBytes"))
            compare(tx.messagePopup.showBytes,true)
            mouseClick(findChild(tx.messagePopup,"txMessageBytes"))
            compare(tx.messagePopup.showBytes,false)
            compare(tree.rowCount,3)
            const messageSelection=findChild(tx.messagePopup,"txMessageSelection")
            messageSelection.currentIndex=1
            messageSelection.activated(1)
            compare(tx.messagePopup.packet.profile,0)
            compare(tree.rowCount,1)
            compare(tree.selectedNode,null)

            closeWithX(tx.messagePopup)
            runBackend.lastResult=null
            runBackend.busy=true
            workbench.pollRun()
            tx.openSpectrum(0)
            tryCompare(tx.spectrumPopup,"opened",true)
            compare(tx.usePreview,false)
            compare(transmitterBackend.busy,false)
            closeWithX(tx.spectrumPopup)
            runBackend.stop()
            workbench.pollRun()
            closeWithX(tx)
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
        function test_run_elapsed_survives_completion_pause_and_stationary_symbols() {
            runBackend.busy = true
            runBackend.timing = {elapsed_text:"00:01:05",profile_elapsed_text:"00:00:42",phase:"Draining",recovered_payloads:12,progress_age_text:"00:00:03"}
            workbench.pollRun()
            const elapsed = findChild(workbench,"runElapsed")
            const progress = findChild(workbench,"runProgress")
            verify(elapsed.visible)
            verify(elapsed.text.indexOf("Elapsed 00:01:05") >= 0)
            verify(elapsed.text.indexOf("Profile 00:00:42") >= 0)
            verify(progress.text.indexOf("Draining") >= 0)
            verify(progress.text.indexOf("Recovered: 12") >= 0)
            runBackend.paused = true;workbench.pollRun()
            runBackend.timing = Object.assign({},runBackend.timing,{elapsed_text:"00:01:10"})
            workbench.pollRun()
            verify(elapsed.text.indexOf("00:01:10") >= 0)
            runBackend.busy = false;runBackend.paused = false
            workbench.pollRun()
            compare(elapsed.text,"Elapsed 00:01:10")
            runBackend.timing = null;workbench.pollRun()
            compare(elapsed.visible,false)
        }
        function test_fec_overview_profiles_tabs_and_partial_empty_states() {
            const dialog = workbench.receiverPopup.fecPopup
            workbench.receiverPopup.open();tryCompare(workbench.receiverPopup,"opened",true)
            dialog.profileId = -1
            receiverBackend.retainedCells = true
            dialog.open();tryCompare(dialog,"opened",true)
            compare(findChild(findChild(dialog,"fecCounters").itemAt(0),"fecCounter_total_cw").text,"122")
            compare(findChild(dialog,"fecIterationChart").histogram.points.length,2)
            const choice = findChild(dialog,"fecProfileChoice")
            choice.currentIndex = 0;choice.activated(0)
            compare(findChild(findChild(dialog,"fecCounters").itemAt(0),"fecCounter_total_cw").text,"21")
            compare(dialog.statistics.profile_id,0)
            mouseClick(findChild(dialog,"fecWordsTab"));wait(30)
            compare(findChild(dialog,"receiverFecWords").count,2)
            mouseClick(findChild(dialog,"fecErrorsOnly"));wait(30)
            compare(findChild(dialog,"receiverFecWords").count,1)
            mouseClick(findChild(dialog,"fecControlTab"));wait(30)
            mouseClick(findChild(dialog,"fecOverviewTab"));wait(30)
            receiverBackend.fecOverride = {available:true,status:"cancelled",profile_id:0,fec:{total_cw:0,ccw:0,ufec:0},iterations:{points:[],maximum:0},words:[]}
            dialog.refresh()
            verify(findChild(dialog,"receiverFecStatus").text.indexOf("Partial")>=0)
            compare(findChild(findChild(dialog,"fecCounters").itemAt(0),"fecCounter_total_cw").text,"0")
            receiverBackend.fecOverride = {available:false,reason:"No results",words:[]}
            dialog.refresh()
            compare(findChild(findChild(dialog,"fecCounters").itemAt(0),"fecCounter_total_cw").text,"—")
            compare(findChild(dialog,"fecIterationChart").histogram.points.length,0)
            closeWithX(dialog);closeWithX(workbench.receiverPopup)
            receiverBackend.fecOverride = null
            dialog.profileId = -1
        }
        function test_apply_cancel_footers_align_and_reject() {
            const editors = [
                [workbench.configurationPopup, "wbConfigurationApply", "dialogCancel"],
                [workbench.receiverConfigPopup, "receiverConfigApply", "dialogCancel"],
                [workbench.ofdmPopup, "ofdmApply", "dialogCancel"],
                [workbench.profilePopup, "profileApply", "dialogCancel"],
                [workbench.sourcePopup, "sourceApply", "sourceCancel"],
                [workbench.rxmerPopup, "measurementApply", "dialogCancel"],
                [workbench.channelPopup.lossPopup, "channelLossApply", "dialogCancel"],
                [workbench.channelPopup.amplifierPopup, "channelAmplifierApply", "dialogCancel"],
                [workbench.channelPopup.attenuatorPopup, "channelAttenuatorApply", "dialogCancel"]
            ]
            for (const editor of editors) {
                const dialog = editor[0]
                const channelEditor = editor[1].indexOf("channel") === 0
                if (channelEditor) {
                    workbench.channelPopup.open()
                    tryCompare(workbench.channelPopup, "opened", true)
                    if (editor[1] === "channelLossApply") {
                        workbench.channelPopup.modelPopup.open()
                        tryCompare(workbench.channelPopup.modelPopup, "opened", true)
                    }
                }
                dialog.open()
                tryCompare(dialog, "opened", true)
                const apply = findChild(dialog, editor[1])
                const cancel = findChild(dialog, editor[2])
                verify(apply !== null)
                verify(cancel !== null)
                compare(apply.text, "Apply")
                compare(cancel.text, "Cancel")
                compare(apply.parent, cancel.parent)
                compare(apply.y, cancel.y)
                compare(apply.width, cancel.width)
                compare(apply.height, cancel.height)
                compare(cancel.x - apply.x - apply.width, 10)
                const corner = cancel.mapToItem(dialog.footer, cancel.width, cancel.height)
                compare(Math.round(dialog.footer.width - corner.x), 10)
                compare(Math.round(dialog.footer.height - corner.y), 10)
                mouseClick(cancel)
                tryCompare(dialog, "visible", false)
                if (channelEditor) {
                    if (editor[1] === "channelLossApply") {
                        closeWithX(workbench.channelPopup.modelPopup)
                    }
                    closeWithX(workbench.channelPopup)
                }
            }
        }
        function test_profile_dialog_bounds_at_minimum_window() {
            const host = window.Window.window
            const previous = [host.width, host.height, window.width, window.height]
            const popup = workbench.profilePopup
            try {
                popup.open(); tryCompare(popup, "opened", true); popup.close()
                host.width = 960; host.height = 640
                window.width = 960; window.height = 640
                popup.open(); tryCompare(popup, "opened", true); wait(200)
                const close = findChild(popup.header, "popupCloseButton")
                const top = close.mapToItem(host.contentItem, 0, 0)
                const bottom = popup.footer.mapToItem(host.contentItem, popup.footer.width, popup.footer.height)
                verify(top.x >= 0 && top.y >= 0)
                verify(top.x + close.width <= host.width)
                verify(bottom.y <= host.height)
                closeWithX(popup)
            } finally {
                popup.close()
                host.width = previous[0]; host.height = previous[1]
                window.width = previous[2]; window.height = previous[3]
                wait(200)
            }
        }
        function test_workbench_configuration_entry_choices() {
            workbench.hasProject = false
            const configure = findChild(workbench, "configureWorkbench")
            const settings = workbench.configurationPopup
            const entry = workbench.entryPopup
            verify(configure.enabled)
            mouseClick(configure)
            tryCompare(entry, "opened", true)
            compare(settings.visible, false)
            mouseClick(findChild(entry, "cancelWorkbenchChoice"))
            tryCompare(entry, "visible", false)
            compare(workbench.hasProject, false)
            mouseClick(configure)
            tryCompare(entry, "opened", true)
            closeWithX(entry)
            compare(workbench.hasProject, false)
            for (const response of [{cancelled:true}, {error:"Invalid WB"}]) {
                fileBackend.openResponse = response
                mouseClick(configure)
                tryCompare(entry, "opened", true)
                mouseClick(findChild(entry, "openWorkbenchChoice"))
                tryCompare(entry, "visible", false)
                compare(workbench.hasProject, false)
                compare(settings.visible, false)
                keyClick(Qt.Key_Escape)
            }
            mouseClick(configure)
            tryCompare(entry, "opened", true)
            mouseClick(findChild(entry, "createWorkbenchChoice"))
            tryCompare(settings, "opened", true)
            compare(workbench.hasProject, true)
            closeWithX(settings)
            workbench.hasProject = false
            fileBackend.openResponse = {opened:true, path:"test.merlin.json",
                transmission:JSON.parse(backend.current()),
                receiver:runBackend.acceptedRunOptions, verified:false, dark:true}
            mouseClick(configure)
            tryCompare(entry, "opened", true)
            mouseClick(findChild(entry, "openWorkbenchChoice"))
            tryCompare(settings, "opened", true)
            compare(workbench.hasProject, true)
            closeWithX(settings)
            fileBackend.openResponse = {cancelled:true}
            runBackend.busy = true
            workbench.pollRun()
            workbench.configureProject()
            compare(entry.visible, false)
            compare(settings.visible, false)
            verify(!configure.enabled)
            runBackend.busy = false
            workbench.pollRun()
        }
        function test_workbench_configuration_lifecycle_and_profile_selection() {
            workbench.hasProject=false
            compare(findChild(workbench,"configureWorkbench").text,"Configure WorkBench")
            compare(findChild(workbench,"configureWorkbench").enabled,true)
            fileBackend.openResponse={cancelled:true};workbench.openProject()
            compare(workbench.hasProject,false)
            workbench.saveProject(false);compare(workbench.hasProject,false)
            fileBackend.saveResponse={saved:true,path:"created.merlin.json"};workbench.saveProject(false)
            compare(workbench.hasProject,true);keyClick(Qt.Key_Escape)
            workbench.hasProject=false;fileBackend.saveResponse={cancelled:true}
            fileBackend.openResponse={error:"Invalid WB"};workbench.openProject()
            compare(workbench.hasProject,false)
            keyClick(Qt.Key_Escape)
            findChild(appMenu,"newWorkbench").action.trigger()
            compare(workbench.hasProject,true)
            compare(findChild(workbench,"configureWorkbench").enabled,true)
            compare(workbench.allBlocksComplete,false)
            profileBackend.acceptedId=0;workbench.refreshProfileState()
            mouseClick(findChild(workbench,"configureWorkbench"))
            const dialog=workbench.configurationPopup
            tryCompare(dialog,"opened",true)
            mouseClick(findChild(dialog,"wbProfilesTab"));wait(50)
            compare(dialog.selectedIds.join(","),"0")
            mouseClick(findChild(dialog,"wbSelectAllProfiles"))
            compare(dialog.selectedIds.join(","),"0,1")
            mouseClick(findChild(dialog,"wbProfileChecks").itemAt(0))
            compare(dialog.selectedIds.join(","),"1")
            mouseClick(findChild(dialog,"wbConfigurationApply"))
            tryCompare(dialog,"visible",false)
            compare(runBackend.acceptedRunOptions.selected_profile_ids.join(","),"1")
            mouseClick(findChild(workbench,"configureWorkbench"))
            tryCompare(dialog,"opened",true)
            compare(dialog.selectedIds.join(","),"1")
            mouseClick(findChild(dialog,"wbProfilesTab"));wait(50)
            mouseClick(findChild(dialog,"wbClearProfiles"))
            compare(dialog.selectedIds.length,0)
            mouseClick(findChild(dialog,"dialogCancel"));tryCompare(dialog,"visible",false)
            compare(runBackend.acceptedRunOptions.selected_profile_ids.join(","),"1")
            runBackend.busy=true;workbench.pollRun()
            compare(findChild(workbench,"configureWorkbench").enabled,false)
            runBackend.busy=false;workbench.pollRun()
            const state={opened:true,path:"test.merlin.json",transmission:JSON.parse(backend.current()),receiver:runBackend.acceptedRunOptions,verified:false,dark:true}
            workbench.hasProject=false;fileBackend.openResponse=state;workbench.openProject()
            compare(workbench.hasProject,true)
            compare(findChild(workbench,"configureWorkbench").enabled,true)
            fileBackend.openResponse={cancelled:true}
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
        function test_receiver_profile_performance_navigation_and_scope() {
            const receiver=workbench.receiverPopup
            receiverBackend.retainedCells=true
            receiver.open();tryCompare(receiver,"opened",true)
            mouseClick(findChild(receiver,"receiverProfilePerformance"))
            const dialog=receiver.performancePopup
            tryCompare(dialog,"opened",true)
            compare(dialog.viewport.profile_id,3)
            compare(dialog.viewport.fec.total_cw,10)
            compare(dialog.viewport.status,"cancelled")
            dialog.profileId=0;dialog.refresh(10,20)
            compare(dialog.viewport.profile_id,0)
            compare(dialog.viewport.start,10);compare(dialog.viewport.count,20)
            mouseClick(findChild(dialog,"performanceFullSpan"));compare(dialog.viewport.count,100)
            mouseClick(findChild(dialog,"popupCloseButton"));tryCompare(dialog,"visible",false)
            receiver.close();receiverBackend.retainedCells=false
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
        function test_channel_stages_tap_routing_and_loss_draft_cancel() {
            const dialog=workbench.channelPopup
            dialog.open();tryCompare(dialog,"opened",true)
            mouseClick(findChild(dialog,"channelRxmer"));tryCompare(workbench.rxmerPopup,"opened",true)
            closeWithX(workbench.rxmerPopup)
            mouseClick(findChild(dialog,"channelEstimate"));tryCompare(workbench.responsePopup,"opened",true)
            closeWithX(workbench.responsePopup)
            mouseClick(findChild(dialog,"channelInput"));tryCompare(dialog.instrumentPopup,"opened",true)
            mouseClick(findChild(dialog.instrumentPopup,"channelQam"));tryCompare(dialog.inputQamPopup,"opened",true)
            compare(dialog.inputQamPopup.observation.available,false)
            compare(findChild(dialog.inputQamPopup,"tapQamScatter").receivedSamples,false)
            closeWithX(dialog.inputQamPopup)
            mouseClick(findChild(dialog,"channelInput"));tryCompare(dialog.instrumentPopup,"opened",true)
            compare(dialog.output,false)
            mouseClick(findChild(dialog.instrumentPopup,"channelSpectrum"));tryCompare(dialog.spectrumPopup,"opened",true)
            closeWithX(dialog.spectrumPopup)
            mouseClick(findChild(dialog,"channelOutput"));tryCompare(dialog.instrumentPopup,"opened",true)
            compare(dialog.output,true)
            mouseClick(findChild(dialog.instrumentPopup,"channelQam"));tryCompare(dialog.outputQamPopup,"opened",true)
            compare(dialog.outputQamPopup.tap,2)
            compare(findChild(dialog.outputQamPopup,"tapQamScatter").receivedSamples,true)
            compare(findChild(dialog.outputQamPopup,"receiverQamView").visible,false)
            closeWithX(dialog.outputQamPopup)
            mouseClick(findChild(dialog,"channelModel"));tryCompare(dialog.modelPopup,"opened",true)
            mouseClick(findChild(dialog.modelPopup,"channelConfigureLoss"));tryCompare(dialog.lossPopup,"opened",true)
            const mode=findChild(dialog.lossPopup,"channelLossMode")
            mode.currentIndex=2;mode.activated(2)
            const length=findChild(dialog.lossPopup,"channelCableLength")
            length.text="100";dialog.lossPopup.refresh()
            closeWithX(dialog.lossPopup)
            compare(measurementBackend.loss.mode,"off")
            mouseClick(findChild(dialog.modelPopup,"channelConfigureLoss"));tryCompare(dialog.lossPopup,"opened",true)
            compare(mode.currentIndex,0)
            mode.currentIndex=2;mode.activated(2);length.text="100";dialog.lossPopup.refresh()
            mouseClick(findChild(dialog.lossPopup,"channelLossApply"));tryCompare(dialog.lossPopup,"visible",false)
            compare(measurementBackend.loss.length,100)
            measurementBackend.loss={mode:"off"}
            closeWithX(dialog.modelPopup);closeWithX(dialog)
        }
        function test_channel_power_controls_cancel_apply_and_tap_symmetry() {
            const dialog=workbench.channelPopup
            dialog.open();tryCompare(dialog,"opened",true)
            mouseClick(findChild(dialog,"channelAmplifier"));tryCompare(dialog.amplifierPopup,"opened",true)
            findChild(dialog.amplifierPopup,"channelAmplifierGain").text="12"
            closeWithX(dialog.amplifierPopup)
            compare(measurementBackend.power.amplifier_gain_db,0)
            mouseClick(findChild(dialog,"channelAmplifier"));tryCompare(dialog.amplifierPopup,"opened",true)
            findChild(dialog.amplifierPopup,"channelAmplifierGain").text="12"
            mouseClick(findChild(dialog.amplifierPopup,"channelAmplifierApply"));tryCompare(dialog.amplifierPopup,"visible",false)
            compare(measurementBackend.power.amplifier_gain_db,12)
            mouseClick(findChild(dialog,"channelAttenuator"));tryCompare(dialog.attenuatorPopup,"opened",true)
            findChild(dialog.attenuatorPopup,"channelAttenuation").text="6"
            mouseClick(findChild(dialog.attenuatorPopup,"channelAttenuatorApply"));tryCompare(dialog.attenuatorPopup,"visible",false)
            compare(measurementBackend.power.attenuation_db,6)
            const before=findChild(dialog,"channelTapBefore"),amp=findChild(dialog,"channelAmplifier"),after=findChild(dialog,"channelTapAfter")
            compare(amp.x-before.x-before.width,after.x-amp.x-amp.width)
            compare(before.y+before.height/2,amp.y+amp.height/2)
            compare(after.y+after.height/2,amp.y+amp.height/2)
            const originalWidth=window.width,originalHeight=window.height
            for (const size of [[960,640],[1240,780],[1680,1000]]) {
                window.width=size[0];window.height=size[1];wait(30)
                const stage=findChild(dialog,"channelStage")
                const blocks=[findChild(dialog,"channelInput"),before,amp,after,findChild(dialog,"channelModel"),findChild(dialog,"channelAttenuator"),findChild(dialog,"channelOutput")]
                const gaps=blocks.slice(1).map(function(block,index){return block.x-blocks[index].x-blocks[index].width})
                verify(Math.min.apply(null,gaps)>=24,"Room for connectors and captions at "+size[0])
                for(const gap of gaps){verify(Math.abs(gap-gaps[0])<0.1)}
                verify(Math.abs(blocks[0].x-(stage.width-blocks[6].x-blocks[6].width))<0.1)
                verify((blocks[6].x+blocks[6].width-blocks[0].x)/stage.width>0.85,"Use the available width")
                compare(findChild(dialog,"channelRxmer").width,findChild(dialog,"channelEstimate").width)
                const upper=findChild(dialog,"channelRxmer"),model=findChild(dialog,"channelModel"),lower=findChild(dialog,"channelEstimate")
                verify(Math.abs(model.y-upper.y-upper.height-(lower.y-model.y-model.height))<0.1)
            }
            window.width=originalWidth;window.height=originalHeight;wait(30)
            mouseClick(after);tryCompare(dialog.instrumentPopup,"opened",true)
            compare(dialog.tapIndex,1)
            mouseClick(findChild(dialog.instrumentPopup,"channelQam"));tryCompare(dialog.inputQamPopup,"opened",true)
            compare(dialog.inputQamPopup.tap,1)
            compare(findChild(dialog.inputQamPopup,"tapQamScatter").receivedSamples,false)
            closeWithX(dialog.inputQamPopup)
            measurementBackend.power={amplifier_gain_db:0,attenuation_db:0,input_data_level_dbmv_per_6mhz:null,amplifier_target_dbmv_per_6mhz:null,receiver_noise_dbmv_hz:null}
            closeWithX(dialog)
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
