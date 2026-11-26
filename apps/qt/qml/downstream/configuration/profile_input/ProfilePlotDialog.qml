/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
// Adapted from PNM Viewer ModulationControls.qml and ProfileBandwidth.qml.
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../../shared/components"
import "../../../shared/presentation"

WorkbenchDialog {
    id: dialog
    objectName: "profilePlotDialog"
    required property var backend
    property var profileIds: []
    property int initialProfile: -1
    property int selectedProfile: -1
    readonly property var selectedIds:coverage.value ? coverage.value.query.profiles : []
    readonly property var selectedRoles:coverage.value ? coverage.value.query.roles : []
    property int metric:0
    readonly property var viewport:coverage.value ? coverage.value.viewport : ({})
    readonly property var allRoles:coverage.value ? coverage.value.allRoles : []
    readonly property var roleColors:coverage.value ? (theme.dark ? coverage.value.roleDark : coverage.value.roleLight) : ({})
    property bool sourceCurrent:true
    property bool contextCurrent:false
    property PresentationOwner presentationCoverage: PresentationOwner {
        id:coverage;theme:dialog.theme
        initialValue:dialog.candidate({points:[],roles:[],profiles:[]},{profiles:[],roles:[],metric:0,start:0,count:0},[])
    }

    /**
     * Return the existing compiled carrier role identifiers, in original order.
     * @returns Plain role array used only in admitted candidates/temporary queries.
     */
    function roleIds() {
        return ["data","continuous_pilot","plc","exclusion","zero_bit_loaded","reserved","unknown"]
    }

    /**
     * Couple the native viewport, exact selection and derived toggle/display data.
     * @param value Unmodified native numerical profile viewport.
     * @param query Original inspect request, including selected profiles/roles.
     * @param ids Actual parent catalog IDs for this admitted context.
     * @returns Plain candidate before any selector/plot/table conversion.
     */
    function candidate(value, query, ids) {
        return {viewport:value,query:query,profileIds:ids,allRoles:roleIds(),
            roleDark:{data:"#51d6bf",continuous_pilot:"#ffd166",plc:"#7cb7ff",exclusion:"#f4a261",zero_bit_loaded:"#bfdd73",reserved:"#c998ff",unknown:"#ff80bb"},
            roleLight:{data:"#007e75",continuous_pilot:"#996000",plc:"#2563b9",exclusion:"#b64b20",zero_bit_loaded:"#517c13",reserved:"#8544bb",unknown:"#b82c68"},
            traceDark:["#51d6bf","#7cb7ff","#ffd166","#c998ff","#ff80bb"],traceLight:["#007e75","#2563b9","#996000","#8544bb","#b82c68"],
            metrics:["dB","Bit loading"],identity:["Channel  "+(value.channel ?? "—"),"MAC  "+(value.mac || "—")],
            profiles:(value.profiles || []).slice().reverse().map(function(profile){return {source:profile,label:"Profile "+profile.id}}),
            roles:(value.roles || []).map(function(role){return {source:role,help:role.count+" assignments in selected profiles"}}),
            bandwidth:{profiles:value.profiles || [],selectedIds:query.profiles},
            bandwidthText:"Profile BW  "+(query.profiles.length===1 ? (value.profiles || []).filter(function(profile){return query.profiles.indexOf(profile.id)>=0}).map(function(profile){return profile.id+" · "+profile.bandwidth_text}).join("") : query.profiles.length+" profiles")+"  ⌄"}
    }

    /**
     * Reuse the existing finite trace palette without changing profile colors.
     * @param id Original numerical profile ID.
     * @returns Existing dark/light color, or the inherited accent before admission.
     */
    function profileColor(id) {
        if (!coverage.value) {
            return theme.accent
        }
        const palette = theme.dark ? coverage.value.traceDark : coverage.value.traceLight
        return palette[id % palette.length]
    }

    /**
     * Preserve the existing immutable selection toggle in temporary construction.
     * @param values Currently admitted ID/role array.
     * @param value Original ID/role to toggle.
     * @returns Temporary candidate array; it is never assigned before admission.
     */
    function toggle(values, value) {
        const copy = values.slice()
        const index = copy.indexOf(value)
        if (index>=0) {
            copy.splice(index,1)
        } else {
            copy.push(value)
        }
        return copy
    }

    /**
     * Query/admit a full viewport before publishing changed selections or metric.
     * @param start Original viewport start.
     * @param count Original viewport count; zero preserves full-span behavior.
     * @param profiles Optional temporary candidate profile IDs.
     * @param roles Optional temporary candidate role IDs.
     * @returns False preserves old viewport/query and restores the metric selector.
     */
    function refresh(start, count, profiles, roles) {
        if (!backend || !sourceCurrent || (!contextCurrent && profiles===undefined)) {
            return false
        }
        const query = {profiles:profiles===undefined ? selectedIds : profiles,roles:roles===undefined ? selectedRoles : roles,metric:metric,start:start,count:count}
        const value = JSON.parse(backend.inspect(JSON.stringify(query)))
        if (!coverage.replace(candidate(value,query,profileIds.slice()))) {
            if (coverage.value) {
                metric = coverage.value.query.metric
            }
            return false
        }
        contextCurrent = true
        return true
    }

    /**
     * Preserve range during existing profile/role/metric selection requests.
     * @param profiles Optional candidate profile IDs.
     * @param roles Optional candidate role IDs.
     * @returns Refresh result; no persistent array changes before admission.
     */
    function refreshRange(profiles, roles) {
        return contextCurrent && refresh(viewport.start || 0,viewport.count || 0,profiles,roles)
    }
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(1100, parent ? parent.width - 32 : 1100)
    height: Math.min(760, parent ? parent.height - 32 : 760)
    title: "Downstream Modulation Profile"
    modal: true
    standardButtons: Dialog.Close
    onOpened: {
        const previousMetric = metric
        metric = 0
        if (refresh(0,0,[initialProfile],roleIds())) {
            selectedProfile = initialProfile
        } else {
            metric = coverage.value ? coverage.value.query.metric : previousMetric
        }
    }
    onClosed:bandwidth.close()
    onProfileIdsChanged:contextCurrent=false
    onSourceCurrentChanged:if(!sourceCurrent){contextCurrent=false}
    ProfileBandwidthDialog {id:bandwidth;theme:dialog.theme;context:coverage.value ? coverage.value.bandwidth : ({});sourceCurrent:dialog.sourceCurrent && dialog.contextCurrent}
    component ProfileToggle: WorkbenchActionButton {
        theme: dialog.theme
        id: toggle
        property color traceColor: dialog.theme.accent
        property bool showIndicator: true
        compact: true
        implicitWidth: 108
        checkable: true; padding: 8
        contentItem: Row {
            spacing: 6
            Rectangle { visible: toggle.showIndicator; width: 10; height: 3; anchors.verticalCenter: parent.verticalCenter; color: toggle.traceColor }
            Label { text: toggle.text; color: dialog.theme.text; font.pixelSize: 12 }
        }
        background: Rectangle {
            radius: 9; color: toggle.checked ? dialog.theme.accentSoft : dialog.theme.button
            border.color: toggle.checked ? toggle.traceColor : dialog.theme.line
            border.width: toggle.activeFocus ? 2 : 1
            opacity: toggle.enabled ? 1 : 0.4
        }
    }
    contentItem: ColumnLayout {
        spacing: 8
        RowLayout {
            Layout.fillWidth: true
            Repeater {
                model:coverage.value ? coverage.value.identity : []
                Label { required property string modelData; text: modelData; color: dialog.theme.text; padding: 8; background: Rectangle { radius: height/2; color: dialog.theme.button; border.color: dialog.theme.line } }
            }
            WorkbenchActionButton {
                theme: dialog.theme
                objectName: "profileBandwidthChip"
                text:coverage.value ? coverage.value.bandwidthText : ""
                enabled:dialog.sourceCurrent && dialog.contextCurrent
                Layout.maximumWidth: 490
                onClicked:bandwidth.openChecked()
            }
            Item { Layout.fillWidth: true }
            WorkbenchActionButton { theme: dialog.theme; objectName: "profilePlotFullSpan";enabled:dialog.sourceCurrent && dialog.contextCurrent; text: "Full Span"; onClicked: dialog.refresh(0, 0) }
        }
        GridLayout {
            columns: 2; columnSpacing: 10; rowSpacing: 6; Layout.fillWidth: true
            Label { text: "Y Axis"; color: dialog.theme.muted }
            ComboBox { objectName: "modulationMetricSelector"; model:coverage.value ? coverage.value.metrics : [];enabled:dialog.sourceCurrent && dialog.contextCurrent;currentIndex:dialog.metric; Layout.preferredWidth: 160; onActivated: function(index) { dialog.metric = index; dialog.refreshRange() } ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered; ToolTip.text: "dB: ideal Shannon minimum MER, not measured RxMER" }
            Label { text: "Profiles"; color: dialog.theme.muted; Layout.alignment: Qt.AlignTop; topPadding: 8 }
            Flow {
                Layout.fillWidth: true; Layout.preferredHeight: childrenRect.height; spacing: 6
                Repeater {
                    objectName: "profileToggleRepeater"
                    model:coverage.value ? coverage.value.profiles : []
                    ProfileToggle {
                        required property var modelData
                        readonly property var profile:modelData.source
                        enabled:dialog.sourceCurrent && dialog.contextCurrent
                        objectName: "modulationProfileToggle" + profile.id
                        text:modelData.label; checkable: true; checked: dialog.selectedIds.indexOf(profile.id) >= 0
                        traceColor: dialog.profileColor(profile.id)
                        onClicked:if(!dialog.refreshRange(dialog.toggle(dialog.selectedIds,profile.id),dialog.selectedRoles)){checked=Qt.binding(function(){return dialog.selectedIds.indexOf(profile.id)>=0})}
                    }
                }
                ProfileToggle { objectName: "modulationAllProfiles"; text: "All"; checkable: false; showIndicator: false; enabled:dialog.sourceCurrent && dialog.contextCurrent;checked:coverage.value && dialog.selectedIds.length===coverage.value.profileIds.length;onClicked:dialog.refreshRange(coverage.value.profileIds.slice(),dialog.selectedRoles) }
            }
            Label { text: "Carriers"; color: dialog.theme.muted; Layout.alignment: Qt.AlignTop; topPadding: 8 }
            Flow {
                Layout.fillWidth: true; Layout.preferredHeight: childrenRect.height; spacing: 6
                Repeater {
                    objectName: "carrierToggleRepeater"
                    model:coverage.value ? coverage.value.roles : []
                    ProfileToggle {
                        required property var modelData
                        readonly property var role:modelData.source
                        objectName: "modulationCarrierToggle" + role.kind
                        text: role.label; checkable: true; checked: dialog.selectedRoles.indexOf(role.kind) >= 0
                        enabled:dialog.sourceCurrent && dialog.contextCurrent && role.count>0
                        traceColor: dialog.roleColors[role.kind]
                        onClicked:if(!dialog.refreshRange(dialog.selectedIds,dialog.toggle(dialog.selectedRoles,role.kind))){checked=Qt.binding(function(){return dialog.selectedRoles.indexOf(role.kind)>=0})}
                        ToolTip.delay: theme.tooltipDelayMs; ToolTip.visible: hovered; ToolTip.text:modelData.help
                    }
                }
                ProfileToggle { objectName: "modulationAllCarriers"; text: "All"; checkable: false; showIndicator: false; enabled:dialog.sourceCurrent && dialog.contextCurrent;checked:dialog.selectedRoles.length===dialog.allRoles.length;onClicked:dialog.refreshRange(dialog.selectedIds,dialog.allRoles.slice()) }
            }
        }
        ProfileChart {
            Layout.fillWidth: true; Layout.fillHeight: true
            theme: dialog.theme; viewport: dialog.viewport; roleColors: dialog.roleColors
            profileColor: dialog.profileColor
            onViewportRequested: function(start, count) { dialog.refresh(start, count) }
        }
        Label {objectName:"profilePlotRetention";visible:coverage.rejected || !dialog.contextCurrent || !dialog.sourceCurrent;text:coverage.rejected ? coverage.message : "Reopen this view before changing its selection.";color:dialog.theme.statusAttention;Layout.fillWidth:true;wrapMode:Text.WordWrap}
        Label { visible: !!dialog.viewport.error; text: dialog.viewport.error || ""; color: dialog.theme.statusError }
    }
}
