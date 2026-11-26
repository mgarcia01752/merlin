/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../fixtures"
import "../../qml/shared/theme"
import "../../qml/upstream"
import "fixtures/ResourceGridFixture.js" as ResourceFixture

Pane {
    id: window
    readonly property int fixturePresentationBytes: 4 * 1024 * 1024
    readonly property int fixturePresentationEntries: 100000
    readonly property int fixtureRetainedViews: 64
    readonly property int fixtureOpenViews: 4
    PresentationBudget {
        id: presentationBudget
        maximumBytes: window.fixturePresentationBytes
        maximumEntries: window.fixturePresentationEntries
        maximumViews: window.fixtureRetainedViews
        maximumOpen: window.fixtureOpenViews
    }
    width: tests.desktopWidth
    height: tests.desktopHeight
    visible: true
    padding: 0
    Theme {
        id: theme
        presentationBackendFactory: presentationBudget.createAdapter
    }
    ThemePalette {
        targetControl: window
        theme: theme
    }
    ThemePalette {
        targetControl: window.Window.window
        theme: theme
    }
    ThemePalette {
        targetControl: dialog
        theme: theme
    }
    QtObject {
        id: host
        property var resourceGridState: ({})
    }
    ResourceGridView {
        id: view
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        theme: theme
        host: host
    }
    ResourceGridDialog {
        id: dialog
        theme: theme
        host: host
    }
    SignalSpy {
        id: querySpy
        target: view
        signalName: "viewRequested"
    }
    SignalSpy {
        id: grantSpy
        target: view
        signalName: "grantRequested"
    }
    SignalSpy {
        id: inspectionSpy
        target: view
        signalName: "cellInspected"
    }
    TestCase {
        id: tests
        name: "UpstreamResourceGridPresentation"
        when: windowShown
        readonly property real desktopWidth: 1200
        readonly property real desktopHeight: 960
        readonly property real narrowWidth: 640
        readonly property real narrowHeight: 480
        // Rust MAX_RESOURCE_GRID_CELLS from resources/view.rs; not a PHY limit.
        readonly property int acceptedMaximumCells: 16384
        readonly property int renderSettleDelayMs: 100
        readonly property int constantVisualItemBudget: 160
        readonly property int wheelZoomDelta: 120
        property var originals: []

        /* Load unchanged public DTO output from the documented Rust-process fixture. */
        function initTestCase() {
            originals = ResourceFixture.snapshots;
            compare(originals.length, 2);
            compare(originals[0].grid.cells.length, 204);
            compare(originals[1].grid.cells.length, 396);
        }

        /* Wrap a real accepted snapshot with application-only session/query ownership. */
        function stateFor(index) {
            return {
                status: "ready",
                reason: "",
                snapshot: originals[index],
                max_cells: acceptedMaximumCells,
                configuration_generation: String(originals[index].configuration_generation),
                session_token: index === 0 ? "configured-50k" : "configured-25k",
                snapshot_session_token: index === 0 ? "configured-50k" : "configured-25k",
                grants: [
                    {
                        grant_id: originals[0].grid.grant_id,
                        session_token: "configured-50k",
                        label: "50 kHz configured grant 17"
                    },
                    {
                        grant_id: originals[1].grid.grant_id,
                        session_token: "configured-25k",
                        label: "25 kHz configured grant 17"
                    }
                ]
            };
        }

        /* Reset application presentation state without inventing a queried scientific view. */
        function restoreOriginal(index) {
            host.resourceGridState = {
                status: "unavailable",
                reason: "No view selected"
            };
            host.resourceGridState = stateFor(index);
            tryCompare(view, "displayable", true);
            tryCompare(findChild(view, "resourceGridCanvas"), "paintedCellCount", originals[index].grid.cells.length);
        }

        function init() {
            presentationBudget.maximumBytes = window.fixturePresentationBytes;
            presentationBudget.maximumEntries = window.fixturePresentationEntries;
            presentationBudget.maximumOpen = window.fixtureOpenViews;
            dialog.close();
            window.width = desktopWidth;
            window.height = desktopHeight;
            theme.dark = true;
            restoreOriginal(0);
            querySpy.clear();
            grantSpy.clear();
            inspectionSpy.clear();
        }

        /* Count actual visual objects; cell-count growth must not create QML cell delegates. */
        function visualItems(item) {
            let count = 1;
            for (const child of item.children) {
                count += visualItems(child);
            }
            return count;
        }

        function test_original_coordinates_roles_and_metadata_data() {
            return [
                {
                    tag: "50k-dark",
                    index: 0,
                    dark: true
                },
                {
                    tag: "50k-light",
                    index: 0,
                    dark: false
                },
                {
                    tag: "25k-dark",
                    index: 1,
                    dark: true
                },
                {
                    tag: "25k-light",
                    index: 1,
                    dark: false
                }
            ];
        }

        function test_original_coordinates_roles_and_metadata(data) {
            restoreOriginal(data.index);
            theme.dark = data.dark;
            wait(renderSettleDelayMs);
            const canvas = findChild(view, "resourceGridCanvas");
            const image = grabImage(canvas);
            const snapshot = originals[data.index];
            const grid = snapshot.grid;
            for (const role of ["data", "complementary", "known_pilot", "zero_loaded", "excluded", "unallocated"]) {
                const cell = grid.cells.find(cell => cell.role === role);
                const rectangle = view.cellRectangle(cell);
                compare(image.pixel(Math.round(rectangle.x + rectangle.width / 2), Math.round(rectangle.y + rectangle.height / 2)), view.roleColor(role));
                compare(view.cellAt(rectangle.x + rectangle.width / 2, rectangle.y + rectangle.height / 2), cell);
            }
            const exclusion = view.cellRectangle(grid.cells[0]);
            const pilot = view.cellRectangle(grid.cells[1]);
            compare(grid.cells[0].physical_carrier, data.index === 0 ? 99 : 199);
            fuzzyCompare(exclusion.y - pilot.y, canvas.cellHeight, 0.001);
            compare(grid.cells[0].frame_symbol, 0);
            compare(grid.cells[0].grant_symbol, 6);
            compare(grid.cells[0].prefix_start_sample, data.index === 0 ? 16960 : 29248);
            const metadata = findChild(view, "resourceGridMetadata").text;
            verify(metadata.indexOf("Full-frame coded bits " + grid.frame_bits) >= 0);
            verify(metadata.indexOf("Full-grant coded bits " + grid.grant_bits) >= 0);
            verify(metadata.indexOf("Absolute frame 12") >= 0);
            verify(metadata.indexOf("Transmission planned " + (data.index === 0 ? "false" : "true")) >= 0);
            verify(findChild(view, "resourceGridStatus").text.indexOf("before Pre-Eq C") >= 0);
        }

        function test_click_inspects_unchanged_cell_and_context() {
            const canvas = findChild(view, "resourceGridCanvas");
            const cell = originals[0].grid.cells[2];
            const rectangle = view.cellRectangle(cell);
            mouseClick(findChild(view, "resourceGridNavigation"), rectangle.x + rectangle.width / 2, rectangle.y + rectangle.height / 2);
            compare(inspectionSpy.count, 1);
            compare(inspectionSpy.signalArguments[0][0], cell);
            compare(inspectionSpy.signalArguments[0][1].session_token, "configured-50k");
            compare(inspectionSpy.signalArguments[0][1].configuration_generation, "1");
            compare(view.inspectedCell.visit_index, 72);
            compare(view.inspectedCell.frame_bit_offset, 247);
            compare(view.inspectedCell.grant_bit_offset, 575);
            verify(findChild(view, "resourceCellInspection").text.indexOf('"physical_carrier": 101') >= 0);
            view.inspectAt(0, 0);
            compare(inspectionSpy.count, 1);
            verify(canvas.paintedCellCount > 0);
            const stale = stateFor(0);
            stale.configuration_generation = "2";
            host.resourceGridState = stale;
            view.inspectAt(rectangle.x + rectangle.width / 2, rectangle.y + rectangle.height / 2);
            compare(inspectionSpy.count, 1);
            compare(view.inspectedCell, null);
        }

        function test_wheel_zoom_requests_nonzero_original_window_and_freezes() {
            const pointer = findChild(view, "resourceGridNavigation");
            const canvas = findChild(view, "resourceGridCanvas");
            mouseWheel(pointer, view.leftAxisMargin + canvas.plotWidth / 2, view.topAxisMargin + canvas.plotHeight / 2, 0, wheelZoomDelta);
            compare(querySpy.count, 1);
            const request = querySpy.signalArguments[0][0];
            compare(Object.keys(request).sort().join(","), "carrier_count,first_carrier,first_symbol,frame_index,max_cells,symbol_count");
            compare(request.first_symbol, 2);
            compare(request.symbol_count, 3);
            compare(request.first_carrier, 108);
            compare(request.carrier_count, 17);
            compare(request.max_cells, acceptedMaximumCells);
            compare(querySpy.signalArguments[0][1].session_token, "configured-50k");
            tryCompare(canvas, "paintedCellCount", 0);
            compare(view.displayable, false);
            compare(findChild(view, "resourceZoomIn").enabled, false);
            view.inspectAt(view.leftAxisMargin + 1, view.topAxisMargin + 1);
            compare(inspectionSpy.count, 0);
            // A real but differently bounded old snapshot cannot acknowledge this query.
            host.resourceGridState = stateFor(0);
            compare(view.displayable, false);
            compare(originals[0].grid.first_symbol, 0);
            compare(originals[0].grid.cells.length, 204);
        }

        function test_matching_owned_original_view_acknowledges_boundary_pan() {
            const pointer = findChild(view, "resourceGridNavigation");
            pointer.forceActiveFocus(Qt.TabFocusReason);
            keyClick(Qt.Key_Left);
            compare(querySpy.count, 1);
            compare(querySpy.signalArguments[0][0].first_symbol, 0);
            compare(querySpy.signalArguments[0][0].first_carrier, 99);
            compare(view.displayable, false);
            host.resourceGridState = stateFor(0);
            tryCompare(view, "displayable", true);
            compare(view.pendingWindow, null);
            compare(view.pendingWindowContext, null);
        }

        function test_pointer_pan_keyboard_full_span_and_frame_requests() {
            const pointer = findChild(view, "resourceGridNavigation");
            const canvas = findChild(view, "resourceGridCanvas");
            mouseDrag(pointer, view.leftAxisMargin + canvas.plotWidth / 2, view.topAxisMargin + canvas.plotHeight / 2, 0, canvas.cellHeight * 2);
            compare(querySpy.count, 1);
            compare(querySpy.signalArguments[0][0].first_carrier, 101);
            restoreOriginal(0);
            pointer.forceActiveFocus(Qt.TabFocusReason);
            keyClick(Qt.Key_Up);
            compare(querySpy.count, 2);
            compare(querySpy.signalArguments[1][0].first_carrier, 133);
            restoreOriginal(1);
            mouseClick(findChild(view, "resourceFullCarriers"));
            const full = querySpy.signalArguments[2][0];
            compare(full.first_carrier, 0);
            compare(full.carrier_count, 4096);
            compare(full.symbol_count, 4);
            verify(full.symbol_count * full.carrier_count <= full.max_cells);
            compare(originals[1].grid.frame_bits, 712);
            restoreOriginal(0);
            const frame = findChild(view, "resourceFrameChoice");
            compare(frame.from, 0);
            compare(frame.to, 1);
            mouseClick(frame.down.indicator);
            compare(querySpy.signalArguments[3][0].frame_index, 0);
        }

        function test_real_grant_session_selection_rejects_late_snapshot() {
            const combo = findChild(view, "resourceGrantChoice");
            mouseClick(combo);
            keyClick(Qt.Key_End);
            keyClick(Qt.Key_Return);
            compare(grantSpy.count, 1);
            compare(grantSpy.signalArguments[0][0], 17);
            compare(grantSpy.signalArguments[0][1].session_token, "configured-50k");
            compare(grantSpy.signalArguments[0][1].target_session_token, "configured-25k");
            compare(view.displayable, false);
            compare(combo.enabled, false);
            host.resourceGridState = stateFor(0);
            compare(view.displayable, false);
            host.resourceGridState = stateFor(1);
            tryCompare(view, "displayable", true);
            compare(view.grid.frame_bits, 712);
            const late = stateFor(1);
            late.snapshot = originals[0];
            late.snapshot_session_token = "configured-50k";
            host.resourceGridState = late;
            compare(view.displayable, false);
            compare(view.inspectedCell, null);
            compare(view.cellAt(view.leftAxisMargin + 1, view.topAxisMargin + 1), null);
        }

        function test_unavailable_stale_and_over_budget_have_no_cells_data() {
            return [
                {
                    tag: "unavailable",
                    kind: "unavailable"
                },
                {
                    tag: "loading",
                    kind: "loading"
                },
                {
                    tag: "error",
                    kind: "error"
                },
                {
                    tag: "stale-generation",
                    kind: "stale"
                },
                {
                    tag: "missing-snapshot",
                    kind: "missing"
                },
                {
                    tag: "query-budget",
                    kind: "budget"
                }
            ];
        }

        function test_unavailable_stale_and_over_budget_have_no_cells(data) {
            const state = stateFor(0);
            if (data.kind === "stale") {
                state.configuration_generation = "2";
            } else if (data.kind === "missing") {
                state.snapshot = null;
            } else if (data.kind === "budget") {
                state.max_cells = 1;
            } else {
                state.status = data.kind;
                state.reason = "Host " + data.kind;
            }
            host.resourceGridState = state;
            compare(view.displayable, false);
            const canvas = findChild(view, "resourceGridCanvas");
            tryCompare(canvas, "paintedCellCount", 0);
            compare(view.cellAt(view.leftAxisMargin + 1, view.topAxisMargin + 1), null);
            compare(findChild(view, "resourceGridMetadata").visible, false);
            verify(findChild(view, "resourceCellInspection").text.indexOf("Select a reported cell") === 0);
        }

        function test_rejected_host_snapshot_preserves_grid_and_freezes_actions() {
            const accepted = view.state;
            const grid = view.grid;
            presentationBudget.maximumBytes = presentationBudget.bytes;
            host.resourceGridState = stateFor(1);
            verify(view.presentationRejected);
            compare(view.state, accepted);
            compare(view.grid, grid);
            compare(view.actionsEnabled, false);
            view.zoom(view.zoomInFactor, 0.5, 0.5);
            compare(querySpy.count, 0);
            compare(view.cellAt(view.leftAxisMargin + 1, view.topAxisMargin + 1), null);
            verify(findChild(view, "resourceGridStatus").text.indexOf("Presentation limits") === 0);
        }

        function test_rejected_pending_request_and_inspection_emit_no_action() {
            const grid = view.grid;
            presentationBudget.maximumBytes = presentationBudget.bytes;
            view.requestWindow(grid.first_symbol, grid.symbol_count, grid.first_carrier, grid.carrier_count, grid.frame_index);
            verify(view.presentationRejected);
            compare(view.pendingWindow, null);
            compare(querySpy.count, 0);
            compare(view.grid, grid);
            compare(view.inspectedCell, null);
            view.inspectAt(view.leftAxisMargin + 1, view.topAxisMargin + 1);
            compare(inspectionSpy.count, 0);
        }

        /* A locally rejected edit must keep the visible frame aligned with its grid. */
        function test_rejected_frame_edit_restores_accepted_selector() {
            const grid = view.grid;
            const selector = findChild(view, "resourceFrameChoice");
            compare(selector.value, grid.frame_index);
            verify(selector.to > selector.from);
            presentationBudget.maximumBytes = presentationBudget.bytes;
            mouseClick(selector.down.indicator);
            verify(view.presentationRejected);
            compare(querySpy.count, 0);
            compare(view.grid, grid);
            compare(selector.value, grid.frame_index);
        }

        function test_wide_identity_tokens_never_use_numeric_conversion() {
            compare(view.sameIdentity("9007199254740992", "9007199254740993"), false);
            compare(view.sameIdentity("18446744073709551615", "18446744073709551615"), true);
            compare(view.scalar("18446744073709551615"), "18446744073709551615");
            compare(view.sameIdentity(null, null), false);
        }

        function test_constant_visual_object_budget_and_resize_data() {
            return [
                {
                    tag: "dark",
                    dark: true
                },
                {
                    tag: "light",
                    dark: false
                }
            ];
        }

        function test_constant_visual_object_budget_and_resize(data) {
            theme.dark = data.dark;
            const firstCount = visualItems(view);
            console.info("ResourceGrid visual items:", firstCount, "for 204 and 396 reported cells");
            verify(firstCount < constantVisualItemBudget);
            restoreOriginal(1);
            compare(visualItems(view), firstCount);
            compare(findChild(view, "resourceGridCanvas").children.length, 1);
            window.width = narrowWidth;
            window.height = narrowHeight;
            verify(dialog.openChecked());
            tryCompare(dialog, "opened", true);
            tryVerify(function () {
                return dialog.width <= window.width - dialog.windowMargin * 2;
            });
            tryVerify(function () {
                return dialog.height <= window.height - dialog.windowMargin * 2;
            });
            const scroller = dialog.contentItem;
            tryVerify(function () {
                return scroller.contentItem.contentHeight > scroller.height;
            });
            const scrollbar = scroller.ScrollBar.vertical;
            mouseDrag(scrollbar, scrollbar.width / 2, scrollbar.height / 5, 0, scrollbar.height / 2);
            verify(scroller.contentItem.contentY > 0);
            keyClick(Qt.Key_Escape);
            tryCompare(dialog, "opened", false);
        }

        function test_render_real_public_grid() {
            wait(renderSettleDelayMs);
            grabImage(view).save("upstream-resource-grid-dark.png");
            theme.dark = false;
            wait(renderSettleDelayMs);
            grabImage(view).save("upstream-resource-grid-light.png");
        }
    }
}
