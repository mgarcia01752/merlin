/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

MenuBar {
    id: bar
    required property var theme
    property bool fileActionsEnabled: true
    signal newRequested()
    signal openRequested()
    signal saveRequested(bool saveAs)
    signal globalSettingsRequested()
    signal aboutRequested()
    signal faqRequested()
    readonly property url terminologyUrl: "https://github.com/mgarcia01752/merlin/blob/master/docs/user/terminology.md"
    signal terminologyRequested(url target)
    background: Rectangle { color: bar.theme.panel }
    Menu {
        title: "&File"
        MenuItem {
            objectName: "newWorkbench"
            action: Action { text: "New"; shortcut: StandardKey.New; enabled: bar.fileActionsEnabled; onTriggered: bar.newRequested() }
        }
        MenuItem {
            objectName: "openWorkbenchConfig"
            action: Action { text: "Open"; shortcut: StandardKey.Open; enabled: bar.fileActionsEnabled; onTriggered: bar.openRequested() }
            Accessible.description: "Open a self-contained MERLIN workbench JSON file."
        }
        MenuItem {
            objectName: "saveWorkbenchConfig"
            action: Action { text: "Save"; shortcut: StandardKey.Save; enabled: bar.fileActionsEnabled; onTriggered: bar.saveRequested(false) }
            Accessible.description: "Save accepted block settings and original inputs in workbench JSON."
        }
        MenuItem { objectName: "saveWorkbenchAs"; action: Action { text: "Save As"; shortcut: StandardKey.SaveAs; enabled: bar.fileActionsEnabled; onTriggered: bar.saveRequested(true) }
            Accessible.description: "Save accepted block settings and original inputs in workbench JSON." }
    }
    Menu {
        title: "&Settings"
        MenuItem { objectName: "globalSettingsAction"; text: "Global"; onTriggered: bar.globalSettingsRequested() }
    }
    Menu {
        title: "&About"
        MenuItem { objectName: "aboutAction"; text: "About MERLIN"; onTriggered: bar.aboutRequested() }
        MenuItem { objectName: "faqAction"; text: "FAQ"; onTriggered: bar.faqRequested(); Accessible.description: "Open the bundled MERLIN FAQ offline." }
        MenuItem {
            objectName: "terminologyAction"
            text: "Terminology"
            onTriggered: bar.terminologyRequested(bar.terminologyUrl)
            Accessible.description: "Open MERLIN terminology on GitHub in the default browser."
        }
    }
}
