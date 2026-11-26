/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import "LogicalFootprint.js" as Footprint

// One concrete persistent DTO owner. The application injects its native factory;
// standalone presentation tests explicitly inject a fixture factory instead.
QtObject {
    id: owner
    required property var theme
    property var initialValue: null
    readonly property var value: state.current
    readonly property bool rejected: state.rejected || (state.textRequired && !state.textAdmitted)
    readonly property bool textAdmitted: state.textAdmitted
    readonly property string message: rejected ? "Presentation limits reached; previous view preserved." : ""
    property QtObject state: QtObject {
        property var current: null
        property var backend: null
        property bool rejected: false
        property bool replacing: false
        property bool textRequired: false
        property bool textAdmitted: false
        property double textBytes: 0
        property var textBackend: null
    }

    /**
     * Obtain the injected, parent-owned native adapter on first use.
     * @returns Null when unavailable; caller admission then fails closed.
     */
    function backend() {
        if (!state.backend && theme && typeof theme.presentationBackendFactory === "function") {
            state.backend = theme.presentationBackendFactory(owner)
        }
        return state.backend
    }

    /**
     * Atomically admit a plain candidate before changing persistent presentation.
     * @param candidate JSON-shaped value measured without mutation or coercion.
     * @returns True after assignment/commit; false preserves the prior value.
     * Old/candidate coverage overlaps until assignment. Aliases share this witness;
     * independently retained derived copies require their own owner.
     */
    function replace(candidate) {
        if (state.replacing) {
            return false
        }
        state.replacing = true
        try {
            const adapter = backend()
            if (!adapter) {
                state.rejected = true
                return false
            }
            const counts = Footprint.measure(candidate, JSON.parse(adapter.limits()))
            if (!counts.ok || !adapter.prepare(counts.utf16_bytes, counts.entries)) {
                state.rejected = true
                return false
            }
            // The accepted value changes only after admission. Child plot bindings
            // refer to this same owner; derived models keep separate coverage.
            const previous = state.current
            state.current = candidate
            if (!adapter.commit()) {
                // This is a protocol failure, not permission to publish unguarded data.
                state.current = previous
                adapter.cancel()
                state.rejected = true
                return false
            }
            state.rejected = false
            return true
        } finally {
            state.replacing = false
        }
    }

    /**
     * Clear a value only when the caller has cleared its independent aliases.
     * Current bindings observe null before native data coverage is released.
     * @returns False during synchronous replacement/clear callbacks; no mutation.
     * Other calls return true after clearing. Nested transactions cannot dispose
     * coverage still needed by an outer assignment.
     */
    function clear() {
        if (state.replacing) {
            return false
        }
        state.replacing = true
        try {
            state.current = null
            if (state.backend) {
                state.backend.clear()
            }
            state.rejected = false
            return true
        } finally {
            state.replacing = false
        }
    }

    /**
     * Reserve visibility before the caller opens its dialog.
     * @returns False leaves the view closed and all retained data unchanged.
     */
    function beginOpen() {
        const adapter = backend()
        const accepted = adapter && adapter.begin_open()
        state.rejected = !accepted
        return !!accepted
    }

    /**
     * End visibility coverage after close; hidden data stays admitted until clear.
     */
    function endOpen() {
        if (state.backend) {
            state.backend.end_open()
        }
    }

    /**
     * Pre-admit finite editable-text coverage using each actual maximumLength.
     * @param fields Existing Qt TextFields; no reference to them is retained.
     * @returns True when their conservative text complement is admitted.
     * Data clear/hide preserves this separate coverage until owner disposal.
     * The caller gates dynamic text and editing on textAdmitted.
     */
    function reserveTextFields(fields) {
        if (state.replacing) {
            return false
        }
        state.replacing = true
        try {
            state.textRequired = true
            if (!state.textBackend && theme && typeof theme.presentationBackendFactory === "function") {
                state.textBackend = theme.presentationBackendFactory(owner)
            }
            const adapter = state.textBackend
            if (!adapter || !Array.isArray(fields)) {
                return false
            }
            const lengths = []
            for (const field of fields) {
                if (!field) {
                    return false
                }
                lengths.push(field.maximumLength)
            }
            const counts = Footprint.textCapacity(lengths, JSON.parse(adapter.limits()).max_utf16_bytes)
            if (!counts.ok) {
                return false
            }
            if (state.textAdmitted && counts.utf16_bytes === state.textBytes) {
                return true
            }
            if (!adapter.prepare(counts.utf16_bytes, 0)) {
                return false
            }
            if (!adapter.commit()) {
                adapter.cancel()
                return false
            }
            state.textBytes = counts.utf16_bytes
            state.textAdmitted = true
            return true
        } finally {
            state.replacing = false
        }
    }

    Component.onCompleted: {
        if (initialValue !== null) {
            replace(initialValue)
        }
        // The initial configuration must not become an unguarded old alias.
        initialValue = null
    }
    // The native QObject is parented to this owner and owns all its reservations.
    // QObject destruction releases them after this value becomes unreachable.
}
