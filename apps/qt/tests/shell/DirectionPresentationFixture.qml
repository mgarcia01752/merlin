/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../../qml/shared/components"
import "../../qml/shared/theme"

// Presentation state and intent only. This fixture executes no scientific backend.
FocusScope {
    id: fixture
    required property var theme
    required property string direction
    required property var tabHost
    readonly property int fixtureMargin: 8
    property string selectedPanel: "Initial fixture panel"
    property real viewportFraction: 0.25
    property bool extraFocusEnabled: true
    // Captured with the displayed fixture state; host adoption must refresh it.
    property var sourceIdentity: undefined
    Component.onCompleted: sourceIdentity = tabHost.identityFor(direction)
    property alias firstField: firstField
    readonly property var secondField: secondFieldLoader.item
    property alias surfaceColor: surface.color
    objectName: direction + "PresentationFixture"

    Rectangle { id: surface; anchors.fill: parent; color: fixture.theme.panel }
    ColumnLayout {
        anchors.fill: parent
        anchors.margins: fixture.fixtureMargin
        spacing: fixture.fixtureMargin
        Label {
            Layout.fillWidth: true
            text: "Presentation fixture · " + fixture.direction + " · no backend execution"
            color: fixture.theme.text
            wrapMode: Text.WordWrap
        }
        TextField {
            id: firstField
            objectName: fixture.direction + "FirstField"
            Layout.fillWidth: true
            text: fixture.direction + " retained text"
            focus: true
            ThemePalette { targetControl: firstField; theme: fixture.theme }
        }
        Loader {
            id: secondFieldLoader
            Layout.fillWidth: true
            active: fixture.extraFocusEnabled
            sourceComponent: Component {
                TextField {
                    id: secondField
                    objectName: fixture.direction + "SecondField"
                    text: "Secondary fixture field"
                    ThemePalette { targetControl: secondField; theme: fixture.theme }
                }
            }
        }
        RowLayout {
            Layout.fillWidth: true
            spacing: fixture.fixtureMargin
            WorkbenchActionButton {
                objectName: fixture.direction + "StartIntent"
                Layout.fillWidth: true
                theme: fixture.theme
                text: "Start"
                onClicked: fixture.tabHost.requestRunAction("start", fixture.direction, fixture.sourceIdentity)
            }
            WorkbenchActionButton {
                objectName: fixture.direction + "PauseIntent"
                Layout.fillWidth: true
                theme: fixture.theme
                text: "Pause"
                onClicked: fixture.tabHost.requestRunAction("pause", fixture.direction, fixture.sourceIdentity)
            }
            WorkbenchActionButton {
                objectName: fixture.direction + "SaveIntent"
                Layout.fillWidth: true
                theme: fixture.theme
                text: "Save"
                onClicked: fixture.tabHost.requestSave(false, fixture.direction, fixture.sourceIdentity)
            }
        }
        Item { Layout.fillWidth: true; Layout.fillHeight: true }
    }
    Shortcut {
        sequence: "Ctrl+Alt+P"
        enabled: fixture.visible && fixture.enabled && fixture.tabHost.scopedActionsAvailable
            && fixture.tabHost.selectedDirection === fixture.direction
        onActivated: fixture.tabHost.requestRunAction("pause", fixture.direction, fixture.sourceIdentity)
    }
}
