/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/instruments"

ColumnLayout {
    id: table
    required property var theme
    // Original words and both independent row variants share the parent witness.
    property var projection: ({})
    property bool errorsOnly: false
    readonly property var filtered: errorsOnly ? projection.filtered || [] : projection.words || []
    readonly property var formattedRows: errorsOnly ? projection.errors || [] : projection.all || []
    readonly property var headings: projection.headings || []

    /**
     * Prepare both original codeword filters and every converted table string.
     * @param words Original ordered codewords; no numerical fields are changed.
     * @param omitted Existing bounded omission count from the original FEC DTO.
     * @returns Plain projection; parent admits it before Repeater/ListView use.
     */
    function prepare(words, omitted) {
        const text = function(value){return value===undefined || value===null ? "" : String(value)}
        const rows = words.map(function(word){return [text(word.receive_sequence),
            word.first_source_symbol+"–"+word.last_source_symbol,
            word.ldpc_success ? "Pass" : "Fail",text(word.ldpc_iterations),
            word.bch_success ? "Pass" : "Fail",text(word.bch_corrected_bits),
            word.comparison ? text(word.comparison.bit_errors) : "—"]})
        return {words:words,filtered:words.filter(function(word){return !word.ldpc_success || !word.bch_success || (word.comparison && word.comparison.bit_errors>0)}),headings:["Word","Source Symbols","LDPC","Iterations","BCH","BCH Corrections","Bit Errors"],
            all:rows,errors:rows.filter(function(row,index){const word=words[index];return !word.ldpc_success || !word.bch_success || (word.comparison && word.comparison.bit_errors>0)}),
            summary:"Whole run · "+words.length+" retained · "+omitted+" omitted"}
    }
    RowLayout {
        Layout.fillWidth: true
        Label { text:table.projection.summary || ""; color: table.theme.muted; Layout.fillWidth: true; wrapMode: Text.WordWrap }
        CheckBox { objectName: "fecErrorsOnly"; text: "Errors Only"; checked: table.errorsOnly; onToggled: table.errorsOnly = checked }
    }
    ObservationTable {
        Layout.fillWidth: true
        Layout.fillHeight: true
        theme: table.theme
        headings: table.headings
        rows: table.formattedRows
        rowsObjectName: "receiverFecWords"
        emptyText: table.errorsOnly ? "No errors in retained records." : "No retained codewords."
    }
}
