/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "fixtures"
import "../qml/shared/theme"
import "../qml/shared/instruments"
import "../qml/downstream/instruments/qam"

// Presentation fixtures only; these coordinates are not scientific acceptance.
Pane {
    id: window
    width: 1160
    height: 800
    visible: true
    padding: 0
    readonly property int fixturePresentationBytes: 4 * 1024 * 1024
    readonly property int fixturePresentationEntries: 10000
    readonly property int fixtureRetainedViews: 32
    readonly property int fixtureOpenViews: 2
    PresentationBudget {
        id: budget
        maximumBytes: window.fixturePresentationBytes
        maximumEntries: window.fixturePresentationEntries
        maximumViews: window.fixtureRetainedViews
        maximumOpen: window.fixtureOpenViews
    }
    Theme {
        id: theme
        presentationBackendFactory: budget.createAdapter
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
        targetControl: receiver
        theme: theme
    }
    TransmitterScatter {
        id: transmitter
        width: 320
        height: 280
        theme: theme
        observation: ({
                points: []
            })
    }
    QtObject {
        id: backend
        property bool available: true
        property var lastQuery: ({})
        property var points: [
            {
                raw_x: 0.25,
                raw_y: 0.25,
                eq_x: 0.75,
                eq_y: 0.75,
                bin: 320,
                symbol: 10,
                source_symbol: 9
            }
        ]
        function qam_selected(document) {
            lastQuery = JSON.parse(document);
            return JSON.stringify({
                available: available,
                points: available ? points : [],
                profiles: [0],
                orders: [256],
                runs: [],
                retained: available ? points.length : 0,
                first_bin: 320,
                last_bin: 320,
                first_symbol: 10,
                last_symbol: 10,
                correction_mode: "Fixture correction",
                correction_name: "Fixture correction",
                reason: "Original unavailable diagnostic",
                raw_reference_mer: {
                    text: "Raw reference fixture"
                },
                equalized_reference_mer: {
                    text: "Equalized reference fixture"
                }
            });
        }
    }
    ReceiverQamDialog {
        id: receiver
        theme: theme
        backend: backend
    }
    TestCase {
        name: "SharedQamPlot"
        when: windowShown
        readonly property int renderSettleMs: 50
        readonly property int largerPointCount: 512
        readonly property real pixelTolerance: 0.001

        function init() {
            receiver.close();
            tryCompare(receiver, "visible", false);
            backend.available = true;
            budget.maximumBytes = window.fixturePresentationBytes;
            budget.maximumEntries = window.fixturePresentationEntries;
            theme.dark = true;
            transmitter.width = 320;
            transmitter.height = 280;
            transmitter.receivedSamples = false;
            transmitter.resetView();
            transmitter.clipToPlot = false;
            transmitter.observation = {
                points: []
            };
            receiver.rawOnly = false;
            receiver.restoreSelection({
                signal: 0,
                view: 2,
                part: 0,
                run_profile: -1,
                profile: -1,
                order: 0,
                first_bin: -1,
                last_bin: -1,
                first_symbol: -1,
                last_symbol: -1
            });
        }

        function cleanup() {
            receiver.close();
        }

        function openReceiver() {
            verify(receiver.openChecked());
            tryCompare(receiver, "opened", true);
            verify(receiver.refresh());
            tryVerify(function () {
                return findChild(receiver.contentItem, "receiverRawScatter") !== null;
            });
            tryCompare(findChild(receiver.contentItem, "receiverRawScatter"), "paintedPointCount", 1);
        }

        function test_transmitter_original_geometry_palette_and_alias() {
            const value = {
                points: [
                    {
                        x: 0.25,
                        y: 0.75,
                        role: "data",
                        index: 320
                    }
                ]
            };
            transmitter.observation = value;
            tryCompare(transmitter, "paintedPointCount", 1);
            verify(transmitter.points === value.points);
            compare(transmitter.plotEdge, 240);
            compare(transmitter.plotLeft, 40);
            compare(transmitter.plotTop, 20);
            compare(transmitter.pointPosition(value.points[0]), Qt.point(100, 200));
            compare(transmitter.roleColor("data"), theme.qamData);
            compare(transmitter.roleColor("ncp"), theme.qamNcp);
            compare(transmitter.roleColor("filler"), theme.qamFiller);
            compare(transmitter.roleColor("plc"), theme.accent);
            compare(transmitter.roleColor("unclassified"), theme.qamUnknown);
            compare(JSON.stringify(value), '{"points":[{"x":0.25,"y":0.75,"role":"data","index":320}]}');
        }

        function test_transmitter_ncp_last_data() {
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

        function test_transmitter_ncp_last(data) {
            theme.dark = data.dark;
            transmitter.observation = {
                points: [
                    {
                        x: 0.5,
                        y: 0.5,
                        role: "ncp"
                    },
                    {
                        x: 0.5,
                        y: 0.5,
                        role: "data"
                    },
                    {
                        x: 0.2,
                        y: 0.2,
                        role: "filler"
                    }
                ]
            };
            tryCompare(transmitter, "paintedPointCount", 3);
            wait(renderSettleMs);
            const position = transmitter.pointPosition(transmitter.points[0]);
            const image = grabImage(transmitter);
            compare(image.pixel(position.x, position.y), theme.qamNcp);
        }

        function test_transmitter_received_samples_and_resize() {
            transmitter.receivedSamples = true;
            transmitter.observation = {
                points: [
                    {
                        x: 0.2,
                        y: 0.3,
                        role: "data"
                    }
                ]
            };
            tryCompare(transmitter, "paintedPointCount", 1);
            compare(transmitter.receivedPointRadius, 0.85);
            transmitter.width = 200;
            transmitter.height = 400;
            compare(transmitter.plotEdge, 160);
            compare(transmitter.plotLeft, 20);
            compare(transmitter.plotTop, 120);
            compare(transmitter.pointPosition(transmitter.points[0]), Qt.point(52, 168));
        }

        function test_receiver_raw_equalized_coordinates_share_original_owner() {
            openReceiver();
            const raw = findChild(receiver.contentItem, "receiverRawScatter");
            const equalized = findChild(receiver.contentItem, "receiverEqualizedScatter");
            tryCompare(equalized, "paintedPointCount", 1);
            verify(raw.observation === receiver.observation);
            verify(equalized.observation === receiver.observation);
            verify(raw.points === equalized.points);
            compare(raw.xCoordinate, "raw_x");
            compare(raw.yCoordinate, "raw_y");
            compare(equalized.xCoordinate, "eq_x");
            compare(equalized.yCoordinate, "eq_y");
            compare(raw.plotInset, 0);
            compare(raw.axisFontPixels, 11);
            compare(raw.horizontalLabelInset, 10);
            compare(raw.verticalLabelOffset, 12);
            verify(raw.receivedSamples && !raw.roleMarkers && !raw.showAxesWithoutData);
            const point = receiver.observation.points[0];
            fuzzyCompare(raw.pointPosition(point).x, raw.plotLeft + raw.plotEdge * 0.25, pixelTolerance);
            fuzzyCompare(equalized.pointPosition(point).x, equalized.plotLeft + equalized.plotEdge * 0.75, pixelTolerance);
            compare(findChild(receiver.contentItem, "receiverRawTitle").text, "Raw · RX FFT");
            compare(findChild(receiver.contentItem, "receiverRawMer").text, "Raw reference fixture");
            compare(findChild(receiver.contentItem, "receiverEqualizedMer").text, "Equalized reference fixture");
        }

        function test_receiver_ncp_plc_and_single_plane_queries_unchanged() {
            openReceiver();
            receiver.signalIndex = 1;
            verify(receiver.refresh());
            compare(backend.lastQuery.signal, 1);
            receiver.signalIndex = 2;
            receiver.plcPartIndex = 1;
            verify(receiver.refresh());
            compare(backend.lastQuery.signal, 2);
            compare(backend.lastQuery.part, 1);
            compare(findChild(receiver.contentItem, "receiverPlcPart").visible, true);
            receiver.viewIndex = 0;
            verify(receiver.refresh());
            compare(backend.lastQuery.view, 0);
            tryVerify(function () {
                return findChild(receiver.contentItem, "receiverRawScatter") !== null;
            });
            compare(findChild(receiver.contentItem, "receiverEqualizedScatter"), null);
            receiver.viewIndex = 1;
            verify(receiver.refresh());
            compare(backend.lastQuery.view, 1);
            tryVerify(function () {
                return findChild(receiver.contentItem, "receiverEqualizedScatter") !== null;
            });
            compare(findChild(receiver.contentItem, "receiverRawScatter"), null);
            compare(Object.keys(backend.lastQuery).sort().join(","), "first_bin,first_symbol,last_bin,last_symbol,order,part,profile,run_profile,signal,view");
            receiver.rawOnly = true;
            compare(receiver.selectionControls.showView, false);
        }

        function test_receiver_rejection_preserves_plot_and_selector_context() {
            openReceiver();
            const previous = receiver.observation;
            const query = JSON.stringify(receiver.presentationCoverage.value.query);
            budget.maximumBytes = 0;
            receiver.signalIndex = 1;
            verify(!receiver.refresh());
            verify(receiver.observation === previous);
            compare(JSON.stringify(receiver.presentationCoverage.value.query), query);
            compare(receiver.signalIndex, 0);
            compare(findChild(receiver.contentItem, "receiverRawScatter").observation, previous);
            compare(findChild(receiver.contentItem, "receiverQamRetention").visible, true);
        }

        function test_unavailable_and_empty_clear_previous_drawing() {
            openReceiver();
            backend.available = false;
            verify(receiver.refresh());
            const raw = findChild(receiver.contentItem, "receiverRawScatter");
            tryCompare(raw, "paintedPointCount", 0);
            verify(!receiver.observation.available);
            compare(findChild(receiver.contentItem, "receiverQamUnavailable").text, "Original unavailable diagnostic");
            transmitter.observation = {
                points: []
            };
            tryCompare(transmitter, "paintedPointCount", 0);
        }

        function test_bounded_screen_zoom_pan_reset_preserves_original_points() {
            const value = {
                points: [
                    {
                        x: 0.25,
                        y: 0.75,
                        role: "data"
                    }
                ]
            };
            const original = JSON.stringify(value);
            transmitter.observation = value;
            const initial = transmitter.pointPosition(value.points[0]);
            compare(transmitter.magnification, 1);
            compare(transmitter.panX, 0);
            compare(transmitter.panY, 0);
            verify(!transmitter.drawingClipped);
            verify(transmitter.zoom(transmitter.zoomFactor));
            compare(transmitter.magnification, 2);
            compare(transmitter.pointPosition(value.points[0]), Qt.point(40, 260));
            verify(transmitter.pan(24, -24));
            fuzzyCompare(transmitter.pointPosition(value.points[0]).x, 64, pixelTolerance);
            fuzzyCompare(transmitter.pointPosition(value.points[0]).y, 236, pixelTolerance);
            wait(renderSettleMs);
            const axisImage = grabImage(transmitter);
            const axisProbeX = transmitter.plotLeft + transmitter.axisLabelOffset;
            const movedOrigin = transmitter.pointPosition({
                x: transmitter.centerFraction,
                y: transmitter.centerFraction
            });
            verify(String(axisImage.pixel(axisProbeX, movedOrigin.y)) !== String(axisImage.pixel(0, movedOrigin.y)));
            const initialAxisY = transmitter.plotTop + transmitter.plotEdge * transmitter.centerFraction;
            compare(axisImage.pixel(axisProbeX, initialAxisY), axisImage.pixel(0, initialAxisY));
            verify(transmitter.zoom(transmitter.maximumMagnification));
            compare(transmitter.magnification, transmitter.maximumMagnification);
            verify(transmitter.pan(transmitter.plotEdge * transmitter.maximumMagnification, -transmitter.plotEdge * transmitter.maximumMagnification));
            const bound = (transmitter.maximumMagnification - transmitter.minimumMagnification) * transmitter.centerFraction;
            compare(transmitter.panX, bound);
            compare(transmitter.panY, -bound);
            verify(!transmitter.zoom(NaN));
            verify(!transmitter.zoom(0));
            verify(!transmitter.pan(Infinity, 0));
            transmitter.resetView();
            compare(transmitter.magnification, 1);
            compare(transmitter.pointPosition(value.points[0]), initial);
            verify(!transmitter.drawingClipped);
            verify(transmitter.points === value.points);
            compare(JSON.stringify(value), original);
        }

        function test_default_edge_marker_pixels_and_opt_in_clip() {
            transmitter.observation = {
                points: [
                    {
                        x: 0,
                        y: 0.5,
                        role: "ncp"
                    }
                ]
            };
            tryCompare(transmitter, "paintedPointCount", 1);
            wait(renderSettleMs);
            const x = transmitter.plotLeft - transmitter.squarePointHalfWidth;
            const y = transmitter.plotTop + transmitter.plotEdge * transmitter.centerFraction;
            compare(grabImage(transmitter).pixel(x, y), theme.qamNcp);
            transmitter.clipToPlot = true;
            wait(renderSettleMs);
            const image = grabImage(transmitter);
            compare(image.pixel(x, y), image.pixel(0, y));
            transmitter.resetView();
            compare(transmitter.magnification, 1);
        }

        function test_batched_points_create_no_visual_delegates() {
            transmitter.observation = {
                points: [
                    {
                        x: 0.25,
                        y: 0.25,
                        role: "data"
                    }
                ]
            };
            tryCompare(transmitter, "paintedPointCount", 1);
            const count = transmitter.children.length;
            const points = [];
            for (let index = 0; index < largerPointCount; index++) {
                points.push({
                    x: 0.25,
                    y: 0.25,
                    role: "data"
                });
            }
            transmitter.observation = {
                points: points
            };
            tryCompare(transmitter, "paintedPointCount", largerPointCount);
            compare(transmitter.children.length, count);
            receiver.close();
            verify(receiver.openChecked());
            tryCompare(receiver, "opened", true);
            keyClick(Qt.Key_Escape);
            tryCompare(receiver, "visible", false);
        }
    }
}
