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

// Presentation-only host fixtures. None is a validated scientific configuration.
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
    QtObject {
        id: host
        property var presentation: ({})
        property var draftState: ({})
        property var upstreamState: ({})
    }
    UpstreamWorkbench {
        id: workbench
        anchors.fill: parent
        theme: theme
        host: host
        onDraftRequested: function (sectionId) {
            if (tests.replyToRequests) {
                tests.deliverDraft(sectionId);
            }
        }
    }
    SignalSpy {
        id: requested
        target: workbench
        signalName: "draftRequested"
    }
    SignalSpy {
        id: applied
        target: workbench
        signalName: "draftApplyRequested"
    }
    SignalSpy {
        id: cancelled
        target: workbench
        signalName: "draftCancelled"
    }
    SignalSpy {
        id: loadRequested
        target: workbench
        signalName: "configurationLoadRequested"
    }
    TestCase {
        id: tests
        name: "UpstreamDraftPresentation"
        when: windowShown
        readonly property real desktopWidth: 1280
        readonly property real desktopHeight: 780
        readonly property real narrowWidth: 640
        readonly property real narrowHeight: 480
        // Independent host-fixture text/selection policies, not scientific limits.
        readonly property int fixtureTextCharacters: 20
        readonly property int fixtureSelectionCount: 2
        readonly property int fixtureHelpLineCount: 12
        readonly property int renderSettleDelayMs: 100
        readonly property var configurationSections: ["ucd", "profiles", "grants", "source", "channel", "preEqualization", "receiver"]
        readonly property string wideIdentity: "18446744073709551615"
        readonly property string baseIdentity: "9007199254740993"
        property int requestSerial: 0
        property bool replyToRequests: true
        property bool includeLongHelp: false
        property int textCharacterLimit: fixtureTextCharacters

        /* Build bounded display descriptors; no domain admission happens in QML. */
        function fieldsFor(section) {
            if (section === "ucd") {
                return [
                    {
                        key: "preset-choice",
                        field: "selected_preset",
                        label: "Execution preset",
                        kind: "choice",
                        editable: true,
                        choices: [
                            {
                                value: "first-display-fixture",
                                label: "First display fixture"
                            },
                            {
                                value: "second-display-fixture",
                                label: "Second display fixture"
                            }
                        ]
                    },
                    {
                        key: "channel-origin",
                        field: "subcarrier_zero_hz",
                        label: "Carrier-zero frequency (Hz)",
                        kind: "integer",
                        editable: true,
                        max_length: textCharacterLimit
                    }
                ];
            }
            if (section === "profiles") {
                return [
                    {
                        key: "burst/fixture/group/opaque/modulation",
                        field: "modulation_index",
                        label: "Original group modulation index",
                        kind: "choice",
                        editable: true,
                        choices: [
                            {
                                value: "4",
                                label: "Supplied option 4"
                            },
                            {
                                value: "2",
                                label: "Supplied option 2"
                            }
                        ]
                    },
                    {
                        key: "unapproved-decoder-control",
                        field: "ldpc_iterations",
                        label: "Must not be editable here",
                        kind: "integer",
                        editable: true,
                        max_length: textCharacterLimit
                    }
                ];
            }
            const scalarFields = {
                source: ["source_seed"],
                channel: ["delay_samples", "noise_seed", "noise_variance", "phase_radians", "frequency_offset_hz"],
                preEqualization: ["initial_id"],
                receiver: ["estimator", "maximum_pilot_gap", "reference_variance", "equalizer_power_floor", "ldpc_iterations"]
            };
            if (scalarFields[section]) {
                return scalarFields[section].map(field => ({
                            key: section + "/opaque/" + field,
                            field: field,
                            label: "Adapter display field " + field,
                            kind: field === "estimator" ? "choice" : ["noise_variance", "phase_radians", "frequency_offset_hz", "reference_variance", "equalizer_power_floor"].indexOf(field) >= 0 ? "decimal" : "integer",
                            editable: true,
                            max_length: textCharacterLimit,
                            choices: field === "estimator" ? [
                                {
                                    value: "first-adapter-option",
                                    label: "First supplied option"
                                },
                                {
                                    value: "second-adapter-option",
                                    label: "Second supplied option"
                                }
                            ] : []
                        }));
            }
            return [
                {
                    key: "original-grant-id",
                    field: "grant_id",
                    label: "Synthetic grant identity",
                    kind: "integer",
                    editable: true,
                    max_length: textCharacterLimit,
                    help: includeLongHelp ? Array(fixtureHelpLineCount).fill("Supplied adapter field description for scroll coverage.").join("\n") : ""
                },
                {
                    key: "original-slot-selection",
                    field: "minislots",
                    label: "Supplied minislot choices",
                    kind: "selection",
                    editable: true,
                    max_items: fixtureSelectionCount,
                    choices: [
                        {
                            value: "0",
                            label: "Reported slot 0"
                        },
                        {
                            value: "7",
                            label: "Reported slot 7"
                        },
                        {
                            value: "11",
                            label: "Reported slot 11"
                        }
                    ]
                }
            ];
        }

        /* Allocate a new application token; complete documents never enter this fixture. */
        function deliverDraft(section, pending) {
            requestSerial += 1;
            const fields = fieldsFor(section);
            const values = {
                "preset-choice": "first-display-fixture",
                "channel-origin": "0",
                "burst/fixture/group/opaque/modulation": "4",
                "unapproved-decoder-control": "50",
                "original-grant-id": wideIdentity,
                "original-slot-selection": ["7", "0"]
            };
            for (const field of fields) {
                if (!Object.prototype.hasOwnProperty.call(values, field.key)) {
                    values[field.key] = field.kind === "choice" ? field.choices[0].value : field.kind === "decimal" ? "0.5" : field.field === "initial_id" ? wideIdentity : "7";
                }
            }
            host.draftState = {
                section: section,
                token: "display-draft-" + requestSerial,
                baseSession: baseIdentity,
                baseGeneration: wideIdentity,
                values: values,
                options: {
                    fields: fields
                },
                can_apply: true,
                pending: pending === true,
                error: "",
                outcome: "editing"
            };
        }

        /* Publish a new immutable presentation envelope, just as the host must. */
        function reply(outcome, pending, diagnostic, extra) {
            host.draftState = Object.assign({}, host.draftState, {
                outcome: outcome,
                pending: pending,
                error: diagnostic || ""
            }, extra || ({}));
        }

        function popupFor(section) {
            return workbench.sectionDialogs[section];
        }

        /* Loaded editors belong to the popup's visual content hierarchy. */
        function controlFor(popup, name) {
            return findChild(popup.contentItem, name) || findChild(popup.header, name) || findChild(popup.footer, name);
        }

        /* Open through real startup navigation so section and draft forwarding are exercised. */
        function openDraft(section) {
            workbench.openSection(section);
            const popup = popupFor(section);
            tryCompare(popup, "opened", true);
            if (replyToRequests) {
                tryCompare(popup, "valuesLoaded", true);
                compare(requested.signalArguments[requested.count - 1][0], section);
            }
            verify(waitForRendering(popup.contentItem));
            return popup;
        }

        /* Type exact bounded text into the actual control without emitting its change handler directly. */
        function enterText(editor, text) {
            editor.forceActiveFocus(Qt.TabFocusReason);
            keyClick(Qt.Key_A, Qt.ControlModifier);
            for (const character of text) {
                keyClick(character);
            }
        }

        /* Choose a supplied option using its real popup and keyboard navigation. */
        function chooseLast(editor) {
            mouseClick(editor);
            tryCompare(editor.popup, "visible", true);
            keyClick(Qt.Key_End);
            keyClick(Qt.Key_Return);
            tryCompare(editor, "currentIndex", editor.count - 1);
        }

        function init() {
            presentationBudget.maximumBytes = window.fixturePresentationBytes;
            presentationBudget.maximumEntries = window.fixturePresentationEntries;
            presentationBudget.maximumOpen = window.fixtureOpenViews;
            for (const section of configurationSections) {
                popupFor(section).close();
                tryCompare(popupFor(section), "visible", false);
            }
            window.width = desktopWidth;
            window.height = desktopHeight;
            theme.dark = true;
            host.draftState = ({});
            host.upstreamState = ({});
            host.presentation = ({});
            replyToRequests = true;
            includeLongHelp = false;
            textCharacterLimit = fixtureTextCharacters;
            requested.clear();
            applied.clear();
            cancelled.clear();
            loadRequested.clear();
            waitForRendering(workbench);
        }

        function test_load_configuration_is_navigation_only_and_busy_guarded() {
            replyToRequests = false;
            for (const section of configurationSections) {
                const popup = openDraft(section);
                const action = controlFor(popup, "upstreamConfigurationLoad");
                verify(action.enabled);
                mouseClick(action);
                compare(loadRequested.count, 1);
                compare(popup.visible, true);
                compare(popup.token, "");
                compare(applied.count, 0);
                host.upstreamState = {
                    busy: true
                };
                compare(action.enabled, false);
                popup.loadConfiguration();
                compare(loadRequested.count, 1);
                host.upstreamState = {
                    application_busy: true
                };
                compare(action.enabled, false);
                popup.loadConfiguration();
                compare(loadRequested.count, 1);
                host.upstreamState = ({});
                deliverDraft(section, true);
                compare(action.enabled, false);
                popup.loadConfiguration();
                compare(loadRequested.count, 1);
                reply("editing", false);
                compare(action.enabled, true);
                popup.cancelDraft();
                tryCompare(popup, "opened", false);
                loadRequested.clear();
            }
        }

        function test_additional_scalar_sections_submit_changed_only_data() {
            return [
                {
                    tag: "source",
                    section: "source",
                    field: "source_seed",
                    replacement: "123"
                },
                {
                    tag: "channel",
                    section: "channel",
                    field: "delay_samples",
                    replacement: "123"
                },
                {
                    tag: "pre-eq",
                    section: "preEqualization",
                    field: "initial_id",
                    replacement: "18446744073709551614"
                },
                {
                    tag: "receiver",
                    section: "receiver",
                    field: "estimator",
                    replacement: "second-adapter-option"
                }
            ];
        }

        function test_additional_scalar_sections_submit_changed_only(data) {
            const popup = openDraft(data.section);
            const original = host.draftState;
            const key = data.section + "/opaque/" + data.field;
            const editor = controlFor(popup, "draftField_" + key);
            if (data.field === "estimator") {
                chooseLast(editor);
            } else {
                enterText(editor, data.replacement);
            }
            popup.applyDraft();
            compare(applied.count, 1);
            compare(applied.signalArguments[0][0], data.section);
            compare(Object.keys(applied.signalArguments[0][2]).join(","), key);
            compare(applied.signalArguments[0][2][key], data.replacement);
            compare(original.values[key], data.field === "initial_id" ? wideIdentity : data.field === "estimator" ? "first-adapter-option" : "7");
            reply("accepted", false);
            tryCompare(popup, "opened", false);
        }

        function test_decimal_text_is_lossless_until_native_validation_data() {
            return [
                {
                    tag: "receiver-exponent",
                    section: "receiver",
                    field: "reference_variance",
                    text: "1.25e-3"
                },
                {
                    tag: "receiver-negative-zero",
                    section: "receiver",
                    field: "equalizer_power_floor",
                    text: "-0"
                },
                {
                    tag: "channel-phase",
                    section: "channel",
                    field: "phase_radians",
                    text: "-2.5e+2"
                },
                {
                    tag: "channel-nonfinite",
                    section: "channel",
                    field: "noise_variance",
                    text: "NaN"
                },
                {
                    tag: "channel-frequency",
                    section: "channel",
                    field: "frequency_offset_hz",
                    text: "1.2e+3"
                }
            ];
        }

        function test_decimal_text_is_lossless_until_native_validation(data) {
            const popup = openDraft(data.section);
            const key = data.section + "/opaque/" + data.field;
            const editor = controlFor(popup, "draftField_" + key);
            compare(editor.maximumLength, fixtureTextCharacters);
            enterText(editor, data.text);
            compare(editor.text, data.text);
            popup.applyDraft();
            compare(applied.signalArguments[0][2][key], data.text);
            compare(typeof applied.signalArguments[0][2][key], "string");
            reply("rejected", false, "Original native validation diagnostic fixture");
            compare(popup.opened, true);
            compare(popup.changes[key], data.text);
            compare(popup.diagnostic, "Original native validation diagnostic fixture");
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
        }

        function test_optional_configuration_absent_has_no_invented_fields_data() {
            return [
                {
                    tag: "channel",
                    section: "channel"
                },
                {
                    tag: "pre-eq",
                    section: "preEqualization"
                }
            ];
        }

        function test_optional_configuration_absent_has_no_invented_fields(data) {
            replyToRequests = false;
            const popup = openDraft(data.section);
            deliverDraft(data.section, true);
            reply("editing", false, "", {
                options: {
                    fields: []
                },
                values: {},
                can_apply: false
            });
            compare(popup.fields.length, 0);
            compare(popup.canApply, false);
            compare(Object.keys(popup.originalValues).length, 0);
            verify(controlFor(popup, "upstreamDraftStatus").text.indexOf("unavailable") >= 0);
            compare(controlFor(popup, "upstreamConfigurationLoad").enabled, true);
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
        }

        function test_transmitter_navigation_preserves_processing_stages() {
            const transmitter = popupFor("transmitter");
            compare(transmitter.stages.join(" | "), "PRBS source → FEC encoder → QAM mapper | Pilot / resource allocation → frequency interleaving | CM Pre-Eq C → IFFT → waveform output");
            workbench.openSection("transmitter");
            tryCompare(transmitter, "opened", true);
            mouseClick(controlFor(transmitter, "upstreamTransmitterConfigureSource"));
            tryCompare(transmitter, "opened", false);
            const source = popupFor("source");
            tryCompare(source, "opened", true);
            compare(requested.signalArguments[requested.count - 1][0], "source");
            compare(source.fields[0].field, "source_seed");
            source.cancelDraft();
            tryCompare(source, "opened", false);
            compare(popupFor("receiver").stages.join(" | "), "Burst detection / framing → timing → FFT | Pilot-based channel estimation → equalization | Resource deinterleaving → soft demapping → LDPC decoding | Received-data recovery");
        }

        function test_missing_adapter_has_no_editable_or_accepted_draft() {
            replyToRequests = false;
            const popup = openDraft("ucd");
            compare(popup.token, "");
            compare(controlFor(popup, "upstreamDraftApply").enabled, false);
            verify(controlFor(popup, "upstreamDraftStatus").text.indexOf("unavailable") >= 0);
            mouseClick(controlFor(popup, "popupCloseButton"));
            tryCompare(popup, "opened", false);
            compare(applied.count, 0);
            compare(cancelled.count, 0);
        }

        function test_choice_submits_changed_only_and_waits_for_acceptance() {
            const popup = openDraft("ucd");
            const original = host.draftState;
            chooseLast(controlFor(popup, "draftField_preset-choice"));
            mouseClick(controlFor(popup, "upstreamDraftApply"));
            compare(applied.count, 1);
            compare(applied.signalArguments[0][0], "ucd");
            compare(applied.signalArguments[0][1], original.token);
            const values = applied.signalArguments[0][2];
            compare(Object.keys(values).join(","), "preset-choice");
            compare(values["preset-choice"], "second-display-fixture");
            compare(original.values["preset-choice"], "first-display-fixture");
            compare(popup.opened, true);
            compare(controlFor(popup, "upstreamDraftApply").enabled, false);
            reply("editing", true);
            compare(popup.opened, true);
            reply("accepted", false);
            tryCompare(popup, "opened", false);
            tryCompare(findChild(workbench, "upstreamUcdBlock"), "activeFocus", true);
        }

        function test_wide_integer_text_and_host_limit_preserve_exact_digits() {
            const popup = openDraft("grants");
            const editor = controlFor(popup, "draftField_original-grant-id");
            compare(editor.text, wideIdentity);
            compare(editor.maximumLength, fixtureTextCharacters);
            const replacement = "18446744073709551614";
            enterText(editor, replacement + "9");
            compare(editor.text, replacement);
            mouseClick(controlFor(popup, "upstreamDraftApply"));
            compare(applied.signalArguments[0][2]["original-grant-id"], replacement);
            compare(typeof applied.signalArguments[0][2]["original-grant-id"], "string");
            compare(host.draftState.values["original-grant-id"], wideIdentity);
            reply("accepted", false);
            tryCompare(popup, "opened", false);
        }

        function test_opaque_profile_key_and_section_whitelist() {
            const popup = openDraft("profiles");
            compare(popup.fields.length, 1);
            compare(controlFor(popup, "draftField_unapproved-decoder-control"), null);
            const key = "burst/fixture/group/opaque/modulation";
            chooseLast(controlFor(popup, "draftField_" + key));
            popup.applyDraft();
            compare(Object.keys(applied.signalArguments[0][2]).join(","), key);
            compare(applied.signalArguments[0][2][key], "2");
            compare(host.draftState.values["unapproved-decoder-control"], "50");
            reply("accepted", false);
            tryCompare(popup, "opened", false);
        }

        function test_explicit_selection_order_and_host_count_are_preserved() {
            const popup = openDraft("grants");
            const selected = host.draftState.values["original-slot-selection"];
            const extra = controlFor(popup, "draftSelection_original-slot-selection_11");
            const first = controlFor(popup, "draftSelection_original-slot-selection_7");
            compare(extra.enabled, false);
            mouseClick(first);
            compare(extra.enabled, true);
            mouseClick(extra);
            compare(popup.valueFor("original-slot-selection").join(","), "0,11");
            compare(selected.join(","), "7,0");
            popup.applyDraft();
            compare(applied.signalArguments[0][2]["original-slot-selection"].join(","), "0,11");
            reply("accepted", false);
            tryCompare(popup, "opened", false);
        }

        function test_reverted_change_and_authoritative_can_apply() {
            const popup = openDraft("ucd");
            const editor = controlFor(popup, "draftField_channel-origin");
            enterText(editor, "123");
            compare(popup.canApply, true);
            reply("editing", false, "", {
                can_apply: false
            });
            compare(popup.canApply, false);
            popup.applyDraft();
            compare(applied.count, 0);
            reply("editing", false, "", {
                can_apply: true
            });
            enterText(editor, "0");
            compare(Object.keys(popup.changes).length, 0);
            compare(popup.canApply, false);
            keyClick(Qt.Key_Escape);
            tryCompare(popup, "opened", false);
        }

        function test_unrelated_terminal_cannot_adopt_or_close_data() {
            return [
                {
                    tag: "token",
                    extra: {
                        token: "unrelated-draft"
                    }
                },
                {
                    tag: "section",
                    extra: {
                        section: "profiles"
                    }
                },
                {
                    tag: "session",
                    extra: {
                        baseSession: "9007199254740992"
                    }
                },
                {
                    tag: "generation",
                    extra: {
                        baseGeneration: "18446744073709551614"
                    }
                }
            ];
        }

        function test_unrelated_terminal_cannot_adopt_or_close(data) {
            const popup = openDraft("ucd");
            enterText(controlFor(popup, "draftField_channel-origin"), "123");
            popup.applyDraft();
            const envelope = host.draftState;
            reply("accepted", false, "", data.extra);
            compare(popup.opened, true);
            compare(popup.submitting, true);
            compare(popup.changes["channel-origin"], "123");
            host.draftState = Object.assign({}, envelope, {
                outcome: "accepted"
            });
            tryCompare(popup, "opened", false);
        }

        function test_rejected_or_stale_keeps_draft_and_exact_error_data() {
            return [
                {
                    tag: "rejected",
                    outcome: "rejected"
                },
                {
                    tag: "stale",
                    outcome: "stale"
                }
            ];
        }

        function test_rejected_or_stale_keeps_draft_and_exact_error(data) {
            const popup = openDraft("ucd");
            enterText(controlFor(popup, "draftField_channel-origin"), "123");
            popup.applyDraft();
            reply(data.outcome, false, "<host diagnostic> & exact context");
            compare(popup.opened, true);
            compare(popup.submitting, false);
            compare(controlFor(popup, "upstreamDraftError").text, "<host diagnostic> & exact context");
            compare(controlFor(popup, "upstreamDraftError").textFormat, Text.PlainText);
            compare(popup.changes["channel-origin"], "123");
            if (data.outcome === "stale") {
                compare(popup.canApply, false);
            }
            mouseClick(controlFor(popup, "upstreamDraftCancel"));
            tryCompare(popup, "opened", false);
        }

        function test_pending_cancel_waits_for_exact_terminal_data() {
            return [
                {
                    tag: "cancel",
                    action: "cancel",
                    outcome: "cancelled"
                },
                {
                    tag: "title-x",
                    action: "x",
                    outcome: "rejected"
                },
                {
                    tag: "escape",
                    action: "escape",
                    outcome: "stale"
                }
            ];
        }

        function test_pending_cancel_waits_for_exact_terminal(data) {
            const popup = openDraft("ucd");
            enterText(controlFor(popup, "draftField_channel-origin"), "123");
            popup.applyDraft();
            const token = popup.token;
            const envelope = host.draftState;
            if (data.action === "cancel") {
                mouseClick(controlFor(popup, "upstreamDraftCancel"));
            } else if (data.action === "x") {
                mouseClick(controlFor(popup, "popupCloseButton"));
            } else {
                keyClick(Qt.Key_Escape);
            }
            compare(cancelled.count, 1);
            compare(cancelled.signalArguments[0][0], token);
            compare(popup.opened, true);
            compare(popup.cancellationRequested, true);
            popup.cancelDraft();
            compare(cancelled.count, 1);
            reply("cancelled", false, "", {
                token: "other-token"
            });
            compare(popup.opened, true);
            host.draftState = Object.assign({}, envelope, {
                outcome: data.outcome,
                pending: false
            });
            tryCompare(popup, "opened", false);
        }

        function test_reopen_requires_new_token_and_discards_local_changes() {
            const popup = openDraft("ucd");
            const previous = host.draftState;
            enterText(controlFor(popup, "draftField_channel-origin"), "123");
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
            replyToRequests = false;
            openDraft("ucd");
            compare(popup.valuesLoaded, false);
            compare(Object.keys(popup.changes).length, 0);
            host.draftState = Object.assign({}, previous, {
                outcome: "accepted"
            });
            compare(popup.opened, true);
            compare(popup.token, "");
            deliverDraft("ucd");
            tryCompare(popup, "valuesLoaded", true);
            compare(controlFor(popup, "draftField_channel-origin").text, "0");
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
        }

        function test_initial_pending_draft_cancels_before_values_arrive() {
            replyToRequests = false;
            const popup = openDraft("ucd");
            deliverDraft("ucd", true);
            compare(popup.token, host.draftState.token);
            compare(popup.valuesLoaded, false);
            mouseClick(controlFor(popup, "popupCloseButton"));
            compare(cancelled.count, 1);
            compare(cancelled.signalArguments[0][0], host.draftState.token);
            compare(popup.opened, true);
            reply("cancelled", false);
            tryCompare(popup, "opened", false);
        }

        function test_acceptance_racing_cancel_uses_reported_terminal() {
            const popup = openDraft("ucd");
            enterText(controlFor(popup, "draftField_channel-origin"), "123");
            popup.applyDraft();
            popup.cancelDraft();
            compare(cancelled.count, 1);
            reply("accepted", false);
            tryCompare(popup, "opened", false);
            compare(cancelled.count, 1);
        }

        function test_missing_metadata_fails_closed_data() {
            return [
                {
                    tag: "no-editability",
                    field: {
                        editable: undefined
                    }
                },
                {
                    tag: "no-text-limit",
                    field: {
                        max_length: undefined
                    }
                },
                {
                    tag: "no-choices",
                    field: {
                        kind: "choice",
                        choices: []
                    }
                }
            ];
        }

        function test_missing_metadata_fails_closed(data) {
            replyToRequests = false;
            const popup = openDraft("ucd");
            deliverDraft("ucd");
            const state = host.draftState;
            const field = Object.assign({}, state.options.fields[1], data.field);
            for (const key of Object.keys(field)) {
                if (field[key] === undefined) {
                    delete field[key];
                }
            }
            // Missing JSON members are omitted, rather than retained undefined values.
            // Use another new draft response before the first values are adopted.
            popup.close();
            tryCompare(popup, "opened", false);
            openDraft("ucd");
            host.draftState = Object.assign({}, state, {
                token: state.token + "-metadata",
                options: {
                    fields: [field]
                }
            });
            tryCompare(popup, "valuesLoaded", true);
            compare(controlFor(popup, "draftField_channel-origin").enabled, false);
            compare(popup.canApply, false);
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
        }

        function test_candidate_rejection_preserves_coherent_draft_data() {
            return [
                {
                    tag: "bytes",
                    dimension: "bytes"
                },
                {
                    tag: "entries",
                    dimension: "entries"
                }
            ];
        }

        function test_candidate_rejection_preserves_coherent_draft(data) {
            const popup = openDraft("ucd");
            const previous = popup.draft;
            const bytes = presentationBudget.bytes;
            const entries = presentationBudget.entries;
            if (data.dimension === "bytes") {
                presentationBudget.maximumBytes = bytes;
            } else {
                presentationBudget.maximumEntries = entries;
            }
            compare(popup.changeField(popup.fields[1], "123"), false);
            compare(popup.draft, previous);
            compare(popup.valueFor("channel-origin"), "0");
            compare(Object.keys(popup.changes).length, 0);
            compare(popup.token, host.draftState.token);
            compare(popup.canApply, false);
            compare(presentationBudget.bytes, bytes);
            compare(presentationBudget.entries, entries);
            verify(controlFor(popup, "upstreamDraftAdmissionError").visible);
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
        }

        function test_text_capacity_rejection_leaves_blank_disabled_editor() {
            // This fixture descriptor exceeds its independent presentation policy.
            // No scientific parameter bound is asserted by this test.
            textCharacterLimit = window.fixturePresentationBytes;
            const popup = openDraft("ucd");
            const editor = controlFor(popup, "draftField_channel-origin");
            compare(popup.valuesLoaded, true);
            compare(editor.maximumLength, textCharacterLimit);
            compare(editor.textAdmitted, false);
            compare(editor.text, "");
            compare(editor.enabled, false);
            compare(popup.canApply, false);
            verify(controlFor(popup, "upstreamDraftAdmissionError").visible);
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
            textCharacterLimit = fixtureTextCharacters;
            const reopened = openDraft("ucd");
            compare(controlFor(reopened, "draftField_channel-origin").text, "0");
            compare(reopened.textAdmitted, true);
            reopened.cancelDraft();
            tryCompare(reopened, "opened", false);
            // A previous text reservation must not authorize a larger new form.
            textCharacterLimit = window.fixturePresentationBytes;
            const larger = openDraft("ucd");
            compare(larger.editorTextCoverage.textAdmitted, true);
            compare(larger.textAdmitted, false);
            compare(controlFor(larger, "draftField_channel-origin").text, "");
            compare(controlFor(larger, "draftField_channel-origin").enabled, false);
            larger.cancelDraft();
            tryCompare(larger, "opened", false);
        }

        function test_dynamic_integer_fields_share_one_text_complement() {
            replyToRequests = false;
            const popup = openDraft("ucd");
            deliverDraft("ucd", true);
            const state = host.draftState;
            const viewsBeforeText = presentationBudget.views;
            const extra = Object.assign({}, state.options.fields[1], {
                key: "channel-rolloff",
                field: "rolloff_samples",
                label: "Second bounded integer display fixture"
            });
            reply("editing", false, "", {
                options: {
                    fields: state.options.fields.concat([extra])
                },
                values: Object.assign({}, state.values, {
                    "channel-rolloff": "5"
                })
            });
            tryCompare(popup, "textAdmitted", true);
            // The one text owner may already hold coverage from an earlier open.
            verify(presentationBudget.views <= viewsBeforeText + 1);
            compare(controlFor(popup, "draftField_channel-origin").maximumLength, fixtureTextCharacters);
            const second = controlFor(popup, "draftField_channel-rolloff");
            compare(second.maximumLength, fixtureTextCharacters);
            compare(second.text, "5");
            enterText(second, "123");
            popup.applyDraft();
            compare(Object.keys(applied.signalArguments[0][2]).join(","), "channel-rolloff");
            compare(applied.signalArguments[0][2]["channel-rolloff"], "123");
            reply("accepted", false);
            tryCompare(popup, "opened", false);
        }

        function test_open_rejection_has_no_draft_request_or_retained_candidate() {
            const popup = popupFor("ucd");
            presentationBudget.maximumOpen = 0;
            compare(workbench.openSection("ucd"), false);
            compare(popup.visible, false);
            compare(popup.openAdmissionRejected, true);
            compare(requested.count, 0);
            compare(popup.draftCoverage.value, null);
            compare(popup.observationCoverage.value, null);
        }

        function test_closed_and_never_opened_dialogs_do_not_copy_host_snapshots() {
            const popup = popupFor("ucd");
            host.presentation = {
                details: {
                    ucd: {
                        summary: "Closed display fixture",
                        rows: [
                            {
                                label: "Context",
                                value: "Parent alias"
                            }
                        ]
                    }
                }
            };
            deliverDraft("ucd");
            compare(popup.draftCoverage.value, null);
            compare(popup.observationCoverage.value, null);
            openDraft("ucd");
            verify(popup.draftCoverage.value !== null);
            verify(popup.observationCoverage.value !== null);
            compare(controlFor(popup, "upstreamObservationSummary").text, "Closed display fixture");
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
            compare(popup.draftCoverage.value, null);
            compare(popup.observationCoverage.value, null);
            host.presentation = {
                details: {
                    ucd: {
                        summary: "Later closed fixture"
                    }
                }
            };
            deliverDraft("ucd");
            compare(popup.draftCoverage.value, null);
            compare(popup.observationCoverage.value, null);
        }

        function test_unadmitted_fresh_identity_is_cancelled_on_close() {
            replyToRequests = false;
            const popup = openDraft("ucd");
            presentationBudget.maximumBytes = presentationBudget.bytes;
            deliverDraft("ucd", true);
            const token = host.draftState.token;
            compare(popup.token, "");
            compare(popup.presentationRejected, true);
            popup.close();
            tryCompare(popup, "opened", false);
            compare(cancelled.count, 1);
            compare(cancelled.signalArguments[0][0], token);
            compare(popup.draftCoverage.value, null);
            popup.requestCancellation();
            compare(cancelled.count, 1);
        }

        function test_forced_close_suppresses_exact_token_once_and_clears_aliases() {
            const popup = openDraft("ucd");
            enterText(controlFor(popup, "draftField_channel-origin"), "123");
            popup.applyDraft();
            const token = popup.token;
            popup.close();
            tryCompare(popup, "opened", false);
            compare(cancelled.count, 1);
            compare(cancelled.signalArguments[0][0], token);
            compare(popup.draftCoverage.value, null);
            compare(popup.observationCoverage.value, null);
            reply("accepted", false);
            compare(popup.visible, false);
            compare(popup.draftCoverage.value, null);
        }

        function test_rejected_optional_error_does_not_block_terminal_transition() {
            const popup = openDraft("ucd");
            enterText(controlFor(popup, "draftField_channel-origin"), "123");
            popup.applyDraft();
            presentationBudget.maximumBytes = presentationBudget.bytes;
            reply("rejected", false, "Unadmitted optional diagnostic fixture");
            compare(popup.submitting, false);
            compare(popup.changes["channel-origin"], "123");
            compare(popup.presentationRejected, true);
            compare(popup.diagnostic, "");
            compare(popup.canApply, false);
            reply("accepted", false, "Another optional diagnostic fixture");
            tryCompare(popup, "opened", false);
            compare(popup.draftCoverage.value, null);
        }

        function test_unsupported_descriptor_is_rejected_without_coercion() {
            replyToRequests = false;
            const popup = openDraft("ucd");
            deliverDraft("ucd", true);
            const state = host.draftState;
            const field = Object.assign({}, state.options.fields[1], {
                max_length: NaN
            });
            reply("editing", false, "", {
                options: {
                    fields: [field]
                }
            });
            compare(popup.valuesLoaded, false);
            compare(popup.fields.length, 0);
            compare(popup.presentationRejected, true);
            compare(popup.canApply, false);
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
        }

        function test_theme_narrow_resize_scroll_keyboard_footer_data() {
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

        function test_theme_narrow_resize_scroll_keyboard_footer(data) {
            theme.dark = data.dark;
            includeLongHelp = true;
            window.width = narrowWidth;
            window.height = narrowHeight;
            const popup = openDraft("grants");
            compare(popup.background.color, theme.panel);
            verify(popup.width <= window.width - popup.windowMargin * 2);
            verify(popup.height <= window.height - popup.windowMargin * 2);
            const apply = controlFor(popup, "upstreamDraftApply");
            const cancel = controlFor(popup, "upstreamDraftCancel");
            compare(apply.width, cancel.width);
            compare(apply.height, cancel.height);
            verify(apply.x < cancel.x);
            const scroller = popup.contentItem;
            tryVerify(function () {
                return scroller.contentItem.contentHeight > scroller.height;
            });
            const bar = scroller.ScrollBar.vertical;
            mouseDrag(bar, bar.width / 2, bar.height / 5, 0, bar.height / 2);
            verify(scroller.contentItem.contentY > 0);
            cancel.forceActiveFocus(Qt.TabFocusReason);
            keyClick(Qt.Key_Space);
            tryCompare(popup, "opened", false);
        }

        function test_render_draft_to_isolated_evidence() {
            const popup = openDraft("grants");
            wait(renderSettleDelayMs);
            compare(grabImage(window).width, window.width);
            grabImage(window).save("upstream-draft-grants-dark.png");
            theme.dark = false;
            wait(renderSettleDelayMs);
            compare(grabImage(window).height, window.height);
            grabImage(window).save("upstream-draft-grants-light.png");
            popup.cancelDraft();
            tryCompare(popup, "opened", false);
        }
    }
}
