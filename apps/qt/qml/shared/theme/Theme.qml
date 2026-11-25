/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

QtObject {
    // Shared hover/focus tooltip timing, in milliseconds, for every MERLIN view.
    readonly property int tooltipDelayMs: 1000
    // Shared native-session presentation polling cadence; independent of hover timing.
    readonly property int sessionPollIntervalMs: 100

    // Shared canvas framing. Equal side/top values are independent layout policies.
    readonly property int workbenchCanvasSideMargin: 24
    readonly property int workbenchCanvasTopMargin: 24
    readonly property int workbenchCanvasBottomMargin: 20
    // Shared diagram coordinates and pixel styling preserve the downstream layout.
    readonly property int workbenchDiagramWidth: 1140
    readonly property int workbenchDiagramHeight: 580
    readonly property int workbenchBlockWidth: 195
    readonly property int workbenchBlockHeight: 90
    readonly property int workbenchInputRowTop: 65
    readonly property int workbenchMainRowTop: 245
    readonly property int workbenchLowerRowTop: 425
    readonly property int workbenchFirstColumnLeft: 325
    readonly property int workbenchColumnGap: 110
    readonly property int workbenchConfigurationLeft: 10
    readonly property int workbenchConfigurationWidth: 270
    readonly property int workbenchConfigurationRadius: 20
    readonly property real workbenchConfigurationBorderWidth: 1.5
    readonly property int workbenchConfigurationHeadingSize: 17
    readonly property color workbenchConfigurationFill: dark ? "#0b2332" : accentSoft
    // Junction inset is separate from the canvas bottom margin despite equal values.
    readonly property int workbenchConfigurationJunctionInset: 20
    readonly property int workbenchGridOrigin: 16
    readonly property int workbenchGridSpacing: 24
    readonly property int workbenchGridDotSize: 1
    // Injected by the native application; tests supply an explicit fixture.
    property var presentationBackendFactory: null
    property bool presentationAdmissionRejected: false
    property bool dark: true
    readonly property color background: dark ? "#0c1422" : "#f3f6f9"
    readonly property color panel: dark ? "#111e30" : "#ffffff"
    readonly property color text: dark ? "#e8f0f7" : "#142b3e"
    readonly property color muted: dark ? "#93a8be" : "#566c7e"
    readonly property color line: dark ? "#24354b" : "#d8e2ea"
    readonly property color accent: dark ? "#51d6bf" : "#007e75"
    readonly property color accentSoft: dark ? "#163a3d" : "#e0f4ef"
    readonly property color statusComplete: dark ? "#45cf83" : "#187344"
    readonly property color statusAttention: dark ? "#f3cb56" : "#936400"
    readonly property color statusError: dark ? "#f16c75" : "#bb2636"
    readonly property color qamData: dark ? "#45cf83" : "#187344"
    readonly property color qamNcp: dark ? "#ff7078" : "#b52635"
    readonly property color qamFiller: dark ? "#f3cb56" : "#936400"
    readonly property color qamUnknown: dark ? "#b5c5d7" : "#566c7e"
    readonly property color txPlc: dark ? "#6badff" : "#255eb7"
    readonly property color txContinuousPilot: dark ? "#f3cb56" : "#936400"
    readonly property color txScatteredPilot: dark ? "#d29aff" : "#8248aa"
    readonly property color button: dark ? "#19293e" : "#f6f9fb"
}
