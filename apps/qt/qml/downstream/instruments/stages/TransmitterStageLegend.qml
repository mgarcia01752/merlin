/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
Flow {
    id:legend
    required property var theme
    required property var graph
    required property var observation
    spacing:14
    visible:observation.available
    Label {visible:legend.graph.kind === "constellation" && (legend.observation.role_counts || {}).data>0;text:"■ Data";color:legend.theme.qamData}
    Label {visible:legend.graph.kind === "constellation" && (legend.observation.role_counts || {}).ncp>0;text:"● NCP";color:legend.theme.qamNcp}
    Label {visible:legend.graph.kind === "constellation" && (legend.observation.role_counts || {}).unknown>0;text:"■ Unclassified";color:legend.theme.qamUnknown}
    Label {visible:legend.graph.kind === "constellation" && (legend.observation.role_counts || {}).filler>0;text:"◆ Filler";color:legend.theme.qamFiller}
    Label {visible:legend.graph.kind === "constellation" && (legend.observation.role_counts || {}).plc>0;text:"■ PLC";color:legend.theme.accent}
    Label {visible:legend.graph.kind === "sequence";text:"— I";color:legend.theme.accent}
    Label {visible:legend.graph.kind === "sequence";text:"— Q";color:legend.theme.statusAttention}
    Label {visible:legend.graph.kind === "raster";text:"■ 0";color:legend.theme.muted}
    Label {visible:legend.graph.kind === "raster";text:"■ 1";color:legend.theme.accent}
    Label {visible:legend.graph.kind === "physical";text:"■ Data locations";color:legend.theme.qamData}
    Label {visible:legend.graph.kind === "physical";text:"■ PLC";color:legend.theme.txPlc}
    Label {visible:legend.graph.kind === "physical";text:"■ Continuous pilots";color:legend.theme.txContinuousPilot}
    Label {visible:legend.graph.kind === "physical";text:"■ Scattered pilots";color:legend.theme.txScatteredPilot}
    Label {visible:legend.graph.kind === "physical";text:"■ Excluded";color:legend.theme.qamUnknown}
}
