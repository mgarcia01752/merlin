/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest
import "fixtures"
import "../qml/shared/theme"
import "../qml/shared/components"
import "../qml/downstream/instruments/fec"
import "../qml/downstream/instruments/qam"
import "../qml/shared/presentation"
import "../qml/shared/presentation/LogicalFootprint.js" as Footprint

Item {
    id: root
    width: 640
    height: 480
    PresentationBudget {
        id: presentationBudget
        // Independent capacities for existing presentation-only fixture graphs.
        maximumBytes: 10000000
        maximumEntries: 100000
        maximumViews: 96
        maximumOpen: 12
    }
    Theme { presentationBackendFactory: presentationBudget.createAdapter;
        id: theme
    }
    QtObject {
        id: unsupportedObject
    }
    TlvTreeGrid {
        id: tlv
        theme: theme
        visible: false
    }
    PresentationOwner {id:fecPresentation;theme:theme}
    FecCodewordTable {
        id: fec
        projection:fecPresentation.value || ({})
        theme: theme
        visible: false
    }
    ReceiverCellControls {
        id: selectors
        theme: theme
        visible: false
    }
    TestCase {
        name: "PresentationLogicalFootprint"
        when: windowShown

        // Independent test-host policies, never production/application defaults.
        readonly property int fixtureByteLimit: 65536
        readonly property int fixtureEntryLimit: 16384
        readonly property int fixtureDepthLimit: 64
        readonly property int deepContainerCount: 2048
        readonly property int oversizedArraySlots: 100000

        function limits(bytes, entries, depth) {
            return {
                max_utf16_bytes: bytes,
                max_entries: entries,
                max_depth: depth
            };
        }

        function fixtureLimits() {
            return limits(fixtureByteLimit, fixtureEntryLimit, fixtureDepthLimit);
        }

        function expectCounts(result, bytes, arrays, objects) {
            compare(result.ok, true, result.error || "");
            compare(result.utf16_bytes, bytes);
            compare(result.array_entries, arrays);
            compare(result.object_entries, objects);
            compare(result.entries, arrays + objects);
            compare(Object.keys(result).sort().join(","), "array_entries,entries,object_entries,ok,utf16_bytes");
        }

        function expectError(result, code) {
            compare(result.ok, false);
            compare(result.error, code);
            compare(Object.keys(result).sort().join(","), "error,ok");
        }

        function init() {
            tlv.nodes = [];
            fecPresentation.clear();
            fec.errorsOnly = false;
            selectors.metadata = ({});
        }

        function test_actual_utf16_code_units_data() {
            return [
                {
                    tag: "empty",
                    value: "",
                    bytes: 0
                },
                {
                    tag: "ascii",
                    value: "A",
                    bytes: 2
                },
                {
                    tag: "bmp",
                    value: "é",
                    bytes: 2
                },
                {
                    tag: "surrogate-pair",
                    value: "😀",
                    bytes: 4
                },
                {
                    tag: "unpaired-high",
                    value: "\ud800",
                    bytes: 2
                },
                {
                    tag: "unpaired-low",
                    value: "\udc00",
                    bytes: 2
                },
                {
                    tag: "mixed",
                    value: "a😀é",
                    bytes: 8
                }
            ];
        }

        function test_actual_utf16_code_units(data) {
            expectCounts(Footprint.measure(data.value, limits(data.bytes, 0, 0)), data.bytes, 0, 0);
            if (data.bytes > 0) {
                expectError(Footprint.measure(data.value, limits(data.bytes - 1, 0, 0)), "presentation.utf16_bytes");
            }
        }

        function test_keys_arrays_and_exact_aggregate_limits() {

            // Keys are nine code units; values add A, one pair and é: 26 bytes.
            const value = {
                a: "A",
                face: "😀",
                rows: [null, true, 3, "é"]
            };
            expectCounts(Footprint.measure(value, limits(26, 7, 2)), 26, 4, 3);
            expectError(Footprint.measure(value, limits(25, 7, 2)), "presentation.utf16_bytes");
            expectError(Footprint.measure(value, limits(26, 6, 2)), "presentation.entries");
            expectError(Footprint.measure(value, limits(26, 7, 1)), "presentation.depth");
            const pairedKey = {};
            pairedKey["😀"] = "A";
            expectCounts(Footprint.measure(pairedKey, limits(6, 1, 1)), 6, 0, 1);
            expectError(Footprint.measure(pairedKey, limits(5, 1, 1)), "presentation.utf16_bytes");
        }

        function test_zero_limits_and_finite_scalars() {
            for (const value of [null, true, false, 0, -7, 3.25,
                {},
                []]) {
                expectCounts(Footprint.measure(value, limits(0, 0, 0)), 0, 0, 0);
            }
            expectCounts(Footprint.measure([null,
                {},
                []], limits(0, 3, 1)), 0, 3, 0);
            expectError(Footprint.measure([null], limits(0, 0, 1)), "presentation.entries");
            expectError(Footprint.measure([null], limits(0, 1, 0)), "presentation.depth");
        }

        function test_safe_integer_policies_are_not_truncated() {
            const largest = Number.MAX_SAFE_INTEGER;
            expectCounts(Footprint.measure("A", limits(largest, largest, largest)), 2, 0, 0);
            expectError(Footprint.measure("A", limits(1, largest, largest)), "presentation.utf16_bytes");
        }

        function test_invalid_limits_data() {
            return [
                {
                    tag: "absent",
                    policy: undefined
                },
                {
                    tag: "null",
                    policy: null
                },
                {
                    tag: "array",
                    policy: []
                },
                {
                    tag: "missing-depth",
                    policy: {
                        max_utf16_bytes: 4,
                        max_entries: 4
                    }
                },
                {
                    tag: "negative",
                    policy: limits(-1, 4, 4)
                },
                {
                    tag: "fraction",
                    policy: limits(4, 1.5, 4)
                },
                {
                    tag: "nan",
                    policy: limits(4, 4, NaN)
                },
                {
                    tag: "infinite",
                    policy: limits(Infinity, 4, 4)
                },
                {
                    tag: "overflow",
                    policy: limits(4, Number.MAX_SAFE_INTEGER + 1, 4)
                },
                {
                    tag: "string-limit",
                    policy: limits("4", 4, 4)
                }
            ];
        }

        function test_invalid_limits(data) {
            expectError(Footprint.measure(null, data.policy), "presentation.invalid_limits");
        }

        function test_unsupported_values_data() {
            const sparse = [];
            sparse.length = 1;
            const extended = [];
            extended.extra = "retained";
            const symbolKey = {};
            symbolKey[Symbol("field")] = "retained";
            return [
                {
                    tag: "undefined",
                    value: undefined
                },
                {
                    tag: "function",
                    value: function () {
                        return "must not execute";
                    }
                },
                {
                    tag: "nan",
                    value: NaN
                },
                {
                    tag: "infinite",
                    value: Infinity
                },
                {
                    tag: "negative-infinite",
                    value: -Infinity
                },
                {
                    tag: "date",
                    value: new Date(0)
                },
                {
                    tag: "regexp",
                    value: /presentation/
                },
                {
                    tag: "boxed-number",
                    value: new Number(4)
                },
                {
                    tag: "custom-prototype",
                    value: Object.create({
                        inherited: "retained"
                    })
                },
                {
                    tag: "typed-array",
                    value: new Uint8Array([1])
                },
                {
                    tag: "symbol",
                    value: Symbol("value")
                },
                {
                    tag: "symbol-field",
                    value: symbolKey
                },
                {
                    tag: "sparse-array",
                    value: sparse
                },
                {
                    tag: "array-extra-field",
                    value: extended
                },
                {
                    tag: "qobject",
                    value: unsupportedObject
                }
            ];
        }

        function test_unsupported_values(data) {
            expectError(Footprint.measure(data.value, fixtureLimits()), "presentation.unsupported");
        }

        function test_accessor_values_and_limits_are_never_invoked() {
            let calls = 0;
            const value = {};
            Object.defineProperty(value, "hidden", {
                get: function () {
                    calls += 1;
                    return "not read";
                }
            });
            expectError(Footprint.measure(value, fixtureLimits()), "presentation.unsupported");
            compare(calls, 0);
            const policy = limits(4, 4, 4);
            Object.defineProperty(policy, "max_depth", {
                get: function () {
                    calls += 1;
                    return 4;
                }
            });
            expectError(Footprint.measure(null, policy), "presentation.invalid_limits");
            compare(calls, 0);
        }

        function test_non_enumerable_keys_null_prototype_and_frozen_inputs() {
            const record = Object.create(null);
            Object.defineProperty(record, "hidden", {
                value: "x",
                enumerable: false
            });
            expectCounts(Footprint.measure(record, limits(14, 1, 1)), 14, 0, 1);
            const child = Object.freeze({
                s: "x"
            });
            const array = Object.freeze([true, child]);
            const value = Object.freeze({
                child: child,
                arr: array
            });

            // Included references count twice; the helper does not deduplicate heap storage.
            expectCounts(Footprint.measure(value, limits(24, 6, 3)), 24, 2, 4);
            compare(value.child, child);
            compare(value.arr, array);
            compare(value.arr[1], child);
            compare(child.s, "x");
        }

        function test_cycles_reject_and_repeated_aliases_are_conservative() {
            const self = {};
            self.loop = self;
            expectError(Footprint.measure(self, fixtureLimits()), "presentation.cycle");
            compare(self.loop, self);
            const array = [];
            array.push(array);
            expectError(Footprint.measure(array, fixtureLimits()), "presentation.cycle");
            const first = {};
            const second = {
                next: first
            };
            first.next = second;
            expectError(Footprint.measure(first, fixtureLimits()), "presentation.cycle");
            const alias = {
                s: "x"
            };
            expectCounts(Footprint.measure([alias, alias], limits(8, 4, 2)), 8, 2, 2);
            expectError(Footprint.measure([alias, alias], limits(7, 4, 2)), "presentation.utf16_bytes");
        }

        function test_iterative_depth_and_entry_bounded_traversal() {
            let value = null;
            for (let index = 0; index < deepContainerCount; index++) {
                value = {
                    next: value
                };
            }

            // Each "next" key is four UTF-16 code units; no recursive JS call is needed.
            expectCounts(Footprint.measure(value, limits(16384, 2048, 2048)), 16384, 0, 2048);
            expectError(Footprint.measure(value, limits(16384, 2048, 2047)), "presentation.depth");
            const sparse = [];
            sparse.length = oversizedArraySlots;
            expectError(Footprint.measure(sparse, limits(0, 0, 0)), "presentation.entries");
            let calls = 0;
            const beyond = {};
            Object.defineProperty(beyond, "notVisited", {
                get: function () {
                    calls += 1;
                    return value;
                }
            });
            expectError(Footprint.measure(beyond, limits(0, 0, 0)), "presentation.entries");
            compare(calls, 0);
        }

        function test_actual_tlv_expansion_wrappers() {
            const leaf = {
                name: "leaf",
                value: "y",
                children: []
            };
            const branch = {
                name: "root",
                value: "x",
                children: [leaf]
            };
            tlv.nodes = [branch];
            tlv.expandAll(true);
            compare(tlv.visibleRows.length, 2);
            compare(tlv.visibleRows[0].node, branch);
            compare(tlv.visibleRows[1].node, leaf);
            compare(tlv.visibleRows[1].path, "0.0");

            // Actual node/depth/path/branch wrappers include the leaf twice by reference.
            expectCounts(Footprint.measure(tlv.visibleRows, limits(216, 20, 6)), 216, 3, 17);
            expectError(Footprint.measure(tlv.visibleRows, limits(215, 20, 6)), "presentation.utf16_bytes");
            compare(branch.children[0], leaf);
            compare(leaf.value, "y");
        }

        function test_actual_fec_filtered_array_keeps_original_records() {
            const good = {
                receive_sequence: 0,
                first_source_symbol: 1,
                last_source_symbol: 2,
                ldpc_success: true,
                ldpc_iterations: 3,
                bch_success: true,
                bch_corrected_bits: 0,
                comparison: {
                    bit_errors: 0
                }
            };
            const failed = {
                receive_sequence: 1,
                first_source_symbol: 3,
                last_source_symbol: 4,
                ldpc_success: false,
                ldpc_iterations: 3,
                bch_success: true,
                bch_corrected_bits: 0,
                comparison: {
                    bit_errors: 1
                }
            };
            verify(fecPresentation.replace(fec.prepare([good, failed],0)));
            fec.errorsOnly = true;
            tryCompare(fec.filtered, "length", 1);
            compare(fec.filtered[0], failed);
            expectCounts(Footprint.measure(fec.filtered, limits(258, 10, 3)), 258, 1, 9);

            // Explicit source+filtered inclusion is conservative, not an alias-storage claim.
            expectCounts(Footprint.measure({
                words: fec.projection.words,
                filtered: fec.filtered
            }, limits(800, 32, 4)), 800, 3, 29);
            compare(fec.projection.words.length, 2);
            compare(fec.projection.words[0], good);
        }

        function test_actual_selector_derived_strings_and_entries() {
            selectors.metadata = {
                latest_run_profile: 7,
                runs: [
                    {
                        id: 7,
                        available: true
                    },
                    {
                        id: 9,
                        available: false
                    }
                ],
                profiles: [1, 2],
                orders: [4, 16]
            };
            const runs = findChild(selectors, "receiverRunProfile");
            const profiles = findChild(selectors, "receiverProfile");
            const orders = findChild(selectors, "receiverModulation");
            tryCompare(runs, "count", 3);

            // Qt converts ComboBox.model to a sequence, outside plain-data input.
            expectError(Footprint.measure(runs.model, fixtureLimits()), "presentation.unsupported");
            const runLabels = [];
            const profileLabels = [];
            const orderLabels = [];
            for (let index = 0; index < runs.count; index++) {
                runLabels.push(runs.textAt(index));
            }
            for (let index = 0; index < profiles.count; index++) {
                profileLabels.push(profiles.textAt(index));
            }
            for (let index = 0; index < orders.count; index++) {
                orderLabels.push(orders.textAt(index));
            }

            // Count these actual labels as plain arrays. Production must measure
            // its JS candidate before model conversion and retain that witness.
            expectCounts(Footprint.measure(runLabels, limits(104, 3, 1)), 104, 3, 0);
            expectCounts(Footprint.measure(profileLabels, limits(60, 3, 1)), 60, 3, 0);
            expectCounts(Footprint.measure(orderLabels, limits(22, 2, 1)), 22, 2, 0);
            expectError(Footprint.measure(runLabels, limits(104, 2, 1)), "presentation.entries");
        }

        function test_calls_do_not_cache_or_retain_prior_counts() {
            expectCounts(Footprint.measure("A", limits(2, 0, 0)), 2, 0, 0);
            expectError(Footprint.measure("AB", limits(2, 0, 0)), "presentation.utf16_bytes");
            expectCounts(Footprint.measure(null, limits(0, 0, 0)), 0, 0, 0);
        }
    }
}
