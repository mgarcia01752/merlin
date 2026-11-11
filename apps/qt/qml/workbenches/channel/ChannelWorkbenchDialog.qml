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
    property alias amplifierPopup:amplifier
    property alias attenuatorPopup:attenuator
    property alias instrumentPopup:instruments
    property alias inputQamPopup:qam
    property alias outputQamPopup:qam
    property alias spectrumPopup:spectrum
    property int tapIndex:0
    property bool output:tapIndex===2
    property var power:({amplifier_gain_db:0,attenuation_db:0})
    readonly property string tapName:["Before Amplifier","After Amplifier","Channel Output / RX Input"][tapIndex]
    property var spectrumState:({has_result:false,can_generate:false})
    property string seenToken:""
    function refreshPower(){if(measurementBackend){power=JSON.parse(measurementBackend.power_current())}}
    function settingsApplied(){refreshPower();modelApplied()}
    function openTap(index){tapIndex=index;instruments.open()}
    function refreshSpectrum(){if(measurementBackend){spectrumState=JSON.parse(measurementBackend.tap_state(tapIndex))}}
    function openSpectrum(){refreshSpectrum();spectrum.clearView();spectrum.open()}
    function openQam(){qam.tap=tapIndex;qam.tapName=tapName;qam.open()}
    onOpened:refreshPower()
    onRunStateChanged: {
        if(seenToken!==(runState.result_token || "")){
            seenToken=runState.result_token || "";refreshSpectrum()
            if(spectrum.visible){spectrum.clearView();spectrum.updateResult()}
            if(qam.visible){qam.refresh()}
        }
    }
    title:"Channel Simulator Workbench"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1100,parent ? parent.width-40 : 1100)
    height:Math.min(560,parent ? parent.height-40 : 560)
    modal:true;standardButtons:Dialog.Close
    contentItem:Item {
        Item {
            id:stage;anchors.centerIn:parent;width:900;height:380
            scale:Math.min(1,parent.width/width,parent.height/height)
            readonly property real blockWidth:110
            readonly property real gap:16
            readonly property real tpSize:36
            readonly property real symbolWidth:70
            readonly property real modelWidth:130
            readonly property real pathWidth:2*blockWidth+modelWidth+2*tpSize+2*symbolWidth+6*gap
            readonly property real pathLeft:(width-pathWidth)/2
            readonly property real rowY:155
            readonly property real centerY:195
            readonly property real inputX:pathLeft
            readonly property real tp1X:inputX+blockWidth+gap
            readonly property real ampX:tp1X+tpSize+gap
            readonly property real tp2X:ampX+symbolWidth+gap
            readonly property real modelX:tp2X+tpSize+gap
            readonly property real attenX:modelX+modelWidth+gap
            readonly property real outputX:attenX+symbolWidth+gap
            readonly property real modelCenter:modelX+modelWidth/2
            Repeater {
                model:[[stage.inputX+stage.blockWidth,stage.tp1X],[stage.tp1X+stage.tpSize,stage.ampX],[stage.ampX+stage.symbolWidth,stage.tp2X],[stage.tp2X+stage.tpSize,stage.modelX],[stage.modelX+stage.modelWidth,stage.attenX],[stage.attenX+stage.symbolWidth,stage.outputX]]
                delegate:Connection {required property var modelData;anchors.fill:parent;theme:dialog.theme;fromX:modelData[0];fromY:stage.centerY;toX:modelData[1];toY:stage.centerY}
            }
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:stage.modelCenter;fromY:82;toX:stage.modelCenter;toY:stage.rowY}
            Connection {anchors.fill:parent;theme:dialog.theme;fromX:stage.modelCenter;fromY:308;toX:stage.modelCenter;toY:235}
            ReceiverChip {objectName:"channelRxmer";theme:dialog.theme;stage:true;text:"RxMER";x:stage.modelX;y:2;width:stage.modelWidth;height:80;onClicked:dialog.rxmerRequested()}
            ReceiverChip {objectName:"channelInput";theme:dialog.theme;stage:true;text:"Channel Input";x:stage.inputX;y:stage.rowY;width:stage.blockWidth;height:80;onClicked:dialog.openTap(0)}
            Button {objectName:"channelTapBefore";x:stage.tp1X;y:stage.centerY-stage.tpSize/2;width:stage.tpSize;height:stage.tpSize;text:"TP";Accessible.name:"TapPoint Before Amplifier";onClicked:dialog.openTap(0);background:Rectangle {radius:width/2;color:dialog.theme.panel;border.color:dialog.theme.accent} }
            ConditioningSymbol {objectName:"channelAmplifier";theme:dialog.theme;x:stage.ampX;y:stage.rowY;width:stage.symbolWidth;height:80;valueText:dialog.power.amplifier_target_dbmv_per_6mhz===null || dialog.power.amplifier_target_dbmv_per_6mhz===undefined ? Number(dialog.power.amplifier_gain_db || 0)+" dB" : "Level Target";onClicked:amplifier.open()}
            Button {objectName:"channelTapAfter";x:stage.tp2X;y:stage.centerY-stage.tpSize/2;width:stage.tpSize;height:stage.tpSize;text:"TP";Accessible.name:"TapPoint After Amplifier";onClicked:dialog.openTap(1);background:Rectangle {radius:width/2;color:dialog.theme.panel;border.color:dialog.theme.accent} }
            ReceiverChip {objectName:"channelModel";theme:dialog.theme;stage:true;text:"Channel Model";x:stage.modelX;y:stage.rowY;width:stage.modelWidth;height:80;onClicked:model.open()}
            ConditioningSymbol {objectName:"channelAttenuator";theme:dialog.theme;amplifier:false;x:stage.attenX;y:stage.rowY;width:stage.symbolWidth;height:80;valueText:Number(dialog.power.attenuation_db || 0)+" dB";onClicked:attenuator.open()}
            ReceiverChip {objectName:"channelOutput";theme:dialog.theme;stage:true;text:"Channel Output";x:stage.outputX;y:stage.rowY;width:stage.blockWidth;height:80;onClicked:dialog.openTap(2)}
            ReceiverChip {objectName:"channelEstimate";theme:dialog.theme;stage:true;text:"DS Channel Estimate";x:stage.modelX;y:308;width:stage.modelWidth;height:80;onClicked:dialog.responseRequested()}
        }
    }
    ChannelConfigurationDialog {id:model;theme:dialog.theme;configuration:dialog.configuration;onRxmerRequested:dialog.rxmerRequested();onResponseRequested:dialog.responseRequested();onLossRequested:loss.open()}
    ChannelLossDialog {id:loss;theme:dialog.theme;backend:dialog.measurementBackend;settingsBackend:dialog.settingsBackend;busy:dialog.runState.busy;onModelApplied:dialog.settingsApplied()}
    ChannelAmplifierDialog {id:amplifier;theme:dialog.theme;backend:dialog.measurementBackend;busy:dialog.runState.busy;onModelApplied:dialog.settingsApplied()}
    ChannelAttenuatorDialog {id:attenuator;theme:dialog.theme;backend:dialog.measurementBackend;busy:dialog.runState.busy;onModelApplied:dialog.settingsApplied()}
    ChannelInstrumentDialog {id:instruments;theme:dialog.theme;output:dialog.output;title:dialog.tapName+" · Instruments";onSpectrumRequested:dialog.openSpectrum();onQamRequested:dialog.openQam()}
    TapQamDialog {id:qam;theme:dialog.theme;backend:dialog.measurementBackend}
    QtObject {
        id:spectrumProxy
        function cancel(){}
        function view(start,count,captureAxis){return dialog.measurementBackend.tap_spectrum(dialog.tapIndex,start,count,captureAxis)}
    }
    SpectrumAnalyzerDialog {id:spectrum;theme:dialog.theme;backend:spectrumProxy;state:dialog.spectrumState;autoGenerate:false;showTiming:false;tapOptions:[dialog.tapName];selectedTap:0;generateDescription:"Use Start on the main workbench to run the simulation."}
}
