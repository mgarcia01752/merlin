/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Window

// Fixed opt-in Main consumer. Native Rust supplies every decoder/LLR/verification
// value; this probe uses real picker, parent-popup and run-control connections.
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

    // Scheduling, traversal and receipt populations; never scientific limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseRetryLimit: 4800
    readonly property int overallTimeoutMs: 360000
    readonly property int itemTraversalLimit: 8192
    readonly property int maximumRecords: 48
    readonly property int renderSettleMs: 100
    readonly property int captureGraceMs: 2000
    readonly property int minimumCompletedGrantWitness: 1
    readonly property string upstream: "upstream"
    readonly property string downstream: "downstream"
    readonly property var cases: ["fec-50", "fec-25", "fec-cancel"]
    property QtObject state: QtObject {
        id: observations
        property string phase: "loading"
        property int retries: 0
        property double startedAt: 0
        property double phaseAt: 0
        property bool done: false
        property bool pickerReturned: false
        property bool startRequested: false
        property bool pauseAcknowledged: false
        property bool cancellationLeaseChecked: false
        property int caseIndex: 0
        property var downstreamItem: null
        property var upstreamItem: null
        property var popup: null
        property string downstreamIdentity: ""
        property string downstreamOptions: ""
        property string downstreamScience: ""
        property string identity: ""
        property string key: ""
        property string run: ""
        property string previousRun: ""
        property string report: ""
        property string terminalProgress: ""
        property int requestCount: 0
        property var request: null
        property var requestContext: null
        property string previousSequence: ""
        property string queryAction: ""
        property string afterQuery: ""
        property string firstCells: ""
        property string firstWordInput: ""
        property string firstGrantIdentity: ""
        property string firstRetainedWord: ""
        property var reports: []
        property var records: []
        property var verifications: []
        property var captures: []
        property string captureAfter: ""
    }
    property Timer timer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }
    property Connections requests: Connections {
        target: observations.popup
        ignoreUnknownSignals: true
        function onInstrumentRequested(kind, selection, context) {
            if (kind !== "fec") {
                probe.complete(false, "Actual FEC popup emitted a foreign destination.");
                return;
            }
            observations.requestCount++;
            observations.request = selection;
            observations.requestContext = context;
        }
    }

    /** Reject absent native invariants instead of substituting scientific values. */
    function require(condition, message) {
        if (!condition)
            throw new Error(message);
    }
    /** Enter the next finite asynchronous native/visual phase. */
    function next(phase) {
        observations.phase = phase;
        observations.retries = 0;
        observations.phaseAt = Date.now();
    }
    /** Locate only actual bounded child/content lists, including real popup content. */
    function findItem(root, name) {
        const pending = [root], visited = [];
        while (pending.length) {
            const item = pending.pop();
            if (!item || visited.indexOf(item) >= 0)
                continue;
            require(visited.length < itemTraversalLimit, "FEC object traversal exceeded its bound.");
            visited.push(item);
            if (item.objectName === name)
                return item;
            for (const key of ["children", "contentData", "data"]) {
                const children = item[key];
                if (!children || typeof children.length !== "number")
                    continue;
                require(pending.length + children.length <= itemTraversalLimit, "FEC pending traversal exceeded its bound.");
                for (const child of children)
                    pending.push(child);
            }
        }
        return null;
    }
    /** Activate an existing enabled control through its original signal handler. */
    function click(root, name) {
        const action = findItem(root, name);
        require(action && action.enabled, "Actual FEC action is absent/disabled: " + name);
        action.clicked();
    }
    /** Count real shared drawing children without introducing a performance claim. */
    function visualCounts(root) {
        const pending = [root];
        let items = 0, canvases = 0;
        while (pending.length) {
            const item = pending.pop();
            require(++items <= itemTraversalLimit, "FEC visual traversal exceeded its bound.");
            if (typeof item.getContext === "function" && typeof item.requestPaint === "function")
                canvases++;
            for (const child of item.children || [])
                pending.push(child);
        }
        return {items: items, canvases: canvases};
    }
    /** Read exact application identity independently from backend report generation. */
    function keyText() {
        return JSON.stringify({session: host.upstreamState.session, generation: host.upstreamState.generation});
    }
    /** Exclude only live display age while preserving exact scientific DS outcomes. */
    function dsScience(value) {
        return JSON.stringify({has_result: value.has_result, result: value.result,
            result_token: value.result_token, completed_symbols: value.completed_symbols});
    }
    /** Preserve both real workbenches and the completed report during every passive query. */
    function checkIndependent() {
        const shell = JSON.parse(host.backend.state());
        const ds = JSON.parse(workbench.runBackend.poll());
        require(shell.downstreamIdentity === observations.downstreamIdentity && !ds.busy && JSON.stringify(ds.accepted_options) === observations.downstreamOptions && dsScience(ds) === observations.downstreamScience,
            "Upstream FEC navigation changed/executed independent downstream science.");
        require(tabs.downstreamItem === observations.downstreamItem && tabs.upstreamItem === observations.upstreamItem,
            "FEC navigation recreated a persistent direction workbench.");
        if (observations.key.length)
            require(shell.upstreamIdentity === observations.identity && keyText() === observations.key, "Passive FEC work changed its accepted upstream identity.");
        if (observations.report.length) {
            const current = host.upstreamState;
            require(!shell.applicationBusy && JSON.stringify({has_result: current.has_result, result: current.result}) === observations.report,
                "Passive FEC/LLR inspection replaced or advanced its actual terminal report.");
            if (!current.preparing)
                require(JSON.stringify(current.progress) === observations.terminalProgress, "Passive FEC inspection advanced source/sample progress.");
        }
        return shell;
    }
    /** Wait for the selected native cursor and existing shared controls to agree. */
    function cursorCurrent() {
        const shell = JSON.parse(host.backend.state());
        return shell.selectedDirection === upstream && rootWindow.selectedDirection === upstream && tabs.selectedDirection === upstream && controls.direction === upstream && controls.contextCurrent && controls.identity === tabs.identityFor(upstream);
    }
    /** Return an actual existing parent dialog rather than constructing a detached view. */
    function section(name) {
        const popup = observations.upstreamItem.sectionDialogs[name];
        require(!!popup, "Actual upstream section is unavailable: " + name);
        return popup;
    }
    /** Require real native draft/text ownership before using its acquisition/footer actions. */
    function draftReady(popup) {
        return popup.opened && popup.valuesLoaded && popup.textAdmitted && popup.matchingState && !popup.pending;
    }
    /** Synchronize the external picker runner before invoking the real native chooser. */
    function load() {
        const popup = section("ucd"), action = findItem(popup.footer, "upstreamConfigurationLoad");
        require(draftReady(popup) && action && action.enabled, "Actual configuration picker is unavailable.");
        observations.pickerReturned = false;
        next("loaded");
        console.log("MERLIN_UPSTREAM_FEC_PICKER " + cases[observations.caseIndex]);
        Qt.callLater(function () {
            if (observations.done)
                return;
            try {
                require(action.enabled, "Actual configuration Load became disabled.");
                action.clicked();
                observations.pickerReturned = true;
            } catch (error) {
                complete(false, String(error));
            }
        });
    }
    /** Locate the actual FEC dialog only after its real parent action has dispatched. */
    function attachPopup() {
        observations.popup = findItem(observations.upstreamItem, "upstreamFecDialog");
        require(!!observations.popup, "Actual upstreamFecDialog was not registered on the workbench.");
    }
    /** Wait for an exact admitted native reply, not optimistic request acceptance. */
    function ready() {
        const value = host.instrumentState, popup = observations.popup;
        if (!(popup && popup.opened && popup.contentItem.Window.window === rootWindow && !popup.presentationRejected && popup.contextCurrent && popup.actionsEnabled && value && value.status === "ready" && value.snapshot && value.snapshot.kind === "fec" && value.snapshot.status === "ready" && popup.envelope === value && !host.upstreamState.preparing))
            return false;
        const table = findItem(popup.contentItem, "upstreamFecTable");
        const list = findItem(popup.contentItem, "upstreamFecRows");
        return table && list && table.rows === value.snapshot.table.rows && table.headings === value.snapshot.table.headings && list.count === value.snapshot.table.rows.length;
    }
    /** Prepare correlation before one real selector or paging action. */
    function beginQuery(action, after) {
        require(ready(), "Cannot dispatch from an unaccepted FEC view.");
        observations.previousSequence = host.instrumentState.query_sequence;
        observations.request = null;
        observations.requestContext = null;
        observations.queryAction = action;
        observations.afterQuery = after;
    }
    /** Require exactly one native signal before waiting for its genuine reply. */
    function dispatched(before) {
        require(observations.requestCount === before + 1 && observations.request && observations.requestContext,
            "Actual FEC action emitted no single correlated request.");
        next("query");
    }
    /** Change a supplied native choice through the actual ComboBox activated handler. */
    function select(name, value, action, after) {
        beginQuery(action, after);
        const selector = findItem(observations.popup.contentItem, name);
        require(selector && selector.enabled, "Actual FEC selector is unavailable: " + name);
        const index = selector.model.findIndex(choice => choice.value === value);
        require(index >= 0, "Requested choice is absent from native inventory: " + name);
        const before = observations.requestCount;
        selector.currentIndex = index;
        selector.activated(index);
        dispatched(before);
    }
    /** Request supplied page coordinates through the actual navigation button. */
    function page(name, action, after) {
        beginQuery(action, after);
        const before = observations.requestCount;
        click(observations.popup.contentItem, name);
        dispatched(before);
    }
    /** Compare complete scalar selectors without relying on JSON object ordering. */
    function sameSelection(left, right) {
        const keys = Object.keys(left).sort();
        return keys.join(",") === Object.keys(right).sort().join(",") && keys.every(key => left[key] === right[key]);
    }
    /** Verify all four original native context fields and exact echoed selection. */
    function acknowledged() {
        if (!ready() || host.instrumentState.query_sequence === observations.previousSequence)
            return false;
        const value = host.instrumentState, context = observations.requestContext;
        require(context && value.session_token === context.session_token && value.configuration_generation === context.configuration_generation && value.run === context.run && value.run === observations.run,
            "Actual FEC reply lost its original session/generation/run context.");
        require(sameSelection(value.snapshot.selection, observations.request), "FEC reply did not acknowledge its exact original selector.");
        return true;
    }
    /** Record original typed-native data and the real shared drawing aliases. */
    function record(action) {
        require(ready(), "Cannot record unavailable actual FEC observations.");
        const envelope = host.instrumentState, snapshot = envelope.snapshot;
        const table = findItem(observations.popup.contentItem, "upstreamFecTable");
        const list = findItem(observations.popup.contentItem, "upstreamFecRows");
        require(table && list && table.visible && table.rows === snapshot.table.rows && table.headings === snapshot.table.headings && list.count === snapshot.table.rows.length,
            "Actual shared upstream table lost its original admitted rows/headings.");
        require(snapshot.table.headings.every(text => text.indexOf("BCH") < 0 && text.indexOf("Bit Errors") < 0),
            "Upstream LDPC view acquired a BCH stage or invented per-word source errors.");
        const verification = snapshot.metadata.verification;
        require(verification && verification.status === host.upstreamState.result.status && verification.compared_source_bits === host.upstreamState.result.bits_tested && verification.bit_errors === host.upstreamState.result.bit_errors && verification.ber === host.upstreamState.result.ber && verification.reference_recovered === host.upstreamState.result.reference_recovered,
            "FEC metadata disagrees with actual independent source verification.");
        let chartRecord = null;
        if (snapshot.selection.mode === "words") {
            const chart = findItem(observations.popup.contentItem, "upstreamFecIterations");
            require(chart && chart.visible && chart.chart === snapshot.chart && visualCounts(chart).canvases === 1,
                "Actual shared iteration count-bar renderer lost its native chart alias.");
            require(chart.zeroMessage.indexOf("Zero errors") < 0 && chart.categoryAxisLabel.indexOf("word") >= 0,
                "Upstream iteration drawing inherited downstream error/payload semantics.");
            require(snapshot.data.words.length > 0, "No actual completed decoder word was displayed.");
            require(snapshot.data.words.every(word => typeof word.identity.grant === "string" && typeof word.source_start_bit === "string" && typeof word.iterations === "number" && typeof word.converged === "boolean" && typeof word.parity_valid === "boolean"),
                "Original upstream decoder identities/flags are missing or lossy.");
            chartRecord = {chart: chart.chart, chartAlias: chart.chart === snapshot.chart,
                qmlType: String(chart), heading: chart.heading, categoryAxisLabel: chart.categoryAxisLabel,
                zeroMessage: chart.zeroMessage, visual: visualCounts(chart)};
        } else {
            require(snapshot.metadata.llr_convention === "natural_log_p0_over_p1;positive_favors_zero",
                "Actual soft input lost its natural-log sign convention.");
            require(snapshot.table.rows.length > 0, "Actual retained soft observations are empty.");
            if (snapshot.selection.mode === "soft_word")
                require(snapshot.data.first_bit === snapshot.page.first && snapshot.data.input_llrs.length === snapshot.table.rows.length,
                    "Actual word-input viewport lost original bit positions or values.");
        }
        require(observations.records.length < maximumRecords, "FEC evidence exceeded its fixed receipt population.");
        observations.records = observations.records.concat([{tag: cases[observations.caseIndex], action: action,
            instrument_state: envelope, request: observations.request, context: observations.requestContext,
            table: {headings: table.headings, rows: table.rows, listCount: list.count,
                rowsAlias: table.rows === snapshot.table.rows, headingsAlias: table.headings === snapshot.table.headings,
                qmlType: String(table), visual: visualCounts(table)}, chart: chartRecord}]);
        checkIndependent();
    }
    /** Capture a real QML-owned popup and leave a finite full-window X11 grace. */
    function capture(mode, after) {
        require(ready() && workbench.theme.dark === true && screenshotPath.length > 0, "Actual dark FEC/LLR capture is unavailable.");
        const target = mode === "fec" ? screenshotPath : screenshotPath + ".llr.png";
        observations.captureAfter = after;
        next("capturing");
        console.log(mode === "fec" ? "MERLIN_UPSTREAM_FEC_CAPTURE ready" : "MERLIN_UPSTREAM_LLR_CAPTURE ready");
        const started = observations.popup.contentItem.grabToImage(function (result) {
            if (observations.done)
                return;
            try {
                require(ready() && result.saveToFile(target), "Could not save actual " + mode + " content image.");
                observations.captures = observations.captures.concat([{kind: mode, path: target, state: host.instrumentState}]);
                next("capture-grace");
            } catch (error) {
                complete(false, String(error));
            }
        });
        require(started, "Actual FEC/LLR popup rejected QML image capture.");
    }
    /** Finish once; a failure may stop only this probe's own actual upstream run. */
    function complete(success, message) {
        if (observations.done)
            return;
        observations.done = true;
        timer.stop();
        if (!success && observations.startRequested && observations.key.length) {
            const shell = JSON.parse(host.backend.state()), current = host.upstreamState;
            if (shell.upstreamIdentity === observations.identity && (current.preparing || (current.progress && current.progress.run === observations.run && shell.executionDirection === upstream)))
                host.upstreamBackend.stop();
        }
        const result = JSON.stringify({probe: "upstream-fec", success: success, phase: observations.phase,
            error: message, pickerTag: cases[Math.min(observations.caseIndex, cases.length - 1)], reports: observations.reports,
            records: observations.records, verification: observations.verifications, captures: observations.captures,
            actualPauseAcknowledged: observations.pauseAcknowledged, cancellationLeaseChecked: observations.cancellationLeaseChecked,
            downstreamIdentity: observations.downstreamIdentity, downstreamSciencePreserved: success,
            passiveSourcePreserved: success, dark: !!workbench.theme && workbench.theme.dark === true,
            originalContext: {session_token: host.instrumentState.session_token, run: host.instrumentState.run,
                configuration_generation: host.instrumentState.configuration_generation, query_sequence: host.instrumentState.query_sequence}});
        console.log("MERLIN_UPSTREAM_FEC_PROBE " + result);
        finished(success, result);
    }

    /** Drive three fixed real configurations, bounded queries and genuine cancellation. */
    function advance() {
        if (observations.done)
            return;
        try {
            const now = Date.now();
            require(++observations.retries <= phaseRetryLimit, "FEC probe timed out in " + observations.phase + ".");
            require(now - observations.startedAt <= overallTimeoutMs, "FEC probe exceeded its overall watchdog.");
            if (!workbench || !tabs.downstreamItem || !tabs.upstreamItem)
                return;
            workbench.pollRun();
            host.poll();
            if (observations.phase === "loading") {
                const ds = JSON.parse(workbench.runBackend.poll()), shell = JSON.parse(host.backend.state());
                if (workbench.loadingProject || !workbench.hasProject || !ds.accepted_options || !controls.contextCurrent)
                    return;
                require(rootWindow.visible && workbench.theme.dark === true && !ds.busy && !shell.applicationBusy, "FEC probe requires actual idle dark Main and its original DS workbench.");
                observations.downstreamItem = tabs.downstreamItem;
                observations.upstreamItem = tabs.upstreamItem;
                observations.downstreamIdentity = shell.downstreamIdentity;
                observations.downstreamOptions = JSON.stringify(ds.accepted_options);
                observations.downstreamScience = dsScience(ds);
                require(host.selectDirection(upstream), "Native upstream tab selection was rejected.");
                next("load-open");
                return;
            }
            const shell = checkIndependent(), current = host.upstreamState;
            if (observations.startRequested && current.progress && current.progress.run !== observations.previousRun)
                observations.run = current.progress.run;
            if (observations.phase === "load-open") {
                if (!cursorCurrent() || (observations.popup && observations.popup.visible))
                    return;
                require(observations.upstreamItem.openSection("ucd"), "Actual UCD block navigation was rejected.");
                next("load-ready");
            } else if (observations.phase === "load-ready") {
                if (draftReady(section("ucd")))
                    load();
            } else if (observations.phase === "loaded") {
                if (!observations.pickerReturned || current.preparing || !draftReady(section("ucd")))
                    return;
                require(current.configured && !current.error, "Native FEC configuration load failed: " + (current.error || ""));
                observations.identity = shell.upstreamIdentity;
                observations.key = keyText();
                observations.previousRun = current.result ? current.result.run : "";
                click(section("ucd").footer, "upstreamDraftCancel");
                next("start");
            } else if (observations.phase === "start") {
                if (section("ucd").visible || !cursorCurrent() || !controls.canStart)
                    return;
                observations.startRequested = true;
                observations.run = "";
                click(controls, "directionStart");
                next(observations.caseIndex === cases.length - 1 ? "cancel-running" : "terminal");
            } else if (observations.phase === "cancel-running") {
                require(!current.error, "Actual cancellation fixture rejected Start: " + (current.error || ""));
                if (!current.progress || current.progress.run === observations.previousRun || current.progress.status !== "Running" || current.progress.completed_grants < minimumCompletedGrantWitness) {
                    require(current.busy || current.preparing || !current.has_result, "Cancellation fixture ended before an actual completed-grant pause.");
                    return;
                }
                observations.run = current.progress.run;
                require(shell.applicationBusy && shell.executionDirection === upstream, "Actual upstream run lacks its genuine global lease.");
                click(controls, "directionPause");
                next("cancel-paused");
            } else if (observations.phase === "cancel-paused") {
                if (!current.progress || current.progress.status !== "Paused" || !controls.paused)
                    return;
                require(current.busy && shell.applicationBusy && shell.executionDirection === upstream && current.progress.run === observations.run && current.progress.completed_grants >= minimumCompletedGrantWitness,
                    "Actual acknowledged pause lost its original completed prefix/lease.");
                observations.pauseAcknowledged = true;
                click(controls, "directionStop");
                next("terminal");
            } else if (observations.phase === "terminal") {
                if (current.progress && current.progress.run !== observations.previousRun)
                    observations.run = current.progress.run;
                if (current.busy || current.preparing || shell.applicationBusy) {
                    if (observations.caseIndex === cases.length - 1 && observations.pauseAcknowledged) {
                        require(shell.applicationBusy && shell.executionDirection === upstream && !findItem(controls, "directionStart").enabled,
                            "Cancellation released/redirected its lease before the actual terminal report.");
                        observations.cancellationLeaseChecked = true;
                    }
                    return;
                }
                const cancelled = observations.caseIndex === cases.length - 1;
                require(current.has_result && current.result && current.result.run !== observations.previousRun && current.result.status === (cancelled ? "Cancelled" : "Completed"),
                    "Real FEC run has no matching completed/cancelled terminal outcome: " + JSON.stringify(current.result));
                require(current.result.completed_grants >= (cancelled ? minimumCompletedGrantWitness : 2), "FEC fixture did not retain its required actual started/completed grants.");
                if (cancelled) {
                    require(observations.pauseAcknowledged && current.result.reference_recovered === false, "Cancelled coverage was mislabeled full reference recovery.");
                    observations.cancellationLeaseChecked = true;
                }
                observations.run = current.result.run;
                observations.startRequested = false;
                observations.report = JSON.stringify({has_result: current.has_result, result: current.result});
                observations.terminalProgress = JSON.stringify(current.progress);
                observations.reports = observations.reports.concat([{tag: cases[observations.caseIndex], identity: observations.identity,
                    key: observations.key, result: current.result, progress: current.progress,
                    execution_view: current.execution_view, shell: shell}]);
                require(observations.upstreamItem.openSection("receiver"), "Actual upstream receiver block was rejected.");
                next("receiver-ready");
            } else if (observations.phase === "receiver-ready") {
                if (!draftReady(section("receiver")))
                    return;
                click(section("receiver").footer, "upstreamFecInspect");
                attachPopup();
                next("words-initial");
            } else if (observations.phase === "words-initial") {
                if (!ready() || section("receiver").visible)
                    return;
                observations.request = null;
                observations.requestContext = null;
                record("words-initial");
                observations.firstGrantIdentity = host.instrumentState.snapshot.data.identity.grant;
                if (observations.caseIndex < cases.length - 1) {
                    const grants = observations.popup.choices.grants;
                    require(grants.length >= 2, "Native fixture lacks its second actual started grant.");
                    select("upstreamFecGrant", grants[1].value, "second-grant", "words-capture");
                } else {
                    const snapshot = host.instrumentState.snapshot;
                    require(snapshot.metadata.fec_summary.pending_words > 0 && snapshot.metadata.verification.reference_recovered === false,
                        "Actual cancelled FEC page erased pending coverage or claimed full recovery.");
                    next("cells-mode");
                }
            } else if (observations.phase === "query") {
                if (!acknowledged())
                    return;
                record(observations.queryAction);
                next(observations.afterQuery);
            } else if (observations.phase === "words-capture") {
                if (now - observations.phaseAt < renderSettleMs)
                    return;
                require(host.instrumentState.snapshot.data.identity.grant !== observations.firstGrantIdentity,
                    "Second started-grant selection repeated the original grant identity.");
                if (observations.caseIndex === 1)
                    capture("fec", "cells-mode");
                else
                    next("cells-mode");
            } else if (observations.phase === "cells-mode") {
                select("upstreamFecMode", "soft_cells", "soft-cells-initial", "cells-next");
            } else if (observations.phase === "cells-next") {
                observations.firstCells = JSON.stringify({data: observations.popup.snapshot.data, table: observations.popup.snapshot.table});
                require(Number.isInteger(observations.popup.page.next), "Actual retained cell inventory lacks its continuation.");
                page("upstreamFecNext", "soft-cells-next", "cells-previous");
            } else if (observations.phase === "cells-previous") {
                page("upstreamFecPrevious", "soft-cells-previous", "cells-restored");
            } else if (observations.phase === "cells-restored") {
                require(JSON.stringify({data: observations.popup.snapshot.data, table: observations.popup.snapshot.table}) === observations.firstCells,
                    "Cell Previous did not restore exact original cell/bit values.");
                select("upstreamFecMode", "soft_word", "soft-word-initial", "word-next");
            } else if (observations.phase === "word-next") {
                observations.firstWordInput = JSON.stringify({data: observations.popup.snapshot.data, table: observations.popup.snapshot.table});
                observations.firstRetainedWord = JSON.stringify(observations.popup.snapshot.metadata.word);
                require(Number.isInteger(observations.popup.page.next), "Actual completed LDPC input lacks its bit continuation.");
                page("upstreamFecNext", "soft-word-next", "word-first");
            } else if (observations.phase === "word-first") {
                page("upstreamFecFirst", "soft-word-first", "word-restored");
            } else if (observations.phase === "word-restored") {
                require(JSON.stringify({data: observations.popup.snapshot.data, table: observations.popup.snapshot.table}) === observations.firstWordInput,
                    "Word-input First did not restore exact original bit indices/LLRs.");
                if (observations.caseIndex < cases.length - 1) {
                    const words = observations.popup.choices.soft_words;
                    require(words.length >= 2, "Native completed fixtures lack a second actual retained word input.");
                    select("upstreamFecSoftWord", words[1].value, "second-retained-word", "word-second");
                } else
                    next("llr-capture");
            } else if (observations.phase === "word-second") {
                require(JSON.stringify(observations.popup.snapshot.metadata.word) !== observations.firstRetainedWord,
                    "Second retained-word selector repeated the first original word input.");
                next("llr-capture");
            } else if (observations.phase === "llr-capture") {
                if (now - observations.phaseAt < renderSettleMs)
                    return;
                if (observations.caseIndex === 1)
                    capture("llr", "verification-close");
                else
                    next("verification-close");
            } else if (observations.phase === "capture-grace") {
                require(ready(), "Actual FEC/LLR owner disappeared during X11 capture grace.");
                if (now - observations.phaseAt >= captureGraceMs)
                    next(observations.captureAfter);
            } else if (observations.phase === "verification-close") {
                click(observations.popup.header, "popupCloseButton");
                next("verification-open");
            } else if (observations.phase === "verification-open") {
                if (observations.popup.visible)
                    return;
                require(observations.upstreamItem.openSection("verification"), "Actual upstream PRBS-RX navigation was rejected.");
                next("verification-ready");
            } else if (observations.phase === "verification-ready") {
                const popup = section("verification"), view = current.execution_view.verification;
                if (!popup.opened || popup.presentationRejected || Date.now() - observations.phaseAt < renderSettleMs)
                    return;
                require(popup.displayedPresentation.summary === view.summary && JSON.stringify(popup.displayedRows) === JSON.stringify(view.rows.map(row => row.label + ": " + row.value)),
                    "Actual upstream PRBS-RX dialog lost its original verification rows/summary.");
                require(view.rows.some(row => row.label.toLowerCase().indexOf("seed") >= 0), "Actual PRBS-RX view omitted its original source seed.");
                observations.verifications = observations.verifications.concat([{tag: cases[observations.caseIndex], view: view,
                    displayedPresentation: popup.displayedPresentation, displayedRows: popup.displayedRows,
                    result: current.result}]);
                observations.request = null;
                observations.requestContext = null;
                click(popup.contentItem, "upstreamDetailFecInspect");
                attachPopup();
                next("verification-fec");
            } else if (observations.phase === "verification-fec") {
                if (!ready() || section("verification").visible)
                    return;
                require(observations.popup.selection.mode === "words", "PRBS-RX FEC link did not reopen original completed decoder outcomes.");
                record("verification-route-words");
                click(observations.popup.header, "popupCloseButton");
                next("case-done");
            } else if (observations.phase === "case-done") {
                if (observations.popup.visible)
                    return;
                if (++observations.caseIndex < cases.length) {
                    observations.key = "";
                    observations.report = "";
                    observations.terminalProgress = "";
                    observations.request = null;
                    observations.requestContext = null;
                    next("load-open");
                } else {
                    require(observations.reports.length === cases.length && observations.verifications.length === cases.length && observations.captures.length === 2 && observations.pauseAcknowledged && observations.cancellationLeaseChecked,
                        "Actual FEC/LLR/partial verification/capture evidence is incomplete.");
                    complete(true, "");
                }
            } else if (observations.phase !== "capturing") {
                throw new Error("Unknown upstream FEC probe phase: " + observations.phase);
            }
        } catch (error) {
            complete(false, String(error));
        }
    }
    Component.onCompleted: {
        observations.startedAt = Date.now();
        observations.phaseAt = observations.startedAt;
        timer.start();
    }
    Component.onDestruction: timer.stop()
}
