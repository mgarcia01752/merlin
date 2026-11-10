/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

MenuBar {
    id: bar
    required property var theme
    signal globalSettingsRequested()
    signal aboutRequested()
    background: Rectangle { color: bar.theme.panel }
    Menu {
        title: "&File"
        MenuItem {
            objectName: "openWorkbenchConfig"; text: "Open WB configuration…"; enabled: false
            Accessible.description: "JSON workbench loading is pending."
        }
        MenuItem {
            objectName: "saveWorkbenchConfig"; text: "Save WB configuration…"; enabled: false
            Accessible.description: "JSON workbench saving is pending."
        }
    }
    Menu {
        title: "&Settings"
        MenuItem { objectName: "globalSettingsAction"; text: "Global…"; onTriggered: bar.globalSettingsRequested() }
    }
    Menu {
        title: "&About"
        MenuItem { objectName: "aboutAction"; text: "About MERLIN…"; onTriggered: bar.aboutRequested() }
    }
}
