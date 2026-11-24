/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../fixtures"
import "../../qml/shared/theme"
import "../../qml/shell/directions"
import "../../qml/upstream"

// Actual presentation owners/routing; only native engine responses are fixtures.
Pane {
    id: window

    readonly property int fixtureEntries: 100000
    readonly property string fixtureFieldKey: "ucd/opaque/randomization_seed"
    readonly property string fixtureGeneration: "9007199254740993"
    readonly property string fixtureLoadedGeneration: "9007199254740995"
    readonly property int fixtureLogicalBytes: 4 * 1024 * 1024
    readonly property int fixtureOpenViews: 4
    readonly property string fixtureSession: "18446744073709551615"
    readonly property int fixtureTextCharacters: 20
    readonly property int fixtureViews: 64
    readonly property string importError: "Original native import diagnostic <not markup>."

    height: 780
    padding: 0
    visible: true
    width: 1280

    PresentationBudget {
        id: budget

        maximumBytes: window.fixtureLogicalBytes
        maximumEntries: window.fixtureEntries
        maximumOpen: window.fixtureOpenViews
        maximumViews: window.fixtureViews
    }
    Theme {
        id: fixtureTheme

        presentationBackendFactory: budget.createAdapter
    }
    ThemePalette {
        targetControl: window
        theme: fixtureTheme
    }
    QtObject {
        id: metadata

        readonly property string upstreamIdentity: "upstream/" + window.fixtureSession + "/" + window.fixtureGeneration

        function accepts(direction, identity) {
            return direction === "upstream" && identity === upstreamIdentity;
        }
        function state() {
            return JSON.stringify({
                selectedDirection: "upstream",
                downstreamIdentity: "downstream/fixture/unchanged",
                upstreamIdentity: upstreamIdentity,
                executionStatus: "",
                applicationBusy: engine.snapshot.busy
            });
        }
    }
    QtObject {
        id: engine

        property string acceptedGeneration: window.fixtureGeneration
        property string acceptedValue: "0"
        property var appliedValues: ({})
        property int applyRequests: 0
        property var cancelledTokens: []
        property int draftRequests: 0
        property string pickerOutcome: "cancel"
        property int pickerRequests: 0
        property var snapshot: initialState()

        function apply_draft(section, token, values) {
            applyRequests += 1;
            appliedValues = JSON.parse(values);
            if (snapshot.draft.section !== section || snapshot.draft.token !== token) {
                return "Fixture rejected a foreign Apply origin.";
            }
            publishDraft({
                pending: true
            });
            return "";
        }
        function begin_draft(section) {
            draftRequests += 1;
            const values = {};
            values[window.fixtureFieldKey] = acceptedValue;
            snapshot = Object.assign({}, snapshot, {
                error: "",
                draft: {
                    section: section,
                    token: "native-fixture-draft-" + draftRequests,
                    baseSession: window.fixtureSession,
                    baseGeneration: acceptedGeneration,
                    values: values,
                    options: {
                        fields: [
                            {
                                key: window.fixtureFieldKey,
                                field: "randomization_seed",
                                label: "Native fixture randomization seed",
                                kind: "integer",
                                editable: true,
                                max_length: window.fixtureTextCharacters
                            }
                        ]
                    },
                    can_apply: true,
                    pending: false,
                    error: "",
                    outcome: "editing"
                }
            });
            return "";
        }
        function cancel_draft(token) {
            cancelledTokens = cancelledTokens.concat([token]);
            // Queued import owns a separate operation; an old draft cannot stop it.
            if (!snapshot.preparing && snapshot.draft.token === token && !snapshot.draft.pending) {
                publishDraft({
                    outcome: "cancelled"
                });
            }
            return "";
        }
        function choose_configuration() {
            pickerRequests += 1;
            if (pickerOutcome === "queued") {
                snapshot = Object.assign({}, snapshot, {
                    preparing: true,
                    busy: true
                });
            }
            return "";
        }

        /* A bounded mock DTO, never an accepted scientific configuration. */
        function initialState() {
            return {
                configured: true,
                preparing: false,
                busy: false,
                application_busy: false,
                can_start: true,
                has_result: false,
                result: null,
                progress: null,
                error: "",
                configuration_view: {
                    blocks: {},
                    details: {}
                },
                draft: {}
            };
        }
        function poll() {
            return state();
        }
        function publishDraft(changes) {
            snapshot = Object.assign({}, snapshot, {
                draft: Object.assign({}, snapshot.draft, changes)
            });
        }
        function settleImport(error) {
            if (!error) {
                acceptedValue = "7";
                acceptedGeneration = window.fixtureLoadedGeneration;
            }
            snapshot = Object.assign({}, snapshot, {
                preparing: false,
                busy: false,
                error: error
            });
        }
        function state() {
            return JSON.stringify(snapshot);
        }
    }
    DirectionShellHost {
        id: host

        backend: metadata
        downstreamBackend: engine
        theme: fixtureTheme
        upstreamBackend: engine
        upstreamView: workbench
    }
    UpstreamWorkbench {
        id: workbench

        anchors.fill: parent
        host: host
        theme: fixtureTheme

        onConfigurationLoadRequested: host.loadConfiguration()
        onDraftApplyRequested: function (section, token, values) {
            host.applyDraft(section, token, values);
        }
        onDraftCancelled: function (token) {
            host.cancelDraft(token);
        }
        onDraftRequested: function (section) {
            host.beginDraft(section);
        }
    }
    TestCase {
        id: tests

        function button(name) {
            return findChild(popup().footer, name);
        }
        function cleanup() {
            budget.maximumBytes = window.fixtureLogicalBytes;
            budget.maximumEntries = window.fixtureEntries;
            popup().close();
            tryCompare(popup(), "visible", false);
        }

        /* Reject every optional replacement while existing owners remain retained. */
        function holdWholeHostPressure() {
            verify(budget.bytes > 0);
            budget.maximumBytes = 0;
            verify(!host.poll());
            compare(host.presentationRejected, true);
        }
        function init() {
            budget.maximumBytes = window.fixtureLogicalBytes;
            budget.maximumEntries = window.fixtureEntries;
            budget.maximumOpen = window.fixtureOpenViews;
            for (const section of Object.keys(workbench.sectionDialogs)) {
                workbench.sectionDialogs[section].close();
            }
            tryCompare(popup(), "visible", false);
            engine.snapshot = engine.initialState();
            engine.draftRequests = 0;
            engine.applyRequests = 0;
            engine.pickerRequests = 0;
            engine.cancelledTokens = [];
            engine.appliedValues = ({});
            engine.pickerOutcome = "cancel";
            engine.acceptedValue = "0";
            engine.acceptedGeneration = window.fixtureGeneration;
            host.reopenConfigurationIndex = host.noRequestedSection;
            verify(host.poll());
        }

        /* Exercise actual navigation, descriptor loading and text reservation. */
        function openDraft() {
            verify(workbench.openSection("ucd"));
            tryCompare(popup(), "opened", true);
            tryCompare(popup(), "valuesLoaded", true);
            tryCompare(popup(), "textAdmitted", true);
            compare(popup().baseSession, window.fixtureSession);
            compare(popup().baseGeneration, engine.acceptedGeneration);
            return popup().token;
        }
        function popup() {
            return workbench.sectionDialogs.ucd;
        }

        /* Edit the real descriptor-created TextField and click its actual Apply. */
        function submitDraft() {
            const editor = findChild(popup().contentItem, "draftField_" + window.fixtureFieldKey);
            verify(editor !== null);
            verify(editor.enabled);
            editor.text = "1";
            editor.textEdited();
            tryCompare(popup(), "canApply", true);
            const apply = button("upstreamDraftApply");
            verify(apply.enabled);
            mouseClick(apply);
            compare(engine.applyRequests, 1);
            compare(engine.appliedValues[window.fixtureFieldKey], "1");
            compare(popup().pending, true);
            compare(popup().visible, true);
            compare(host.draftState.pending, true);
        }
        function test_accepted_terminal_closes_under_held_whole_host_pressure() {
            const token = openDraft();
            submitDraft();
            holdWholeHostPressure();
            const held = host.presentation;
            engine.publishDraft({
                outcome: "accepted",
                pending: false
            });
            verify(!host.poll());
            tryCompare(popup(), "visible", false);
            compare(host.presentation, held);
            compare(host.draftState.token, token);
            compare(host.draftState.pending, true);
            compare(engine.cancelledTokens.indexOf(token), -1);
        }
        function test_failed_load_keeps_original_error_without_reopening_draft() {
            openDraft();
            engine.pickerOutcome = "queued";
            mouseClick(findChild(popup().footer, "upstreamConfigurationLoad"));
            compare(engine.pickerRequests, 1);
            tryCompare(popup(), "visible", false);
            compare(host.reopenConfigurationIndex, host.configurationSections().indexOf("ucd"));
            engine.settleImport(window.importError);
            verify(host.poll());
            // The old accepted configuration still exists; its presence must not reopen.
            compare(host.upstreamState.configured, true);
            compare(host.upstreamState.error, window.importError);
            compare(engine.snapshot.error, window.importError);
            compare(host.reopenConfigurationIndex, host.noRequestedSection);
            waitForRendering(workbench);
            compare(popup().visible, false);
            compare(engine.draftRequests, 1);
            verify(host.poll());
            compare(engine.snapshot.error, window.importError);
            compare(host.upstreamState.error, window.importError);
        }
        function test_foreign_terminal(data) {
            const token = openDraft();
            submitDraft();
            const original = engine.snapshot.draft;
            holdWholeHostPressure();
            engine.publishDraft(Object.assign({
                outcome: "accepted",
                pending: false
            }, data.changes));
            verify(!host.poll());
            compare(popup().visible, true);
            compare(popup().token, token);
            compare(popup().pending, true);
            compare(popup().terminalReceived, false);
            engine.snapshot = Object.assign({}, engine.snapshot, {
                draft: Object.assign({}, original, {
                    outcome: "accepted",
                    pending: false
                })
            });
            verify(!host.poll());
            tryCompare(popup(), "visible", false);
        }
        function test_foreign_terminal_data() {
            return [
                {
                    tag: "token",
                    changes: {
                        token: "foreign-native-token"
                    }
                },
                {
                    tag: "session",
                    changes: {
                        baseSession: "18446744073709551614"
                    }
                },
                {
                    tag: "generation",
                    changes: {
                        baseGeneration: "9007199254740994"
                    }
                },
                {
                    tag: "section",
                    changes: {
                        section: "receiver"
                    }
                }
            ];
        }
        function test_pending_cancel_waits_for_exact_ack_under_host_pressure() {
            const token = openDraft();
            submitDraft();
            holdWholeHostPressure();
            const cancel = button("upstreamDraftCancel");
            verify(cancel.enabled);
            mouseClick(cancel);
            compare(engine.cancelledTokens, [token]);
            compare(popup().cancellationRequested, true);
            compare(popup().visible, true);
            engine.publishDraft({
                outcome: "cancelled",
                pending: false
            });
            verify(!host.poll());
            tryCompare(popup(), "visible", false);
            compare(engine.cancelledTokens, [token]);
            compare(host.draftState.pending, true);
        }
        function test_picker_cancel_preserves_open_draft_and_exact_values() {
            const token = openDraft();
            const oldValue = popup().valueFor(window.fixtureFieldKey);
            const load = findChild(popup().footer, "upstreamConfigurationLoad");
            verify(load.enabled);
            mouseClick(load);
            compare(engine.pickerRequests, 1);
            compare(engine.draftRequests, 1);
            compare(popup().visible, true);
            compare(popup().token, token);
            compare(popup().valueFor(window.fixtureFieldKey), oldValue);
            compare(host.reopenConfigurationIndex, host.noRequestedSection);
        }
        function test_rejected_or_stale_terminal(data) {
            openDraft();
            submitDraft();
            holdWholeHostPressure();
            const held = host.presentation;
            engine.publishDraft({
                outcome: data.outcome,
                pending: false,
                error: "Optional native diagnostic."
            });
            verify(!host.poll());
            compare(popup().visible, true);
            compare(popup().submitting, false);
            compare(popup().terminalReceived, false);
            compare(host.presentation, held);
            compare(popup().diagnostic, "");
        }
        function test_rejected_or_stale_terminal_data() {
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
        function test_successful_load_reopens_fresh_base_and_values() {
            const oldToken = openDraft();
            engine.pickerOutcome = "queued";
            mouseClick(findChild(popup().footer, "upstreamConfigurationLoad"));
            compare(engine.pickerRequests, 1);
            tryCompare(popup(), "visible", false);
            engine.settleImport("");
            verify(host.poll());
            tryCompare(popup(), "opened", true);
            tryCompare(popup(), "valuesLoaded", true);
            compare(engine.draftRequests, 2);
            verify(popup().token !== oldToken);
            compare(popup().baseGeneration, window.fixtureLoadedGeneration);
            compare(popup().valueFor(window.fixtureFieldKey), engine.acceptedValue);
            compare(host.reopenConfigurationIndex, host.noRequestedSection);
        }

        name: "UpstreamHostDraftLifecycle"
        when: windowShown
    }
}
