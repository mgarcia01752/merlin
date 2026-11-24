/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import "../shared/presentation"
import "../shared/presentation/LogicalFootprint.js" as Footprint

// Opt-in native acceptance only; Main owns creation and the fresh evidence path.
QtObject {
    id: probe
    required property var theme
    required property var workbench
    required property var window
    required property string screenshotPath
    signal finished(bool success, string message)

    // Scheduling and traversal bounds, independent of presentation/scientific limits.
    readonly property int pollIntervalMs: 25
    readonly property int phaseRetryLimit: 400
    readonly property int overallTimeoutMs: 120000
    readonly property int itemTraversalLimit: 8192
    readonly property int searchPartitions: 2

    property PresentationOwner pressureOwner: PresentationOwner {
        id: pressure
        theme: probe.theme
    }
    property QtObject state: QtObject {
        property string phase: "opening"
        property bool done: false
        property bool connected: false
        property bool screenshotSaved: false
        property bool optionalDetailsRejected: false
        property bool pollNotice: false
        property bool helpRejected: false
        property int phaseRetries: 0
        property int applyCount: 0
        property int searchTrials: 0
        property int searchTrialLimit: 0
        property double startedAt: 0
        property double lowerUnits: 0
        property double upperUnits: 0
        property double maximumUnits: 0
    }
    property Timer phaseTimer: Timer {
        interval: probe.pollIntervalMs
        repeat: true
        onTriggered: probe.advance()
    }

    /** Fail the acceptance operation instead of continuing with partial evidence. */
    function require(condition, message) {
        if (!condition) {
            throw new Error(message)
        }
    }

    /** Move to one bounded asynchronous phase without blocking the GUI thread. */
    function next(phase) {
        state.phase = phase
        state.phaseRetries = 0
    }

    /** Count only the actual receiver dialog's successful native Apply callback. */
    function countApplied() {
        state.applyCount += 1
    }

    /** Disconnect the one callback without changing the receiver's own handlers. */
    function disconnectApplied() {
        if (state.connected) {
            workbench.receiverConfigPopup.configurationApplied.disconnect(probe.countApplied)
            state.connected = false
        }
    }

    /** Finish once and release the sole filler on success, failure or unwinding. */
    function complete(success, message) {
        if (state.done) {
            return
        }
        state.done = true
        phaseTimer.stop()
        pressure.clear()
        disconnectApplied()
        finished(success, message)
    }

    /** Locate the actual rendered admission notice through a finite Item traversal. */
    function noticeVisible() {
        const pending = [window.contentItem]
        let visited = 0
        while (pending.length && visited < itemTraversalLimit) {
            const item = pending.pop()
            visited += 1
            if (item.objectName === "presentationAdmissionNotice") {
                return item.visible
            }
            const children = item.children || []
            require(pending.length + children.length <= itemTraversalLimit,
                    "Admission-notice traversal exceeded its fixture bound.")
            for (const child of children) {
                pending.push(child)
            }
        }
        return false
    }

    /** Derive code-unit capacity from native policy and the existing UTF-16 helper. */
    function beginSearch() {
        const adapter = pressure.backend()
        require(!!adapter, "Native presentation adapter was not created.")
        const limits = JSON.parse(adapter.limits())
        const unit = Footprint.measure("x", limits)
        require(unit.ok && unit.utf16_bytes > 0,
                "Existing logical helper did not measure one code unit.")
        state.maximumUnits = Math.floor(limits.max_utf16_bytes / unit.utf16_bytes)
        require(Number.isSafeInteger(state.maximumUnits) && state.maximumUnits > 0,
                "Native presentation policy has no usable filler capacity.")
        state.lowerUnits = 0
        state.upperUnits = state.maximumUnits
        state.searchTrialLimit = Math.ceil(Math.log(state.maximumUnits + 1)
                / Math.log(searchPartitions)) + 1
        next("searching")
    }

    /** Test one fresh candidate; no preceding filler survives the trial boundary. */
    function searchStep() {
        if (state.lowerUnits === state.upperUnits) {
            next("filling")
            return
        }
        state.searchTrials += 1
        require(state.searchTrials <= state.searchTrialLimit,
                "Logical filler search exceeded its derived trial bound.")
        require(pressure.clear(), "Prior filler could not be cleared.")
        const units = Math.floor((state.lowerUnits + state.upperUnits + 1)
                / searchPartitions)
        if (pressure.replace("x".repeat(units))) {
            state.lowerUnits = units
        } else {
            state.upperUnits = units - 1
        }
    }

    /** Prove the inclusive boundary, then leave the largest filler held for Apply. */
    function holdPressure() {
        require(state.lowerUnits > 0, "No logical filler was admitted.")
        require(pressure.clear(), "Search filler could not be cleared.")
        require(!pressure.replace("x".repeat(state.lowerUnits + 1)),
                "One-unit-above candidate unexpectedly fit after the search.")
        require(pressure.replace("x".repeat(state.lowerUnits)),
                "Largest measured filler no longer fits.")
        require(!pressure.replace(pressure.value + "x"),
                "Filler growth unexpectedly fit while the old owner remained held.")
        require(pressure.value.length === state.lowerUnits,
                "Rejected growth changed the accepted filler.")
        next("applying")
    }

    /** Capture the real dark window while all pressure and rejection remain held. */
    function captureWindow() {
        require(theme.dark === true, "Native evidence requires the existing dark theme.")
        require(screenshotPath.length > 0, "A fresh screenshot destination is required.")
        require(pressure.value && pressure.value.length === state.lowerUnits,
                "Pressure was released before the screenshot.")
        next("capturing")
        // ApplicationWindow.contentItem is Qt-owned and has no QML engine.
        const started = workbench.grabToImage(function(result) {
            if (state.done) {
                return
            }
            try {
                require(result.saveToFile(screenshotPath), "Native screenshot could not be saved.")
                state.screenshotSaved = true
                next("recovering")
            } catch (error) {
                complete(false, String(error))
            }
        })
        require(started, "Native window capture could not start.")
    }

    /** Advance only existing native actions; never starts a scientific run. */
    function advance() {
        if (state.done) {
            return
        }
        try {
            state.phaseRetries += 1
            require(state.phaseRetries <= phaseRetryLimit,
                    "Native presentation probe timed out in " + state.phase + ".")
            require(Date.now() - state.startedAt <= overallTimeoutMs,
                    "Native presentation probe exceeded its overall watchdog.")
            const dialog = workbench.receiverConfigPopup
            if (state.phase === "opening") {
                if (workbench.loadingProject || workbench.runState.busy) {
                    return
                }
                require(theme.dark === true, "Probe must preserve the existing dark theme.")
                require(!dialog.visible && !workbench.faqPopup.visible,
                        "Probe requires initially closed receiver and Help dialogs.")
                dialog.configurationApplied.connect(probe.countApplied)
                state.connected = true
                require(dialog.openChecked(), "Receiver configuration could not open before pressure.")
                next("waiting_editor")
            } else if (state.phase === "waiting_editor") {
                if (!dialog.opened || !dialog.presentationCoverage.textAdmitted
                        || !dialog.presentationCoverage.value || !dialog.footer.applyEnabled) {
                    return
                }
                beginSearch()
            } else if (state.phase === "searching") {
                searchStep()
            } else if (state.phase === "filling") {
                holdPressure()
            } else if (state.phase === "applying") {
                require(dialog.opened && dialog.footer.applyEnabled,
                        "Pressure disabled the existing native Apply action.")
                require(state.applyCount === 0, "Receiver Apply fired before the probe action.")
                dialog.footer.applied()
                next("waiting_apply")
            } else if (state.phase === "waiting_apply") {
                require(state.applyCount <= 1, "Receiver Apply callback fired more than once.")
                if (state.applyCount !== 1 || dialog.visible || dialog.opened) {
                    return
                }
                const current = JSON.parse(workbench.runBackend.poll())
                require(current.accepted_options !== null
                        && typeof current.accepted_options === "object",
                        "Native Apply did not retain accepted receiver options.")
                state.optionalDetailsRejected = dialog.presentationCoverage.rejected
                require(state.optionalDetailsRejected,
                        "Optional receiver detail replacement did not reject under pressure.")
                theme.presentationAdmissionRejected = false
                workbench.pollRun()
                require(theme.presentationAdmissionRejected,
                        "Actual workbench polling did not report detail rejection.")
                state.pollNotice = true
                next("rejecting_help")
            } else if (state.phase === "rejecting_help") {
                theme.presentationAdmissionRejected = false
                require(workbench.faqPopup.showEntry("", null) === false,
                        "Help unexpectedly opened under full logical pressure.")
                require(!workbench.faqPopup.visible && !workbench.faqPopup.opened
                        && theme.presentationAdmissionRejected,
                        "Rejected Help did not stay closed with visible global feedback.")
                state.helpRejected = true
                next("waiting_notice")
            } else if (state.phase === "waiting_notice") {
                if (!noticeVisible()) {
                    return
                }
                captureWindow()
            } else if (state.phase === "capturing") {
                // The bounded Timer remains the watchdog for the image callback.
                return
            } else if (state.phase === "recovering") {
                require(state.screenshotSaved, "Pressure evidence was not saved.")
                require(pressure.clear(), "Final logical pressure could not be released.")
                workbench.pollRun()
                require(workbench.faqPopup.showEntry("", null) === true,
                        "Help did not recover after pressure was released.")
                next("waiting_help")
            } else if (state.phase === "waiting_help") {
                if (!workbench.faqPopup.opened) {
                    return
                }
                workbench.faqPopup.close()
                next("waiting_close")
            } else if (state.phase === "waiting_close") {
                if (workbench.faqPopup.visible || workbench.faqPopup.opened) {
                    return
                }
                complete(true, JSON.stringify({probe: "session-presentation",
                    receiver_apply_count: state.applyCount,
                    native_accepted_options: true,
                    optional_receiver_details_rejected: state.optionalDetailsRejected,
                    poll_notice_visible: state.pollNotice,
                    help_rejected_under_pressure: state.helpRejected,
                    help_recovered: true,
                    largest_padding_code_units: state.lowerUnits,
                    maximum_policy_code_units: state.maximumUnits,
                    search_trials: state.searchTrials,
                    screenshot: screenshotPath,
                    dark_theme: theme.dark}))
            } else {
                throw new Error("Unknown native probe phase.")
            }
        } catch (error) {
            complete(false, String(error))
        }
    }

    Component.onCompleted: {
        state.startedAt = Date.now()
        phaseTimer.start()
    }
    Component.onDestruction: {
        phaseTimer.stop()
        pressure.clear()
        disconnectApplied()
    }
}
