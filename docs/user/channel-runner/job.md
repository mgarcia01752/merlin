# Channel job lifecycle

The job wrapper owns host-runtime limits, cooperative cancellation, progress and
bounded retention. The underlying session still owns scientific sequencing and
independent TX/RX state. Simulated observation duration does not set host execution
speed or the wall-clock budget.

```mermaid
flowchart TB
    REQUEST["Validate scientific settings and job limits"] --> SESSION["Create channel session"]
    SESSION --> CHECK["Check cancellation and elapsed host time"]
    CHECK -->|"Continue"| STEP["Advance one bounded OFDM step"]
    STEP --> SAVE["Retain completed scalar totals and bounded word details"]
    SAVE --> PROGRESS["Small read-only progress notification"] --> CHECK
    STEP -->|"Finished"| COMPLETE["Complete result"]
    STEP -->|"Stage or sink error"| FAILED["Failed result plus diagnostic and completed totals"]
    CHECK -->|"Cancelled or runtime exhausted"| PARTIAL["Explicit partial status; preserve completed science"]
```

Statuses are `complete`, `cancelled`, `runtime_exceeded` and `failed`. These are
independent of BER and FEC success flags. Cancellation/runtime exhaustion does
not fabricate comparisons for incomplete words and does not force an extra drain.
It preserves completed scalar science, finite word details and current pending
received bits/window counts. A partial prefix may legitimately have no compared
bits and null BER/rate.

The token is checked before every symbol and final-tail operation, including
acquisition/completion/drain. A step, constructor, observer or progress callback
cannot be interrupted internally, so the host budget can be exceeded by that
bounded operation and callback latency. Caller callback costs count toward the
next budget check. The default host budget is300000ms; requested observation time
remains separately configurable with a one-second default.

Detailed words are retained up to an explicit count, default64 and maximum10000.
Zero retention is supported. Scientific totals continue after detail retention is
exhausted. `omitted_word_details` counts details not retained, including recovery
completed before a failed observer prevented a successful step return.
`unrecovered_generated_payloads` counts generated/admitted words without recovered
comparisons. It is not the same as received pending coded bits or announced starts.

Observer failures return a Failed result with the diagnostic and already-completed
summary comparisons. This preserves science without pretending a failed export
was successful. There is no implicit restart or retry of a failed session.
The CLI writes that failed result before returning a nonzero exit status.
Cancellation/runtime partial results are valid outputs and return CLI success;
consumers must inspect `status`.

A deterministic-clock unit test checks runtime exhaustion without flaky sleeps.
Integration tests cancel after real recovery, cancel before work, reject invalid
limits, preserve results after sink failure and bound retained details. Separate
[CLI tests](../../../crates/cli/tests/channel.rs) exercise actual file behavior.
