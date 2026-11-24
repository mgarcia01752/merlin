/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Explicit standalone-QML fixture transport, not application accounting evidence.
// Rust View/counter tests and the native executable own actual-budget acceptance.
QtObject {
    id: budget
    property int maximumBytes: 24
    property int maximumEntries: 12
    property int maximumViews: 3
    property int maximumOpen: 1
    property int bytes: 0
    property int entries: 0
    property int views: 0
    property int open: 0
    property bool failCommit: false
    property Component adapterComponent: Component {
        QtObject {
            property int currentBytes: 0
            property int currentEntries: 0
            property int candidateBytes: 0
            property int candidateEntries: 0
            property bool hasCurrent: false
            property bool pending: false
            property bool retained: false
            property bool opened: false
            function limits() {
                // Test traversal policy; independent of production default ceilings.
                return JSON.stringify({max_utf16_bytes: budget.maximumBytes,
                    max_entries: budget.maximumEntries, max_depth: 16})
            }
            function ensureView() {
                if (retained) {
                    return true
                }
                if (budget.views >= budget.maximumViews) {
                    return false
                }
                budget.views++
                retained = true
                return true
            }
            function prepare(nextBytes, nextEntries) {
                if (pending || budget.bytes + nextBytes > budget.maximumBytes
                        || budget.entries + nextEntries > budget.maximumEntries) {
                    return false
                }
                if (!ensureView()) {
                    return false
                }
                budget.bytes += nextBytes
                budget.entries += nextEntries
                candidateBytes = nextBytes
                candidateEntries = nextEntries
                pending = true
                return true
            }
            function commit() {
                if (!pending || budget.failCommit) {
                    return false
                }
                budget.bytes -= currentBytes
                budget.entries -= currentEntries
                currentBytes = candidateBytes
                currentEntries = candidateEntries
                candidateBytes = 0
                candidateEntries = 0
                pending = false
                hasCurrent = true
                return true
            }
            function releaseEmpty() {
                if (retained && !hasCurrent && !pending && !opened) {
                    budget.views--
                    retained = false
                }
            }
            function cancel() {
                budget.bytes -= candidateBytes
                budget.entries -= candidateEntries
                candidateBytes = 0
                candidateEntries = 0
                pending = false
                releaseEmpty()
            }
            function begin_open() {
                if (opened) {
                    return true
                }
                if (budget.open >= budget.maximumOpen || !ensureView()) {
                    return false
                }
                budget.open++
                opened = true
                return true
            }
            function end_open() {
                if (opened) {
                    budget.open--
                    opened = false
                }
                releaseEmpty()
            }
            function clear() {
                cancel()
                budget.bytes -= currentBytes
                budget.entries -= currentEntries
                currentBytes = 0
                currentEntries = 0
                hasCurrent = false
                releaseEmpty()
            }
            Component.onDestruction: {
                clear()
                end_open()
                if (retained) {
                    budget.views--
                }
            }
        }
    }
    /**
     * Create a parent-owned fixture adapter for standalone QML behavior tests.
     * @param parent Actual test owner; its destruction releases fixture quantities.
     * @returns A bounded fixture using this test's explicitly supplied policies.
     */
    function createAdapter(parent) {
        return adapterComponent.createObject(parent)
    }
}
