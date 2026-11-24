/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import "../../shared/presentation"

// UI routing only. Native direction adapters own all scientific operations.
QtObject {
    id: host
    required property var theme
    required property var backend
    required property var downstreamBackend
    required property var upstreamBackend
    property var downstreamView: null
    property var upstreamView: null
    readonly property var presentation: coverage.value || ({})
    readonly property var upstreamState: presentation.upstreamState || ({})
    readonly property var draftState: upstreamState.draft || ({})
    readonly property var resourceGridState: upstreamState.resource_grid_view || ({})
    readonly property var instrumentState: upstreamState.instrument_view || ({})
    // One scalar cursor over compiled section tags, never retained source text.
    readonly property int noRequestedSection: -1
    property int reopenConfigurationIndex: noRequestedSection
    readonly property bool presentationRejected: coverage.rejected
    property PresentationOwner presentationCoverage: PresentationOwner {
        id: coverage
        theme: host.theme
        initialValue: host.candidate(JSON.parse(host.backend.state()),
            JSON.parse(host.upstreamBackend.state()), "", false)
    }

    /** Couple native scalar metadata with its derived display strings before retention. */
    function candidate(metadata, upstream, message, isError) {
        // A native query sequence changes with actual view requests. Preserve the
        // admitted envelope between polls so inspected/hovered cells stay visible.
        for (const name of ["resource_grid_view", "instrument_view"]) {
            const previous = upstreamState[name]
            const next = upstream[name]
            if (previous && next && previous.query_sequence === next.query_sequence
                    && previous.status === next.status && previous.reason === next.reason
                    && previous.session_token === next.session_token
                    && previous.configuration_generation === next.configuration_generation
                    && previous.run === next.run) {
                upstream[name] = previous
            }
        }
        if (sameCaptureView(upstreamState.capture_view, upstream.capture_view)) {
            upstream.capture_view = upstreamState.capture_view
        }
        const status = upstream.error || (upstream.execution_view ? upstream.execution_view.status
            : upstream.preparing ? "Preparing upstream work"
            : upstream.busy ? (upstream.progress ? "Upstream simulation " + upstream.progress.status : "Upstream simulation active")
            : upstream.has_result ? "Upstream simulation " + upstream.result.status
            : upstream.configured ? "Upstream configuration ready" : "No accepted upstream configuration")
        return Object.assign({}, metadata, {upstreamState: upstream,
            blocks: upstream.configuration_view ? upstream.configuration_view.blocks : ({}),
            details: upstream.configuration_view ? upstream.configuration_view.details : ({}),
            feedbackMessage: message, feedbackIsError: isError,
            display: {executionStatus: metadata.executionStatus, feedback: message,
                upstreamStatus: status}})
    }

    /**
     * Reuse immutable original-source display aliases across unchanged polling.
     * Native hashes/metadata identify exact delivery and binary source; page and
     * plot bounds identify its deterministic view. New errors also replace it.
     * Temporary metadata comparisons retain no second source or signature owner.
     */
    function sameCaptureView(previous, next) {
        if (!previous || !next || previous.family !== next.family || !previous.view || !next.view) {
            return false
        }
        const left = previous.view, right = next.view
        if (!Array.isArray(left.metadata_rows) || !Array.isArray(right.metadata_rows)
                || left.error !== right.error || !!left.plot !== !!right.plot
                || JSON.stringify(left.metadata_rows) !== JSON.stringify(right.metadata_rows)
                || JSON.stringify(left.page) !== JSON.stringify(right.page)) {
            return false
        }
        const leftPlot = left.plot || {}, rightPlot = right.plot || {}
        return leftPlot.start === rightPlot.start && leftPlot.count === rightPlot.count
            && leftPlot.total === rightPlot.total && leftPlot.maximum_count === rightPlot.maximum_count
    }

    /** Request an original capture rectangle; decoding and bounds stay native. */
    function capturePlot(family, first, count) {
        const error = upstreamBackend.capture_plot(family, first, count) || ""
        poll()
        feedback(error)
        return error.length === 0
    }

    /**
     * Publish one coherent shell snapshot; failed details keep fixed control flags live.
     * @returns False preserves old identities, selection and display data.
     */
    function adopt(value) {
        if (coverage.replace(value)) {
            return true
        }
        theme.presentationAdmissionRejected = true
        // These existing boolean slots have unchanged logical bytes/entry counts.
        if (coverage.value) {
            coverage.value.applicationBusy = !!value.applicationBusy
            for (const key of ["busy", "preparing", "can_start"]) {
                if (Object.prototype.hasOwnProperty.call(upstreamState, key)
                        && typeof value.upstreamState[key] === "boolean") {
                    upstreamState[key] = value.upstreamState[key]
                }
            }
            host.presentationChanged()
            host.upstreamStateChanged()
        }
        return false
    }

    /** Poll the upstream owner even when its persistent tab is hidden. */
    function poll() {
        const upstream = JSON.parse(upstreamBackend.poll())
        // Consume exact-identity controls transiently before optional text admission.
        // A held old presentation must never strand a native terminal operation.
        if (upstreamView && upstreamView.sectionDialogs && upstream.draft) {
            const popup = upstreamView.sectionDialogs[upstream.draft.section]
            if (popup && typeof popup.receiveControl === "function") {
                popup.receiveControl(upstream.draft)
            }
        }
        const accepted = adopt(candidate(JSON.parse(backend.state()), upstream,
            presentation.feedbackMessage || "", !!presentation.feedbackIsError))
        if (accepted && reopenConfigurationIndex !== noRequestedSection && !upstream.preparing) {
            const section = configurationSections()[reopenConfigurationIndex]
            reopenConfigurationIndex = noRequestedSection
            if (!upstream.error && upstream.configured && upstreamView
                    && presentation.selectedDirection === "upstream") {
                Qt.callLater(function() { upstreamView.openSection(section) })
            }
        }
        return accepted
    }

    /** Request a native read-only draft of the exact accepted upstream configuration. */
    function beginDraft(section) {
        if (!accepts("upstream", presentation.upstreamIdentity)) {
            return false
        }
        const error = upstreamBackend.begin_draft(section) || ""
        poll()
        feedback(error)
        return !error
    }

    /** Submit exact changed strings; only a later native adoption is acceptance. */
    function applyDraft(section, token, values) {
        if (!accepts("upstream", presentation.upstreamIdentity)) {
            return false
        }
        const error = upstreamBackend.apply_draft(section, token, JSON.stringify(values)) || ""
        poll()
        feedback(error)
        return !error
    }

    /** Target the original draft even after a tab change; never call run Stop. */
    function cancelDraft(token) {
        const error = upstreamBackend.cancel_draft(token) || ""
        poll()
        return !error
    }

    /** Open an existing public observation through the direction-owned worker. */
    function beginInstrument(kind) {
        if (!accepts("upstream", presentation.upstreamIdentity)) {
            return false
        }
        const error = upstreamBackend.begin_instrument(kind) || ""
        poll()
        feedback(error)
        return !error
    }

    /** Forward finite native selectors with their exact admitted view context. */
    function requestInstrument(kind, selection, context) {
        if (!accepts("upstream", presentation.upstreamIdentity)) {
            return false
        }
        const error = upstreamBackend.instrument(kind, JSON.stringify(selection), JSON.stringify(context)) || ""
        poll()
        feedback(error)
        return !error
    }

    /** Open a passive configured-resource query without acquiring a run lease. */
    function beginResourceGrid() {
        if (!accepts("upstream", presentation.upstreamIdentity)) {
            return false
        }
        const error = upstreamBackend.begin_resource_grid() || ""
        poll()
        feedback(error)
        return !error
    }

    /** Forward bounded viewport fields and their exact captured native context. */
    function requestResourceGrid(request, context) {
        if (!accepts("upstream", presentation.upstreamIdentity)) {
            return false
        }
        const error = upstreamBackend.resource_grid(JSON.stringify(request), JSON.stringify(context)) || ""
        poll()
        feedback(error)
        return !error
    }

    /** Validate the actual configured choice through the same native query route. */
    function selectResourceGrant(grantId, context) {
        const grid = resourceGridState.snapshot ? resourceGridState.snapshot.grid : null
        if (!grid) {
            return false
        }
        const target = Object.assign({}, context, {target_grant_id: String(grantId)})
        return requestResourceGrid({frame_index: grid.frame_index, first_symbol: grid.first_symbol,
            symbol_count: grid.symbol_count, first_carrier: grid.first_carrier,
            carrier_count: grid.carrier_count, max_cells: resourceGridState.max_cells}, target)
    }

    /** Inspect an original capture through the native picker without applying it. */
    function loadCapture(family) {
        if (!accepts("upstream", presentation.upstreamIdentity)) {
            return false
        }
        const error = upstreamBackend.choose_capture(family) || ""
        poll()
        feedback(error)
        return !error
    }

    /** Forward the native original-record page coordinate without decoding it. */
    function capturePage(family, first) {
        if (!accepts("upstream", presentation.upstreamIdentity)) {
            return false
        }
        const error = upstreamBackend.capture_page(family, first) || ""
        poll()
        feedback(error)
        return !error
    }

    /** Keep picker Cancel intact; close/reopen only after a real queued import. */
    function loadConfiguration() {
        if (!accepts("upstream", presentation.upstreamIdentity)) {
            return false
        }
        const section = draftState.section || "ucd"
        const error = upstreamBackend.choose_configuration() || ""
        const state = JSON.parse(upstreamBackend.state())
        if (!error && state.preparing && upstreamView) {
            reopenConfigurationIndex = configurationSections().indexOf(section)
            const popup = upstreamView.sectionDialogs[section]
            if (popup) {
                popup.close()
            }
        }
        poll()
        feedback(error)
        return !error
    }

    /** Return transient compiled navigation tags without retaining a source array. */
    function configurationSections() {
        return ["ucd", "profiles", "grants", "source", "receiver", "channel", "preEqualization"]
    }

    /**
     * Admit selection before changing the existing native cursor.
     * @param direction One compiled direction tag; no active run is redirected.
     * @returns False preserves the previous selection when presentation cannot fit.
     */
    function selectDirection(direction) {
        if (direction !== "downstream" && direction !== "upstream") {
            return false
        }
        if (presentation.selectedDirection === direction) {
            return true
        }
        const metadata = JSON.parse(backend.state())
        metadata.selectedDirection = direction
        if (!adopt(candidate(metadata, upstreamState, "", false))) {
            return false
        }
        // Both tags were checked above; native selection changes no scientific owner.
        return backend.select(direction)
    }

    /** Validate exact current native origin before every direction-specific action. */
    function accepts(direction, identity) {
        return !!identity && direction === presentation.selectedDirection
            && backend.accepts(direction, identity)
    }

    /** Retain original bounded action errors only after presentation admission. */
    function feedback(message) {
        return adopt(candidate(JSON.parse(backend.state()), upstreamState, message, !!message))
    }

    /**
     * Dispatch existing native controls to their captured selected direction only.
     * @returns False for stale origins, unavailable controls or original native errors.
     * No action is redirected to another direction that happens to be running.
     */
    function runAction(action, direction, identity) {
        if (!accepts(direction, identity)) {
            feedback("Configuration changed. Retry the action in the selected direction.")
            return false
        }
        if (coverage.rejected && (action === "start" || action === "configure")) {
            return false
        }
        const downstream = direction === "downstream"
        const view = downstream ? downstreamView : upstreamView
        const engine = downstream ? downstreamBackend : upstreamBackend
        const current = JSON.parse(engine.poll())
        let error = ""
        if (action === "configure") {
            if (!view || current.busy) {
                return false
            }
            if (downstream) {
                view.configureProject()
            } else {
                view.openSection("ucd")
            }
        } else if (action === "start") {
            if (!current.can_start || (downstream && (!view || !view.hasProject
                    || !view.allBlocksComplete || view.loadingProject))) {
                return false
            }
            if (downstream) {
                // The preserved downstream QObject has a void Start contract.
                // Observe its real outcome; never interpret undefined as success.
                engine.start()
                const after = JSON.parse(engine.poll())
                const accepted = !!after.busy || (typeof after.result_token === "string"
                    && after.result_token !== current.result_token && !!after.timing)
                if (!accepted) {
                    error = after.reason || "Downstream simulation did not start."
                }
            } else {
                error = engine.start() || ""
            }
        } else if (action === "stop" || action === "pause") {
            if (!current.busy) {
                return false
            }
            error = action === "stop" ? (engine.stop() || "") : (engine.pause() || "")
        } else {
            return false
        }
        if (downstream && downstreamView) {
            downstreamView.pollRun()
        }
        poll()
        feedback(error)
        // Optional display admission never changes the actual native action outcome.
        return error.length === 0
    }
}
