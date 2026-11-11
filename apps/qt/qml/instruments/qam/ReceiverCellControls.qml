/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../components"
ColumnLayout {
    id: controls
    required property var theme
    property var metadata: ({})
    property int signalIndex: 0
    property int viewIndex: 1
    property int plcPartIndex: 0
    property int profile: -1
    property int order: 0
    property int firstBin: -1
    property int lastBin: -1
    property int firstSymbol: -1
    property int lastSymbol: -1
    property bool showView: true
    property bool resetting: false
    signal selectionChanged()
    function query() { return {signal:signalIndex,view:viewIndex,part:plcPartIndex,profile:profile,order:order,first_bin:firstBin,last_bin:lastBin,first_symbol:firstSymbol,last_symbol:lastSymbol} }
    function resetFilters() {
        resetting=true;profile=-1;order=0;firstBin=-1;lastBin=-1;firstSymbol=-1;lastSymbol=-1;resetting=false
        selectionChanged()
    }
    onSignalIndexChanged: resetFilters()
    onPlcPartIndexChanged: resetFilters()
    onViewIndexChanged: { if (!resetting) { selectionChanged() } }
    spacing: 8
    RowLayout {
        Layout.fillWidth:true
        Label { text:"Signal";color:controls.theme.text }
        ComboBox { objectName:"receiverQamSignal";model:["Data Profile","NCP","PLC","Continuous Pilots","Scattered Pilots"];currentIndex:controls.signalIndex;Layout.preferredWidth:180;onActivated:function(index){controls.signalIndex=index} }
        Label { visible:controls.showView;text:"QAM View";color:controls.theme.text }
        ComboBox { objectName:"receiverQamView";visible:controls.showView;model:["Raw","Equalized","Paired"];currentIndex:controls.viewIndex;Layout.preferredWidth:135;onActivated:function(index){controls.viewIndex=index} }
        Label { visible:controls.signalIndex===2;text:"PLC Part";color:controls.theme.text }
        ComboBox { objectName:"receiverPlcPart";visible:controls.signalIndex===2;model:["Data","Preamble"];currentIndex:controls.plcPartIndex;onActivated:function(index){controls.plcPartIndex=index} }
        Item { Layout.fillWidth:true }
        WorkbenchActionButton { theme: controls.theme; text:"Reset Filters";onClicked:controls.resetFilters() }
    }
    RowLayout {
        Layout.fillWidth:true
        Label { visible:controls.signalIndex===0;text:"Received Profile";color:controls.theme.muted }
        ComboBox {
            objectName:"receiverProfile";visible:controls.signalIndex===0
            model:["All received"].concat((controls.metadata.profiles || []).map(function(id){return "Profile "+id}))
            enabled:(controls.metadata.profiles || []).length>0
            currentIndex:controls.profile<0 ? 0 : (controls.metadata.profiles || []).indexOf(controls.profile)+1
            Layout.preferredWidth:160
            onActivated:function(index){controls.profile=index===0 ? -1 : controls.metadata.profiles[index-1];controls.order=0;controls.selectionChanged()}
        }
        Label { text:"Modulation";color:controls.theme.muted }
        ComboBox {
            objectName:"receiverModulation";model:(controls.metadata.orders || []).map(function(order){return order+" QAM"})
            displayText:(controls.metadata.orders || []).length ? currentText : "No available group"
            currentIndex:controls.order===0 ? 0 : (controls.metadata.orders || []).indexOf(controls.order)
            Layout.preferredWidth:170;enabled:(controls.metadata.orders || []).length>0
            onActivated:function(index){controls.order=controls.metadata.orders[index];controls.selectionChanged()}
        }
        Item { Layout.fillWidth:true }
    }
    GridLayout {
        columns:8;columnSpacing:8;rowSpacing:4;Layout.fillWidth:true
        Label { text:"First Bin";color:controls.theme.muted }
        SpinBox { objectName:"receiverFirstBin";from:-1;to:8191;editable:true;value:controls.firstBin;textFromValue:function(v){return v<0 ? "All" : v.toString()};valueFromText:function(t){return t==="All" ? -1 : Number(t)};Layout.preferredWidth:105;onValueModified:{controls.firstBin=value;controls.selectionChanged()} }
        Label { text:"Last Bin";color:controls.theme.muted }
        SpinBox { objectName:"receiverLastBin";from:-1;to:8191;editable:true;value:controls.lastBin;textFromValue:function(v){return v<0 ? "All" : v.toString()};valueFromText:function(t){return t==="All" ? -1 : Number(t)};Layout.preferredWidth:105;onValueModified:{controls.lastBin=value;controls.selectionChanged()} }
        Label { text:"First RX Symbol";color:controls.theme.muted }
        SpinBox { objectName:"receiverFirstSymbol";from:-1;to:1000000;editable:true;value:controls.firstSymbol;textFromValue:function(v){return v<0 ? "All" : v.toString()};valueFromText:function(t){return t==="All" ? -1 : Number(t)};Layout.preferredWidth:115;onValueModified:{controls.firstSymbol=value;controls.selectionChanged()} }
        Label { text:"Last RX Symbol";color:controls.theme.muted }
        SpinBox { objectName:"receiverLastSymbol";from:-1;to:1000000;editable:true;value:controls.lastSymbol;textFromValue:function(v){return v<0 ? "All" : v.toString()};valueFromText:function(t){return t==="All" ? -1 : Number(t)};Layout.preferredWidth:115;onValueModified:{controls.lastSymbol=value;controls.selectionChanged()} }
    }
    Label { text:metadata.retained ? "Available: "+metadata.retained+" cells · bins "+metadata.first_bin+"–"+metadata.last_bin+" · RX symbols "+metadata.first_symbol+"–"+metadata.last_symbol : "No data for this signal";color:controls.theme.muted;font.pixelSize:11;Layout.fillWidth:true;wrapMode:Text.WordWrap }
}
