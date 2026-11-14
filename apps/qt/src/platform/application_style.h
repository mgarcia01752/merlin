/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
#pragma once
#include <QtQuickControls2/QQuickStyle>
#include <QString>

/**
 * Select palette-aware application controls before loading any Qt Quick QML.
 * @return Qt's configured style name for startup smoke verification.
 */
inline QString configureQuickControlsStyle() {
    QQuickStyle::setStyle(QStringLiteral("Fusion"));
    return QQuickStyle::name();
}
