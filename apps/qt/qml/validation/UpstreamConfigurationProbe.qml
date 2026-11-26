/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

// Fixed opt-in Main acceptance consumer. The runner supplies a genuine native
// picker input and image destination; every configuration outcome comes from Rust.
QtObject {
    id: probe
    required property var rootWindow
    required property var workbench
    required property var tabs
    required property var host
    required property var controls
    required property var captureItem
    required property string screenshotPath
    signal finished(bool success, string message)

    // GUI scheduling/traversal policies, independent of all scientific limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseRetryLimit: 1200
    readonly property int overallTimeoutMs: 180000
    readonly property int captureSettleMs: 100
    readonly property int captureGraceMs: 2000
    readonly property int itemTraversalLimit: 8192
    readonly property string downstream: "downstream"
    readonly property string upstream: "upstream"
    // Explicit accepted/rejected fixture vectors; no scientific calculation.
    readonly property var cases: [
        {
            section: "ucd",
            field: "randomization_seed",
            value: "1",
            invalid: "4294967295"
        },
        {
            section: "profiles",
            field: "modulation_index",
            value: "2",
            invalid: "1"
        },
        {
            section: "grants",
            field: "grant_id",
            value: "9007199254740993",
            invalid: "99999999999999999999"
        },
        {
            section: "source",
            field: "source_seed",
            value: "2",
            invalid: "99999"
        },
        {
            section: "channel",
            field: "phase_radians",
            value: "0.1",
            invalid: "NaN"
        },
        {
            section: "preEqualization",
            field: "initial_id",
            value: "2",
            invalid: "0"
        },
        {
            section: "receiver",
            field: "ldpc_iterations",
            value: "2",
            invalid: "0"
        }
    ]
    property QtObject state: QtObject {
        property string phase: "loading"
        property int phaseRetries: 0
        property double phaseStartedAt: 0
        property double startedAt: 0
        property bool done: false
        property bool pickerReturned: false
        property bool pickerCancelled: false
        property bool configurationLoaded: false
        property bool tabIsolation: false
        property bool screenshotSaved: false
        property int caseIndex: 0
        property var results: []
        property var downstreamItem: null
        property var upstreamItem: null
        property string downstreamIdentity: ""
        property string acceptedOptions: ""
        property string initialToken: ""
        property string initialValues: ""
        property string actionToken: ""
        property string originalValue: ""
        property string generationBefore: ""
        property string identityBefore: ""
        property string identityAfter: ""
        property string invalidBaseSession: ""
        property string generationAfter: ""
        property string finalGeneration: ""
        property string finalUpstreamIdentity: ""
    }
    property Timer phaseTimer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }

    /**
     * Fail this fixed operation on the first missing actual invariant.
     * @param condition Observed native or QML condition.
     * @param message Original explanation included in terminal JSON.
     */
    function require(condition, message) {
        if (!condition) {
            throw new Error(message);
        }
    }

    /**
     * Move to one finite asynchronous phase without blocking the GUI thread.
     * @param phase Next compiled phase tag.
     */
    function next(phase) {
        state.phase = phase;
        state.phaseRetries = 0;
        state.phaseStartedAt = Date.now();
    }

    /**
     * Search actual public QML child lists with a finite traversal budget.
     * @param root Existing visual content or footer to inspect.
     * @param name Existing stable objectName.
     * @returns The real object or null; never a fixture replacement.
     */
    function findItem(root, name) {
        const pending = [root];
        const visited = [];
        while (pending.length) {
            const item = pending.pop();
            if (!item || visited.indexOf(item) >= 0) {
                continue;
            }
            require(visited.length < itemTraversalLimit, "Configuration probe object traversal exceeded its bound.");
            visited.push(item);
            if (item.objectName === name) {
                return item;
            }
            for (const key of ["children", "contentData"]) {
                const children = item[key];
                if (!children || typeof children.length !== "number") {
                    continue;
                }
                require(pending.length + children.length <= itemTraversalLimit, "Configuration probe pending traversal exceeded its bound.");
                for (const child of children) {
                    pending.push(child);
                }
            }
        }
        return null;
    }

    /**
     * Read a real upstream dialog by its existing navigation identifier.
     * @param section Compiled source/configuration section.
     * @returns Existing persistent popup, without creating an alternate owner.
     */
    function popupFor(section) {
        const popup = state.upstreamItem.sectionDialogs[section];
        require(!!popup, "Actual upstream section is unavailable: " + section);
        return popup;
    }

    /**
     * Wait for the actual admitted editor and exact native envelope.
     * @param popup Existing upstream configuration dialog.
     * @returns True after open, values/text admission and native identity match.
     */
    function ready(popup) {
        return popup.opened && popup.valuesLoaded && popup.textAdmitted && popup.matchingState && !popup.pending;
    }

    /**
     * Resolve a semantic field in its actual native descriptor order.
     * @param popup Existing dialog with admitted fields.
     * @param semantic Existing public field name; keys remain opaque.
     * @returns The first actual matching descriptor, including its original key.
     */
    function fieldFor(popup, semantic) {
        const field = popup.fields.find(value => value.field === semantic);
        require(!!field && typeof field.key === "string" && field.editable === true, "Native editable descriptor is unavailable: " + semantic);
        return field;
    }

    /**
     * Edit the real bounded TextField through its existing textEdited handler.
     * @param popup Actual admitted configuration dialog.
     * @param field Exact native descriptor, never reconstructed from a key.
     * @param value Fixed accepted/rejected text vector.
     */
    function edit(popup, field, value) {
        require(ready(popup) && popup.editable && popup.fieldAvailable(field), "Actual configuration editor is not admitted or editable.");
        require(field.kind === "integer" || field.kind === "decimal", "The fixed scalar vector requires a native text descriptor.");
        const editor = findItem(popup.contentItem, field.editorObjectName);
        require(editor && editor.enabled && typeof editor.textEdited === "function", "The actual configuration TextField is unavailable or disabled.");
        require(value.length <= editor.maximumLength, "The fixed input vector exceeds its actual text transport bound.");
        editor.text = value;
        editor.textEdited();
        require(editor.text === value && popup.valueFor(field.key) === value, "The real text handler did not preserve the exact changed string.");
        require(popup.canApply, "Native can_apply did not permit this changed draft.");
    }

    /**
     * Click an actual visible dialog footer action after checking its enabled state.
     * @param popup Existing dialog.
     * @param name Existing footer objectName.
     */
    function clickFooter(popup, name) {
        const button = findItem(popup.footer, name);
        require(popup.opened && button && button.enabled, "Actual configuration footer action is disabled: " + name);
        button.clicked();
    }

    /**
     * Check the real selected cursor and persistent children after bindings settle.
     * @param direction Compiled target direction.
     * @returns Whether every shared control observes the native selected identity.
     */
    function cursorCurrent(direction) {
        const shell = JSON.parse(host.backend.state());
        return shell.selectedDirection === direction && rootWindow.selectedDirection === direction && tabs.selectedDirection === direction && controls.direction === direction && controls.contextCurrent && controls.identity === tabs.identityFor(direction) && tabs.downstreamItem === state.downstreamItem && tabs.upstreamItem === state.upstreamItem;
    }

    /**
     * Preserve the independently accepted downstream owner without running science.
     * @returns Fresh downstream status for diagnostics only.
     */
    function checkDownstream() {
        const shell = JSON.parse(host.backend.state());
        const current = JSON.parse(workbench.runBackend.poll());
        require(shell.downstreamIdentity === state.downstreamIdentity, "Upstream configuration changed the independent downstream identity.");
        require(JSON.stringify(current.accepted_options) === state.acceptedOptions, "Upstream configuration changed downstream accepted_options.");
        require(!current.busy && !shell.applicationBusy, "Configuration acceptance unexpectedly started scientific execution.");
        return current;
    }

    /**
     * Request one real native picker after its external runner synchronization marker.
     * @param action Exactly cancel or load, interpreted only by the X11 runner.
     */
    function requestPicker(action) {
        const popup = popupFor("ucd");
        require(ready(popup), "The actual UCD dialog is not ready for its native picker.");
        const button = findItem(popup.footer, "upstreamConfigurationLoad");
        require(button && button.enabled, "Actual Load Configuration is disabled.");
        state.pickerReturned = false;
        next(action === "cancel" ? "picker-cancel-return" : "picker-load-return");
        console.log("MERLIN_UPSTREAM_PICKER " + action);
        Qt.callLater(function () {
            if (state.done) {
                return;
            }
            try {
                require(button.enabled, "Load Configuration became disabled before the picker.");
                button.clicked();
                state.pickerReturned = true;
            } catch (error) {
                complete(false, String(error));
            }
        });
    }

    /**
     * Finish once with finite per-section results and actual identity evidence.
     * @param success True only after all seven sections, rejection and picker checks.
     * @param message Failure explanation; empty only after complete success.
     */
    function complete(success, message) {
        if (state.done) {
            return;
        }
        state.done = true;
        phaseTimer.stop();
        finished(success, JSON.stringify({
            probe: "upstream-configuration",
            success: success,
            phase: state.phase,
            error: message,
            sections: state.results,
            chooserCancelAndLoad: state.pickerCancelled && state.configurationLoaded,
            invalidPreservation: state.results.length === cases.length && state.results.every(value => value.invalidPreserved),
            tabIsolation: state.tabIsolation,
            downstreamIdentity: state.downstreamIdentity,
            downstreamAcceptedOptionsPreserved: success,
            upstreamGeneration: state.finalGeneration || host.upstreamState.generation || null,
            upstreamIdentity: state.finalUpstreamIdentity || null,
            screenshot: state.screenshotSaved ? screenshotPath : null,
            captureTarget: "receiver-contentItem",
            fullWindowCaptureMarker: state.screenshotSaved,
            dark: !!workbench.theme && workbench.theme.dark === true
        }));
    }

    /**
     * Capture the explicitly QML-owned receiver ScrollView, avoiding Qt-owned
     * window content without a QML engine. The runner captures the actual Main
     * X11 window after the marker; a finite grace period keeps its popup visible.
     */
    function capture() {
        require(workbench.theme.dark === true, "Actual dark theme is required for evidence.");
        const target = popupFor("receiver").contentItem;
        require(target && target.width > 0 && target.height > 0 && screenshotPath.length > 0, "A real receiver frame and fresh screenshot path are required.");
        console.log("MERLIN_UPSTREAM_CAPTURE ready");
        next("capturing");
        const started = target.grabToImage(function (result) {
            if (state.done) {
                return;
            }
            try {
                checkDownstream();
                require(popupFor("receiver").opened, "The real accepted receiver dialog closed before capture.");
                require(result.saveToFile(screenshotPath), "Could not save the actual Main receiver image.");
                state.screenshotSaved = true;
                next("capture-grace");
            } catch (error) {
                complete(false, String(error));
            }
        });
        require(started, "The actual receiver frame rejected image capture.");
    }

    /**
     * Drive only real GUI configuration operations with per-phase/overall watchdogs.
     * No Start/Stop, source execution or scientific result is synthesized here.
     */
    function advance() {
        if (state.done) {
            return;
        }
        try {
            state.phaseRetries += 1;
            require(state.phaseRetries <= phaseRetryLimit, "Upstream configuration probe timed out in " + state.phase + ".");
            require(Date.now() - state.startedAt <= overallTimeoutMs, "Upstream configuration probe exceeded its overall watchdog.");
            if (!workbench || !tabs.downstreamItem || !tabs.upstreamItem) {
                return;
            }
            workbench.pollRun();
            host.poll();
            if (state.phase === "loading") {
                const current = JSON.parse(workbench.runBackend.poll());
                if (workbench.loadingProject || !workbench.hasProject || !current.accepted_options || !controls.contextCurrent) {
                    return;
                }
                require(rootWindow.visible && workbench.theme.dark === true, "Probe requires the visible dark Main with the genuine DS example loaded.");
                const shell = JSON.parse(host.backend.state());
                require(!current.busy && !shell.applicationBusy, "Probe requires an idle host and cannot stop unrelated work.");
                state.downstreamItem = tabs.downstreamItem;
                state.upstreamItem = tabs.upstreamItem;
                state.downstreamIdentity = shell.downstreamIdentity;
                state.acceptedOptions = JSON.stringify(current.accepted_options);
                require(host.selectDirection(upstream), "Native upstream selection was rejected.");
                next("unconfigured-open");
            } else {
                checkDownstream();
                if (state.phase === "unconfigured-open") {
                    if (!cursorCurrent(upstream)) {
                        return;
                    }
                    require(!host.upstreamState.configured, "Picker acceptance must begin with a genuinely unconfigured upstream owner.");
                    require(state.upstreamItem.openSection("ucd"), "Actual UCD open was rejected.");
                    next("unconfigured-ready");
                } else if (state.phase === "unconfigured-ready") {
                    const popup = popupFor("ucd");
                    if (!ready(popup)) {
                        return;
                    }
                    require(popup.fields.length === 0 && !popup.canApply && typeof popup.token === "string" && popup.token.length > 0, "Unconfigured UCD invented fields or omitted its real native draft token.");
                    state.initialToken = popup.token;
                    state.initialValues = JSON.stringify(popup.originalValues);
                    requestPicker("cancel");
                } else if (state.phase === "picker-cancel-return") {
                    if (!state.pickerReturned) {
                        return;
                    }
                    const popup = popupFor("ucd");
                    require(ready(popup) && popup.token === state.initialToken && JSON.stringify(popup.originalValues) === state.initialValues && !host.upstreamState.configured && !host.upstreamState.preparing, "Native picker Cancel changed the original visible draft.");
                    state.pickerCancelled = true;
                    requestPicker("load");
                } else if (state.phase === "picker-load-return") {
                    if (!state.pickerReturned || host.upstreamState.preparing) {
                        return;
                    }
                    const popup = popupFor("ucd");
                    if (!ready(popup)) {
                        return;
                    }
                    require(host.upstreamState.configured && popup.fields.length > 0 && popup.token !== state.initialToken && typeof host.upstreamState.generation === "string", "Native picker Load did not accept and reopen a fresh real source draft.");
                    state.configurationLoaded = true;
                    clickFooter(popup, "upstreamDraftCancel");
                    next("case-open");
                } else if (state.phase === "case-open") {
                    if (popupFor("ucd").visible) {
                        return;
                    }
                    const spec = cases[state.caseIndex];
                    require(state.upstreamItem.openSection(spec.section), "Actual configuration section open was rejected: " + spec.section);
                    next("cancel-ready");
                } else if (state.phase === "cancel-ready") {
                    const spec = cases[state.caseIndex];
                    const popup = popupFor(spec.section);
                    if (!ready(popup)) {
                        return;
                    }
                    const field = fieldFor(popup, spec.field);
                    state.originalValue = popup.valueFor(field.key);
                    require(typeof state.originalValue === "string" && state.originalValue !== spec.value, "The real source fixture does not supply the expected distinct scalar anchor.");
                    state.generationBefore = host.upstreamState.generation;
                    state.identityBefore = JSON.parse(host.backend.state()).upstreamIdentity;
                    state.actionToken = popup.token;
                    edit(popup, field, spec.value);
                    clickFooter(popup, "upstreamDraftCancel");
                    next("cancel-closed");
                } else if (state.phase === "cancel-closed") {
                    const spec = cases[state.caseIndex];
                    if (popupFor(spec.section).visible) {
                        return;
                    }
                    require(host.upstreamState.generation === state.generationBefore && JSON.parse(host.backend.state()).upstreamIdentity === state.identityBefore, "Cancelling local edits changed accepted upstream identity/generation.");
                    require(state.upstreamItem.openSection(spec.section), "Reopening the cancelled editor failed.");
                    next("apply-ready");
                } else if (state.phase === "apply-ready") {
                    const spec = cases[state.caseIndex];
                    const popup = popupFor(spec.section);
                    if (!ready(popup)) {
                        return;
                    }
                    const field = fieldFor(popup, spec.field);
                    require(popup.valueFor(field.key) === state.originalValue && popup.token !== state.actionToken, "Cancel did not preserve the real value and reopen a fresh draft.");
                    state.actionToken = popup.token;
                    edit(popup, field, spec.value);
                    clickFooter(popup, "upstreamDraftApply");
                    next("apply-accepted");
                } else if (state.phase === "apply-accepted") {
                    const spec = cases[state.caseIndex];
                    if (host.upstreamState.preparing || popupFor(spec.section).visible) {
                        return;
                    }
                    require(host.draftState.token === state.actionToken && host.draftState.outcome === "accepted" && typeof host.upstreamState.generation === "string" && host.upstreamState.generation !== state.generationBefore, "Queued Apply did not produce genuine correlated native adoption.");
                    state.generationAfter = host.upstreamState.generation;
                    state.identityAfter = JSON.parse(host.backend.state()).upstreamIdentity;
                    require(state.upstreamItem.openSection(spec.section), "Reopening accepted values failed.");
                    next("invalid-ready");
                } else if (state.phase === "invalid-ready") {
                    const spec = cases[state.caseIndex];
                    const popup = popupFor(spec.section);
                    if (!ready(popup)) {
                        return;
                    }
                    const field = fieldFor(popup, spec.field);
                    require(popup.valueFor(field.key) === spec.value, "The reopened editor does not show its actual accepted scalar value.");
                    state.actionToken = popup.token;
                    state.invalidBaseSession = popup.baseSession;
                    require(popup.baseGeneration === state.generationAfter, "The real invalid draft does not retain its accepted generation.");
                    edit(popup, field, spec.invalid);
                    clickFooter(popup, "upstreamDraftApply");
                    next("invalid-rejected");
                } else if (state.phase === "invalid-rejected") {
                    const spec = cases[state.caseIndex];
                    const popup = popupFor(spec.section);
                    if (host.upstreamState.preparing || popup.pending) {
                        return;
                    }
                    require(popup.opened && host.draftState.token === state.actionToken && host.draftState.outcome === "rejected" && host.upstreamState.generation === state.generationAfter && popup.baseSession === state.invalidBaseSession && popup.baseGeneration === state.generationAfter && JSON.parse(host.backend.state()).upstreamIdentity === state.identityAfter, "Invalid native Apply changed accepted state/base or dismissed its draft.");
                    require(typeof host.draftState.error === "string" && host.draftState.error.length > 0 && popup.diagnostic === host.draftState.error, "Native rejection diagnostic was lost or replaced in the actual dialog.");
                    clickFooter(popup, "upstreamDraftCancel");
                    next("invalid-closed");
                } else if (state.phase === "invalid-closed") {
                    const spec = cases[state.caseIndex];
                    if (popupFor(spec.section).visible) {
                        return;
                    }
                    require(host.upstreamState.generation === state.generationAfter, "Closing a rejected draft changed the accepted generation.");
                    state.results = state.results.concat([
                        {
                            section: spec.section,
                            field: spec.field,
                            cancelPreserved: true,
                            applied: true,
                            acceptedValue: spec.value,
                            invalidPreserved: true,
                            originalDiagnosticPreserved: true,
                            generationBefore: state.generationBefore,
                            generationAfter: state.generationAfter
                        }
                    ]);
                    state.caseIndex += 1;
                    if (state.caseIndex < cases.length) {
                        next("case-open");
                    } else {
                        state.finalGeneration = host.upstreamState.generation;
                        state.finalUpstreamIdentity = JSON.parse(host.backend.state()).upstreamIdentity;
                        require(host.selectDirection(downstream), "Returning to actual downstream was rejected.");
                        next("downstream-tab");
                    }
                } else if (state.phase === "downstream-tab") {
                    if (!cursorCurrent(downstream)) {
                        return;
                    }
                    require(host.upstreamState.generation === state.finalGeneration && JSON.parse(host.backend.state()).upstreamIdentity === state.finalUpstreamIdentity, "Returning downstream changed accepted upstream identity/generation.");
                    require(host.selectDirection(upstream), "Returning to actual upstream was rejected.");
                    next("upstream-tab");
                } else if (state.phase === "upstream-tab") {
                    if (!cursorCurrent(upstream)) {
                        return;
                    }
                    require(host.upstreamState.generation === state.finalGeneration && JSON.parse(host.backend.state()).upstreamIdentity === state.finalUpstreamIdentity, "Tab round trip changed accepted upstream identity/generation.");
                    state.tabIsolation = true;
                    require(state.upstreamItem.openSection("receiver"), "Final actual receiver open failed.");
                    next("capture-ready");
                } else if (state.phase === "capture-ready") {
                    const popup = popupFor("receiver");
                    if (!ready(popup) || Date.now() - state.phaseStartedAt < captureSettleMs) {
                        return;
                    }
                    require(popup.valueFor(fieldFor(popup, "ldpc_iterations").key) === "2", "Final receiver evidence does not show the genuinely accepted setting.");
                    require(state.results.length === cases.length && state.pickerCancelled && state.configurationLoaded && state.tabIsolation, "Native configuration probe omitted a required acceptance phase.");
                    capture();
                } else if (state.phase === "capture-grace") {
                    require(popupFor("receiver").opened && host.upstreamState.generation === state.finalGeneration, "Accepted receiver changed during the external capture grace period.");
                    if (Date.now() - state.phaseStartedAt >= captureGraceMs) {
                        complete(true, "");
                    }
                } else if (state.phase !== "capturing") {
                    throw new Error("Unknown upstream configuration probe phase.");
                }
            }
        } catch (error) {
            complete(false, String(error));
        }
    }

    Component.onCompleted: {
        state.startedAt = Date.now();
        state.phaseStartedAt = state.startedAt;
        phaseTimer.start();
    }
    Component.onDestruction: phaseTimer.stop()
}
