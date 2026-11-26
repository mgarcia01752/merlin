/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../shared/presentation"
RowLayout {
    id: scope
    required property var theme
    property int captureCount: 1
    required property int maximumCaptures
    property alias range: range.checked
    property alias first: first.text
    property alias last: last.text
    signal edited()
    property PresentationOwner presentationCoverage: PresentationOwner { id: coverage; theme: scope.theme }
    readonly property bool textAdmitted: coverage.textAdmitted
    function reserveText() { return coverage.reserveTextFields([first, last]) }
    /** Count the independently formatted scope label under this same owner. */
    function refreshLabel() {return coverage.replace({all:"All "+scope.captureCount+" captures"})}
    onCaptureCountChanged: if (coverage) {refreshLabel()}
    Component.onCompleted: {reserveText();refreshLabel()}
    function value() { return range.checked ? {mode:"range",first:Number(first.text),last:Number(last.text)} : {mode:"all"} }
    function restore(value) {
        if (!reserveText()) {
            return false
        }
        range.checked = value.mode === "range"
        first.text = String(value.first || 1)
        last.text = String(value.last || captureCount)
        return true
    }
    CheckBox { id:range; enabled:scope.textAdmitted; text:"Capture range"; onToggled:scope.edited(); ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible:hovered || visualFocus; ToolTip.text:"Unchecked applies to every capture. Checked uses one-based inclusive capture numbers; equal numbers select one capture." }
    Label { visible:!range.checked; text:coverage.value ? coverage.value.all : "";color:scope.theme.muted }
    TextField { id:first; visible:range.checked; enabled:scope.textAdmitted; text:scope.textAdmitted ? "1" : ""; Layout.preferredWidth:55; validator:IntValidator {bottom:1;top:scope.maximumCaptures}
 Accessible.name:"First capture"; onEditingFinished:scope.edited() }
    Label {visible:range.checked;text:"to";color:scope.theme.muted}
    TextField { id:last; visible:range.checked; enabled:scope.textAdmitted; text:scope.textAdmitted ? "1" : ""; Layout.preferredWidth:55; validator:IntValidator {bottom:1;top:scope.maximumCaptures}
 Accessible.name:"Last capture"; onEditingFinished:scope.edited() }
    Label { visible:coverage.rejected; text:coverage.message; color:scope.theme.statusAttention; wrapMode:Text.WordWrap }
}
