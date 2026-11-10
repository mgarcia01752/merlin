> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/run-infrastructure/`
> Logs, captures and specification images are not published.

# RUN + infrastructure completion

Scope complete: RUN-01 through RUN-08, ARCH-05/06/07 and QA-04 for fixed-profile
noiseless operation. No new waveform impairment, hard slicer, mixed profiles,
higher-order coding mode, Python binding, Qt UI or device/network service.

Linux pinned Rust1.85.1. The final full pre-push gate passed on the final behavioral
source: formatting, warnings-denied all-target Clippy, workspace/analysis-only tests,
32 Git/release helper tests, release CLI build, old coding smoke, new channel and
versioned JSONL smoke, version metadata. Eighteen new tests cover independent
stage maps/reverse composition, schema/path errors, RNG invariance, byte/I/O/flush
failures, source-window edges, cancellation/retention, host budget and actual CLI.
Final changes after that gate are documentation/rustdoc only; rustfmt and whitespace
checks passed. Scientific-testing skill validated through canonical and both links.

Default one-second actual CLI job: status complete, recovered
8862 words, 126123984 global
PRBS bits and zero payload/PLC bit or codeword errors. Source-window rate
126010128.0 information bit/s in the synthetic24MHz50kHz
fixture, excluding1 boundary-crossing word and7 entirely outside words. Sixty-four
word details retained with8798 omissions explicitly counted; no unbounded word history.
This is not a192MHz capacity claim or an external interoperability result.

Actual CLI SIGINT test: status cancelled, retained completed science with
14232 compared bits and zero errors; final window rate
remained unavailable. The handler was exercised after a real recovered payload
probe, using only the owned test process and ignored development outputs.

438 local doc link paths and balanced fences checked. New Mermaid diagrams inspected
as source, not rendered. User block diagram recorded as later impairment reference.
Architecture and checklist updated; next simulation work remains MODEL-04.

Initial gate failed for disk exhaustion, not numerical tests. Removed only this
checkout's generated debug profile via cargo clean --profile dev (15.4GiB), preserving
source, downloads and release artifacts. Successful full gate used command-local
CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 to reduce
debug artifact size. No system packages/global configuration changed.
Windows/macOS runtime and external full-channel interoperability remain unverified.
