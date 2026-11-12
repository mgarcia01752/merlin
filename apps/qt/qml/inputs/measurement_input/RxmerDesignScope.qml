/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
RowLayout {
    id: scope
    required property var theme
    property int captureCount: 1
    property alias range: range.checked
    property alias first: first.text
    property alias last: last.text
    signal edited()
    function value() { return range.checked ? {mode:"range",first:Number(first.text),last:Number(last.text)} : {mode:"all"} }
    function restore(value) { range.checked=value.mode === "range";first.text=String(value.first || 1);last.text=String(value.last || captureCount) }
    CheckBox { id:range; text:"Capture range"; onToggled:scope.edited(); ToolTip.visible:hovered || activeFocus; ToolTip.text:"Unchecked applies to every capture. Checked uses one-based inclusive capture numbers; equal numbers select one capture." }
    Label { visible:!range.checked; text:"All "+scope.captureCount+" captures";color:scope.theme.muted }
    TextField { id:first; visible:range.checked; text:"1"; Layout.preferredWidth:55; validator:IntValidator {bottom:1;top:256}
 Accessible.name:"First capture"; onEditingFinished:scope.edited() }
    Label {visible:range.checked;text:"to";color:scope.theme.muted}
    TextField { id:last; visible:range.checked; text:"1"; Layout.preferredWidth:55; validator:IntValidator {bottom:1;top:256}
 Accessible.name:"Last capture"; onEditingFinished:scope.edited() }
}
