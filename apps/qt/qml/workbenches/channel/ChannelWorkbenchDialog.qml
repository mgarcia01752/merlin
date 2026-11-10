/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
import "../../instruments/qam"
import "../../instruments/spectrum"
WorkbenchDialog {
    id:dialog
    required property var configuration
    required property var measurementBackend
    required property var settingsBackend
    required property var transmitterBackend
    required property var receiverBackend
    required property var runState
    signal rxmerRequested()
    signal responseRequested()
    signal modelApplied()
    property alias modelPopup:model
    property alias lossPopup:loss
    property alias instrumentPopup:instruments
    property alias inputQamPopup:inputQam
    property alias outputQamPopup:outputQam
    property alias spectrumPopup:spectrum
    property bool output:false
    property var spectrumState:({has_result:false,can_generate:false})
    property string seenToken:""
    function openTap(isOutput) {output=isOutput;instruments.open()}
    function refreshSpectrum() {
        if (!receiverBackend || !transmitterBackend) {return}
        spectrumState=JSON.parse(output ? receiverBackend.spectrum(0) : transmitterBackend.spectrum_tap(3))
    }
    function openSpectrum() {refreshSpectrum();spectrum.clearView();spectrum.open()}
    function openQam() {if(output){outputQam.viewIndex=0;outputQam.open()}else{inputQam.open()}}
    onRunStateChanged: {
        if (seenToken!==(runState.result_token || "")) {
            seenToken=runState.result_token || ""
            refreshSpectrum()
            if(spectrum.visible){spectrum.clearView();spectrum.updateResult()}
            if(inputQam.visible){inputQam.refresh()}
            if(outputQam.visible){outputQam.refresh()}
        }
    }
    title:"Channel Simulator Workbench"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1040,parent ? parent.width-40 : 1040)
    height:Math.min(560,parent ? parent.height-40 : 560)
    modal:true;standardButtons:Dialog.Close
    contentItem:Item {
        Item {
            id:stage;anchors.centerIn:parent;width:Math.min(parent.width,940);height:390
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:width/2;fromY:82;toX:width/2;toY:155}
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:width*0.25;fromY:195;toX:width*0.36;toY:195}
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:width*0.64;fromY:195;toX:width*0.75;toY:195}
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:width/2;fromY:308;toX:width/2;toY:235}
            ReceiverChip {objectName:"channelRxmer";theme:dialog.theme;stage:true;text:"RxMER";x:parent.width*0.36;y:2;width:parent.width*0.28;height:80;onClicked:dialog.rxmerRequested()}
            ReceiverChip {objectName:"channelInput";theme:dialog.theme;stage:true;text:"Channel Input";x:0;y:155;width:parent.width*0.25;height:80;onClicked:dialog.openTap(false)}
            ReceiverChip {objectName:"channelModel";theme:dialog.theme;stage:true;text:"Channel Model";x:parent.width*0.36;y:155;width:parent.width*0.28;height:80;onClicked:model.open()}
            ReceiverChip {objectName:"channelOutput";theme:dialog.theme;stage:true;text:"Channel Output";x:parent.width*0.75;y:155;width:parent.width*0.25;height:80;onClicked:dialog.openTap(true)}
            ReceiverChip {objectName:"channelEstimate";theme:dialog.theme;stage:true;text:"DS Channel Estimate";x:parent.width*0.36;y:308;width:parent.width*0.28;height:80;onClicked:dialog.responseRequested()}
        }
    }
    ChannelConfigurationDialog {id:model;theme:dialog.theme;configuration:dialog.configuration;onRxmerRequested:dialog.rxmerRequested();onResponseRequested:dialog.responseRequested();onLossRequested:loss.open()}
    ChannelLossDialog {id:loss;theme:dialog.theme;backend:dialog.measurementBackend;settingsBackend:dialog.settingsBackend;busy:dialog.runState.busy;onModelApplied:dialog.modelApplied()}
    ChannelInstrumentDialog {id:instruments;theme:dialog.theme;output:dialog.output;onSpectrumRequested:dialog.openSpectrum();onQamRequested:dialog.openQam()}
    ChannelInputQamDialog {id:inputQam;theme:dialog.theme;backend:dialog.transmitterBackend}
    ReceiverQamDialog {id:outputQam;theme:dialog.theme;backend:dialog.receiverBackend;rawOnly:true;title:"QAM Analyzer · Channel Output"}
    QtObject {
        id:spectrumProxy
        function cancel() {}
        function view(start,count,captureAxis) {return dialog.output ? dialog.receiverBackend.view_tap(0,start,count,captureAxis) : dialog.transmitterBackend.view_tap(3,start,count,captureAxis)}
    }
    SpectrumAnalyzerDialog {id:spectrum;theme:dialog.theme;backend:spectrumProxy;state:dialog.spectrumState;autoGenerate:false;showTiming:false;tapOptions:[dialog.output ? "Channel Output / RX Input" : "Channel Input / TX Output"];selectedTap:0;generateDescription:"Use Start on the main workbench to run the simulation."}
}
