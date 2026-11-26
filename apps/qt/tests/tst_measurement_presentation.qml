/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml/shared/theme"
import "../qml/downstream/measurements"
import "fixtures"

Item {
    id: window
    width: 1120
    height: 720
    PresentationBudget {
        id: budget
        // Independent room for these concrete presentation fixture graphs.
        maximumBytes: 100000
        maximumEntries: 10000
        maximumViews: 16
        maximumOpen: 2
    }
    Theme { id: fixtureTheme; presentationBackendFactory: budget.createAdapter }
    QtObject {
        id: settings
        function current() { return "{}" }
    }
    QtObject {
        id: fixtureBackend
        function view(rxmer, query, configuration) {
            const selected = JSON.parse(query)
            return JSON.stringify({points: [{x: 0, y: 1}], x_ticks: [], y_ticks: [],
                summary: "Original observation", selected: selected.index})
        }
        function preview_frame(options, position, start, count) {
            return JSON.stringify({points: [{x: 0, y: 2}], x_ticks: [], y_ticks: [],
                frame_index: position, source_index: 0, summary: "Capture preview"})
        }
    }
    Component {
        id: viewerComponent
        MeasurementViewer {
            theme: fixtureTheme
            backend: fixtureBackend
            settingsBackend: settings
            rxmer: true
            options: ({mode: "single", interval_symbols: "1000", tolerance_seconds: 90000})
            selectionCount: 1
            previewEnabled: true
            width: window.width
            height: window.height
        }
    }
    Component {
        id: treeComponent
        MeasurementCaptureTree {
            theme: fixtureTheme
            width: window.width
            height: window.height
            selectionEnabled: true
        }
    }
    TestCase {
        id: cases
        name: "MeasurementPresentation"
        when: windowShown
        function capture(index, name) {
            return {index: index, name: name, format: "Fixture JSON", bytes: 42,
                capture_time: 123, mac: "00:00:00:00:00:01", channel: 34,
                frequency: "Fixture frequency", sha256: "capture-" + index,
                compatible: true, reason: "Fixture compatibility"}
        }
        function init() {
            compare(budget.bytes, 0)
            compare(budget.entries, 0)
            compare(budget.views, 0)
            budget.maximumBytes = 100000
            budget.maximumEntries = 10000
        }
        function test_viewport_rejection_keeps_observation_and_accepted_context() {
            const viewer = createTemporaryObject(viewerComponent, window)
            verify(viewer !== null)
            viewer.files = [capture(0, "Original")]
            verify(viewer.refresh(0, 0))
            const original = viewer.viewport
            const context = viewer.displayedContext
            const limit = budget.maximumBytes
            budget.maximumBytes = budget.bytes
            viewer.selectedIndex = 0
            viewer.metric = 1
            verify(!viewer.refresh(0, 0))
            compare(viewer.viewport, original)
            compare(viewer.displayedContext, context)
            verify(viewer.contextCurrent)
            compare(viewer.selectedIndex,context.selectedIndex)
            compare(viewer.metric,context.metric)
            compare(viewer.symbol,context.symbol)
            verify(!viewer.showFrame(0))
            verify(!viewer.animationActive)
            budget.maximumBytes = limit
            verify(viewer.showFrame(0))
            verify(viewer.animationActive)
            viewer.stopPreview()
            verify(!viewer.previewRequested)
            verify(viewer.animationActive) // Hidden accepted data/context still exists.
            verify(viewer.reset())
            verify(!viewer.animationActive)
        }
        function test_new_files_rejection_keeps_old_rows_and_disables_stale_actions() {
            const tree = createTemporaryObject(treeComponent, window)
            verify(tree !== null)
            tree.files = [capture(0, "Original")]
            tryCompare(tree, "contextCurrent", true)
            const nodes = findChild(tree, "measurementCaptureTree")
            verify(nodes !== null)
            tryCompare(nodes, "count", 1)
            tree.toggleExpanded(0)
            tryCompare(nodes, "count", 7) // File plus its six existing detail rows.
            verify(tree.expanded[0])
            const original = tree.displayedContext
            const limit = budget.maximumBytes
            budget.maximumBytes = budget.bytes
            tree.files = [capture(1, "Replacement")]
            verify(!tree.refreshPresentation())
            compare(tree.displayedContext, original)
            verify(tree.expanded[0])
            verify(!tree.contextCurrent)
            compare(nodes.count, 7)
            const stale = findChild(tree, "captureCheck0")
            verify(stale !== null)
            compare(stale.enabled, false)
            budget.maximumBytes = limit
            verify(tree.refreshPresentation())
            tryCompare(tree, "contextCurrent", true)
            tryCompare(nodes, "count", 1)
            verify(!tree.expanded[0])
            compare(tree.displayedContext.files[0].name, "Replacement")
        }
        function test_qvariant_creation_input_rejects_then_plain_binding_recovers() {
            // Qt createObject initial maps convert nested values to QVariant
            // wrappers. Production files arrive as ordinary JSON/var bindings.
            const tree = createTemporaryObject(treeComponent, window,
                {files: [capture(0, "Variant creation input")]})
            verify(tree !== null)
            verify(!tree.refreshPresentation())
            verify(!tree.contextCurrent)
            const nodes = findChild(tree, "measurementCaptureTree")
            verify(nodes !== null)
            compare(nodes.count, 0)
            tree.files = [capture(0, "Plain binding")]
            verify(tree.refreshPresentation())
            verify(tree.contextCurrent)
            tryCompare(nodes, "count", 1)
        }
        function test_hidden_view_keeps_data_and_deferred_destruction_releases_it() {
            const viewer = createTemporaryObject(viewerComponent, window)
            verify(viewer !== null)
            viewer.files = [capture(0, "Original")]
            verify(viewer.refresh(0, 0))
            const nodes = findChild(viewer, "measurementCaptureTree")
            verify(nodes !== null)
            tryCompare(nodes.parent, "contextCurrent", true)
            viewer.visible = false
            const hiddenBytes = budget.bytes
            verify(hiddenBytes > 0)
            wait(0)
            compare(budget.bytes, hiddenBytes)
            viewer.destroy()
            tryCompare(budget, "bytes", 0)
            tryCompare(budget, "entries", 0)
            tryCompare(budget, "views", 0)
        }
    }
}
