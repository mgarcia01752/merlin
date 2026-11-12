/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

QtObject {
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
