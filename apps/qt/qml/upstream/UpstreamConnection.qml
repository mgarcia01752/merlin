/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import "../shared/components"

// Retain logical roles while reusing the same solid arrows as Downstream.
Connection {
    required property string role
    comparison: false
}
