/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
WorkbenchDialog {
    id:dialog
    required property var backend
    property var runState: ({})
    property string seenToken:""
    property var statistics: ({available:false,words:[]})
    function refresh(){if (backend){statistics=JSON.parse(backend.fec())} }
    function value(v){return v===undefined || v===null ? "Unavailable" : String(v)}
    onOpened:refresh()
    onRunStateChanged:{if (opened && runState.result_token!==seenToken){seenToken=runState.result_token || "";refresh()} }
    title:"FEC Monitor"
    parent:Overlay.overlay;anchors.centerIn:parent
    width:Math.min(1080,parent ? parent.width-40 : 1080);height:Math.min(690,parent ? parent.height-40 : 690)
    modal:true;standardButtons:Dialog.Close
    contentItem:ColumnLayout {
        spacing:10
        Label { objectName:"receiverFecStatus";text:dialog.statistics.available ? "Run "+dialog.statistics.status : dialog.statistics.reason || "Run the simulation first";color:dialog.theme.accent;font.bold:true;Layout.fillWidth:true;wrapMode:Text.WordWrap }
        GridLayout {
            visible:dialog.statistics.available;columns:3;columnSpacing:22;Layout.fillWidth:true
            Label { text:"Payload BCH / LDPC\nGenerated: "+dialog.value(dialog.statistics.generated)+"\nRecovered: "+dialog.value(dialog.statistics.payloads)+"\nLDPC failures: "+dialog.value(dialog.statistics.ldpc_failures)+"\nBCH failures: "+dialog.value(dialog.statistics.bch_failures)+"\nBCH corrected bits: "+dialog.value(dialog.statistics.bch_corrected_bits)+"\nMean LDPC iterations: "+dialog.value(dialog.statistics.mean_iterations);color:dialog.theme.text;Layout.fillWidth:true }
            Label { text:"Protected Control\nCRC-accepted NCP fields: "+dialog.value(dialog.statistics.ncp_fields)+"\nNCP LDPC iterations (total): "+dialog.value(dialog.statistics.ncp_iterations)+"\nPLC frames: "+dialog.value(dialog.statistics.plc_frames)+"\nPLC LDPC iterations (total): "+dialog.value(dialog.statistics.plc_iterations)+"\nPLC compared bit errors: "+dialog.value(dialog.statistics.plc_bit_errors);color:dialog.theme.text;Layout.fillWidth:true }
            Label { text:"Independent Payload Comparison\nCompared bits: "+dialog.value(dialog.statistics.compared_bits)+"\nBit errors: "+dialog.value(dialog.statistics.bit_errors)+"\nBER: "+dialog.value(dialog.statistics.ber)+"\nCodeword errors: "+dialog.value(dialog.statistics.codeword_errors)+"\nUnrecovered generated: "+dialog.value(dialog.statistics.unrecovered);color:dialog.theme.text;Layout.fillWidth:true }
        }
        SpectrumPlot { theme:dialog.theme;viewport:dialog.statistics.histogram || ({points:[],x_ticks:[],y_ticks:[],error:dialog.statistics.reason});Layout.fillWidth:true;Layout.fillHeight:true;Layout.minimumHeight:100 }
        Label { text:"Completed Payload Words · first 64 retained · omitted: "+dialog.value(dialog.statistics.omitted_words);color:dialog.theme.accent;font.bold:true }
        ListView {
            objectName:"receiverFecWords";model:dialog.statistics.words || [];Layout.fillWidth:true;Layout.preferredHeight:115;clip:true
            ScrollBar.vertical:ScrollBar {}
            delegate:Label {
                required property var modelData
                width:ListView.view.width;height:25;color:dialog.theme.text;font.pixelSize:11
                text:"Word "+modelData.receive_sequence+" · source "+modelData.first_source_symbol+"–"+modelData.last_source_symbol+" · LDPC "+(modelData.ldpc_success ? "Pass" : "Fail")+" / "+modelData.ldpc_iterations+" iterations · BCH "+(modelData.bch_success ? "Pass" : "Fail")+" · corrected "+modelData.bch_corrected_bits+" · bit errors "+modelData.comparison.bit_errors
                elide:Text.ElideRight
            }
        }
        Label { visible:text.length>0;text:dialog.statistics.error ? JSON.stringify(dialog.statistics.error) : dialog.statistics.available && dialog.statistics.ber===null ? "BER unavailable: no compared bits." : "";color:dialog.theme.muted;Layout.fillWidth:true;wrapMode:Text.WordWrap;font.pixelSize:11 }
    }
}
