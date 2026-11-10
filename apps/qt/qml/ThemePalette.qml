/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Bind roles on the actual control palette so theme changes remain live after
// Qt copies/resolves palettes for popups, inactive windows and disabled controls.
QtObject {
    id: styling
    required property var targetControl
    required property Theme theme
    property list<QtObject> bindings: [
        Binding { target: styling.targetControl; property: "palette.window"; value: styling.theme.background },
        Binding { target: styling.targetControl; property: "palette.windowText"; value: styling.theme.text },
        Binding { target: styling.targetControl; property: "palette.base"; value: styling.theme.panel },
        Binding { target: styling.targetControl; property: "palette.alternateBase"; value: styling.theme.button },
        Binding { target: styling.targetControl; property: "palette.text"; value: styling.theme.text },
        Binding { target: styling.targetControl; property: "palette.button"; value: styling.theme.button },
        Binding { target: styling.targetControl; property: "palette.buttonText"; value: styling.theme.text },
        Binding { target: styling.targetControl; property: "palette.highlight"; value: styling.theme.accent },
        Binding { target: styling.targetControl; property: "palette.highlightedText"; value: styling.theme.background },
        Binding { target: styling.targetControl; property: "palette.placeholderText"; value: styling.theme.muted },
        Binding { target: styling.targetControl; property: "palette.toolTipBase"; value: styling.theme.panel },
        Binding { target: styling.targetControl; property: "palette.toolTipText"; value: styling.theme.text },
        Binding { target: styling.targetControl; property: "palette.link"; value: styling.theme.accent },
        Binding { target: styling.targetControl; property: "palette.linkVisited"; value: styling.theme.accent },
        Binding { target: styling.targetControl; property: "palette.light"; value: styling.theme.button },
        Binding { target: styling.targetControl; property: "palette.midlight"; value: styling.theme.button },
        Binding { target: styling.targetControl; property: "palette.mid"; value: styling.theme.line },
        Binding { target: styling.targetControl; property: "palette.dark"; value: styling.theme.line },
        Binding { target: styling.targetControl; property: "palette.shadow"; value: styling.theme.line },
        Binding { target: styling.targetControl; property: "palette.disabled.text"; value: styling.theme.muted },
        Binding { target: styling.targetControl; property: "palette.disabled.windowText"; value: styling.theme.muted },
        Binding { target: styling.targetControl; property: "palette.disabled.buttonText"; value: styling.theme.muted },
        Binding { target: styling.targetControl; property: "palette.disabled.placeholderText"; value: styling.theme.muted }
    ]
}
