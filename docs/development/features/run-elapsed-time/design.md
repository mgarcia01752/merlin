# UI-RUN-TIME-01: Visible run and profile elapsed time

Show monotonic wall elapsed time during Running/Paused and retain the frozen total
when the sweep finishes, stops or fails. Also show current-profile wall time.
Reset on a new Start or accepted input/configuration invalidation. These are UI
run diagnostics, not simulated symbol duration or the existing active-time budget.
Wall time includes pauses, setup, CPU scheduling waits and result delivery.

A small shared Rust clock owns timestamps, progress age, phase, decoded-word count
and at most32 per-profile wall durations. The worker marks boundaries and completion;
UI polling cannot extend a finished timer. QML renders formatted strings via its
existing100ms poll and uses wrapping layout to protect lower-right controls.
Preserve Stop/Pause behavior, simulation limits and original result accounting.
Show phase/recovered progress because source-symbol count stops increasing during
interleaver drain, even while receiver/FEC work continues.

Review evidence: core progress fires after each completed step; observation-symbol
count excludes drain. Pause/Stop/runtime checks occur between steps. LDPC invokes a
bounded1..100-iteration decoder per word. A long step can therefore hold progress
steady; elapsed time alone does not prove a hang or identify competing processes.
Validate clock boundaries/formatting without sleeps, worker completion/reset and
QML running/paused/final presentation. Build Qt and capture real native timing.

## UI-TERMS-02: Processing terminology

Extend the existing About-linked terminology Markdown with interleaver depth and
interleaver drain, including the implemented M-1 drain-symbol relation and a
depth16/15-symbol example. Distinguish queued-word completion, waveform tail,
simulated timing and CPU/decoder-dependent wall runtime. Add a canonical UI-skill
rule requiring new technical status terms to be defined in this guide.

## Validation and limits

Qt checks pass40 adapter tests and39 QML checks. Clock tests use injected instants
to verify advancing time with stationary progress, exact formatting, idempotent
finish and delayed-poll freezing without sleeps. Worker tests cover complete
single/multi-profile retention and invalidation; QML checks cover running, paused,
finished and reset display. Formatting and warnings-denied Clippy pass.

A native bounded run uses the curated single-WB inputs, profile3 only, original
50-iteration limit,1ms observation and a reduced5s active budget in an ignored
scratch WB. It consumed5.14 process CPU seconds across5.57 elapsed seconds
(about92% of one core on the8-logical-CPU host), and returned RuntimeExceeded.
During this sample it was acquiring; this does not reproduce the user's full
run or establish why their profile3 took longer. No competing build was running
during the sample. The UI timer advanced and stayed frozen after the worker ended;
current running/final dark captures are linked from the user guide.
Stop/Pause/budget remain cooperative between core steps; a timer is not a watchdog.
No non-Qt core build/tests, profiling changes or algorithm changes were needed.

The existing About → Terminology URL already targets the canonical Markdown.
Added interleaver depth/drain definitions, implemented M-1 relation and explicit
simulated-versus-wall distinction. UI skill validation passes and its discovery
links remain intact. Publication of local documentation follows the normal Git
workflow; no push was performed.
