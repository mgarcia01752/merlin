/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    id: table
    required property var theme
    property var words: []
    property int omitted: 0
    property bool errorsOnly: false
    readonly property var filtered: words.filter(word => !errorsOnly || !word.ldpc_success || !word.bch_success || (word.comparison && word.comparison.bit_errors > 0))
    readonly property var headings: ["Word", "Source Symbols", "LDPC", "Iterations", "BCH", "BCH Corrections", "Bit Errors"]
    RowLayout {
        Layout.fillWidth: true
        Label { text: "Whole run · " + table.words.length + " retained · " + table.omitted + " omitted"; color: table.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        CheckBox { objectName: "fecErrorsOnly"; text: "Errors Only"; checked: table.errorsOnly; onToggled: table.errorsOnly = checked }
    }
    Row {
        Layout.fillWidth: true
        Repeater {
            model: table.headings
            Label { required property string modelData; width: table.width / table.headings.length; text: modelData; color: table.theme.accent; font.bold: true; wrapMode: Text.WordWrap }
        }
    }
    ListView {
        objectName: "receiverFecWords"
        Layout.fillWidth: true; Layout.fillHeight: true; clip: true
        model: table.filtered
        ScrollBar.vertical: ScrollBar {}
        delegate: Row {
            required property var modelData
            width: ListView.view.width; height: 32
            property var values: [modelData.receive_sequence, modelData.first_source_symbol + "–" + modelData.last_source_symbol, modelData.ldpc_success ? "Pass" : "Fail", modelData.ldpc_iterations, modelData.bch_success ? "Pass" : "Fail", modelData.bch_corrected_bits, modelData.comparison ? modelData.comparison.bit_errors : "—"]
            Repeater {
                model: parent.values
                Label { required property var modelData; width: table.width / table.headings.length; text: modelData; color: text === "Fail" ? table.theme.statusError : table.theme.text }
            }
        }
        Label { anchors.centerIn: parent; visible: !table.filtered.length; text: table.errorsOnly ? "No errors in retained records." : "No retained codewords."; color: table.theme.muted }
    }
}
