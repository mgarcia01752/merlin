/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
import "../../instruments/messages"
import "../../instruments/qam"
import "../../instruments/spectrum"
import "../../instruments/stages"

WorkbenchDialog {
    id: dialog
    required property var backend
    required property var transmitterState
    required property var runState
    property alias spectrumPopup: spectrum
    property alias stagePopup: stageInspector
    property alias qamPopup: qam
    property alias messagePopup: messages
    property alias stageRepeater: stages
    property int selectedTap: 0
    readonly property bool usePreview: selectedTap === 0 && !runState.has_result && !runState.busy
    onUsePreviewChanged: if (!usePreview && backend) { backend.cancel() }
    property var retainedState: ({has_result:false,can_generate:false,busy:false})
    property string seenToken: ""
    function refreshSpectrum() { if (backend) { retainedState=JSON.parse(backend.spectrum_tap(selectedTap === 0 ? 3 : selectedTap-1)) } }
    function openSpectrum(index) { selectedTap=index;refreshSpectrum();spectrum.clearView();spectrum.open() }
    function openStage(index) { stageInspector.stageIndex=index;stageInspector.open() }
    onRunStateChanged: {
        if (seenToken !== (runState.result_token || "")) {
            seenToken=runState.result_token || "";refreshSpectrum()
            if (stageInspector.visible) { stageInspector.refresh() }
            if (qam.visible) { qam.refresh() }
            if (messages.visible) { messages.refresh() }
        }
    }
    onOpened: refreshSpectrum()
    title: "Transmitter Workbench"
    parent: Overlay.overlay; anchors.centerIn: parent
    width: Math.min(1160,parent ? parent.width-40 : 1160)
    height: Math.min(570,parent ? parent.height-40 : 570)
    modal:true;standardButtons:Dialog.Close
    contentItem: ColumnLayout {
        spacing:16
        Label { text:"Transmitter Signal Path";font.pixelSize:19;font.bold:true;color:dialog.theme.text }
        Rectangle {
            Layout.fillWidth:true;Layout.preferredHeight:204
            radius:12;color:dialog.theme.background;border.color:dialog.theme.line
            RowLayout {
                anchors.fill:parent;anchors.margins:12;spacing:5
                Repeater {
                    id:stages
                    model: ["Source\nPRBS-TX","BCH / LDPC","Bit Processing /\nRandomizer","QAM Mapper","Cell\nInterleavers","Frame\nAssembly","IFFT / CP /\nWindow / OLA"]
                    ColumnLayout {
                        required property int index
                        required property string modelData
                        property alias tapButton:tap
                        property alias outputSaButton:outputSa
                        Layout.fillWidth:true;Layout.preferredWidth:1;Layout.minimumWidth:0;spacing:8
                        RowLayout {
                            Layout.fillWidth:true;spacing:1
                            StageChip { theme:dialog.theme;text:modelData;Layout.preferredHeight:72;onClicked: index===6 ? dialog.openSpectrum(0) : dialog.openStage([0,1,2,4,5,6][index]) }
                            Label { visible:index<6;text:"→";color:dialog.theme.muted;font.pixelSize:15 }
                        }
                        ReceiverChip {
                            id:tap
                            objectName:"txStageTap"+index
                            theme:dialog.theme;text:["● Bits","● LDPC Word","● Bits / Labels","● Mapped I/Q","● Ordered Cells","● Physical Bins","● TX Output"][index]
                            Layout.preferredHeight:38
                            onClicked:index===6 ? dialog.openSpectrum(0) : dialog.openStage([0,1,2,4,5,6][index])
                        }
                        Rectangle { visible:index===6;Layout.alignment:Qt.AlignHCenter;width:1;height:10;color:dialog.theme.accent }
                        ReceiverChip { id:outputSa;visible:index===6;objectName:index===6 ? "txOutputSa" : "";theme:dialog.theme;text:"Spectrum Analyzer";Layout.preferredHeight:38;onClicked:dialog.openSpectrum(0) }
                        Item { visible:index!==6;Layout.preferredHeight:54 }
                    }
                }
            }
        }
        RowLayout {
            Layout.fillWidth:true;spacing:12
            ReceiverChip { objectName:"txMessages";theme:dialog.theme;text:"OCD / DPD Messages";Layout.minimumWidth:200;Layout.preferredWidth:230;Layout.maximumWidth:230;onClicked:messages.open() }
            Label { text:"→ PLC Encoding / Mapping → Frame Assembly";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        }
        RowLayout {
            Layout.fillWidth:true;spacing:12
            ReceiverChip { objectName:"txProtected";theme:dialog.theme;text:"NCP / Pilots";Layout.minimumWidth:200;Layout.preferredWidth:230;Layout.maximumWidth:230;onClicked:dialog.openStage(6) }
            Label { text:"Separate generation / mapping → Frame Assembly";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        }
        Label { text:"Diagnostic Instruments";color:dialog.theme.accent;font.bold:true }
        RowLayout {
            Layout.fillWidth:true
            ReceiverChip { objectName:"txQam";theme:dialog.theme;text:"QAM Analyzer";onClicked:qam.open() }
            ReceiverChip { objectName:"txMessageInspector";theme:dialog.theme;text:"Message Inspector";onClicked:messages.open() }
            ReceiverChip { objectName:"txBitInspector";theme:dialog.theme;text:"Bit / Codeword Inspector";onClicked:dialog.openStage(0) }
        }
        Item { Layout.fillHeight:true }
    }
    TransmitterStageDialog { id:stageInspector;theme:dialog.theme;backend:dialog.backend }
    TransmitterQamDialog { id:qam;theme:dialog.theme;backend:dialog.backend }
    TransmitterMessageDialog { id:messages;theme:dialog.theme;backend:dialog.backend }
    QtObject {
        id:spectrumProxy
        function generate() { if (dialog.usePreview) { dialog.backend.generate() } }
        function cancel() { if (dialog.usePreview) { dialog.backend.cancel() } }
        function view(start,count,captureAxis) { return dialog.usePreview ? dialog.backend.view(start,count,captureAxis) : dialog.backend.view_tap(dialog.selectedTap===0 ? 3 : dialog.selectedTap-1,start,count,captureAxis) }
        function timing(velocity) { return dialog.backend.timing(velocity) }
    }
    SpectrumAnalyzerDialog {
        id:spectrum;theme:dialog.theme;backend:spectrumProxy
        state:dialog.usePreview ? dialog.transmitterState : dialog.retainedState
        autoGenerate:dialog.usePreview;showTiming:true
        tapOptions:["TX Output · Before Channel","TX IFFT","TX Cyclic Prefix","TX Window"]
        selectedTap:dialog.selectedTap
        onTapRequested:function(index){dialog.selectedTap=index;dialog.refreshSpectrum();clearView();updateResult()}
        generateDescription:dialog.usePreview ? "Run the transmitter preview." : "Use Start on the main workbench to capture TX stages."
    }
}
