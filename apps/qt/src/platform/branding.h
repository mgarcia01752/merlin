/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
#pragma once
#include <QGuiApplication>
#include <QIcon>

/**
 * Set the theme-specific application icon on the Qt UI thread.
 * Uses only embedded MERLIN resources after application construction.
 */
inline void setWorkbenchIcon(bool dark) {
    if (dark) {
        QGuiApplication::setWindowIcon(QIcon(QStringLiteral(":/branding/merlin-icon-dark.png")));
    } else {
        QGuiApplication::setWindowIcon(QIcon(QStringLiteral(":/branding/merlin-icon-light.png")));
    }
}
