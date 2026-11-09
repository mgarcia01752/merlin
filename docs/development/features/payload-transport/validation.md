> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/payload-transport/`
> Logs, captures and specification images are not published.

# CELL-02 / WAVE-RX-06 validation

Completed the supported fixed256-QAM ProfileA / positive uniform NCP path in
reusable TX assembly/stream, received configuration/control/collector/decoder
and RX stream modules. Added10 bounded production observation stages and a
sample-domain reference verifier/example. No new algorithm dependency or version
change. PRBS and configurable-duration job lifecycle remain later work.

## Evidence and contracts

Reused existing independent codec, QAM, randomizer, NCP, cell-interleaving,
pilot-reference and waveform vectors. Reviewed pinned local PHY I20 source
paragraphs for continuous codewords, NCP placement/CRC/selectors and independent
source/channel timing. Exact new source mappings are in the tracked evidence page.
No fresh external interoperability or blind acquisition claim is made.

The receiver consumes a live Ready PLC snapshot and actual normalized received
cells, never TX IDs/offsets/reservation lengths/original bits. Configuration
comparisons preserve independent descriptor counters. An explicit128-symbol
session origin follows real PLC acquisition in the waveform proof. Startup/drain
BPSK replacements are TX-only diagnostics; RX suppresses known M-1 pair warm-up.

## Checks performed

- Full scripts/check-before-push.sh: PASS after final source/test changes.
  Includes32 release/Git helper tests, shell syntax/ShellCheck, formatting,
  warnings-denied all-target Clippy, workspace and analysis-only tests, release
  CLI build/version checks and actual bounded JSON simulation smoke validation.
-258 Rust test passes across workspace and analysis-only invocations; no failures.
-12 new Rust tests in5 component/integration files. Both spacings, all3 NCP
  orders, maximum depth, phase128 crossing, words completed during drain,
  queue/empty-drain/short-interval bounds, changed/rejected gate, malformed input,
  unsupported DPD/activation, orphan/early boundaries and valid-FEC wrong-CRC
  are covered. Pending history remains on terminal control rejection.
- All10 production producers observed; disabled/enabled/failed/exhausted callbacks
  preserve actual recovered payload. Enabled observations must return success;
  failed-callback mode explicitly permits its injected errors.
- Actual waveform proof at both spacings: real PLC acquisition, all4 scheduled
  payloads recovered without bit errors, real NCP/LDPC iterations and BCH flags,
  source reset crossing and explicit final drain/remainder. RX chunks137 samples.
- Example payload_waveform: PASS, JSON assertions confirm4 words/56928 payload
  bits per spacing, zero payload/PLC bit errors,160 accepted NCP fields,
  warm-up/drain15 at25kHz and31 at50kHz.
- Analysis-only all-target Clippy: PASS. No signal-processing/native FFT dependency added.
- Documentation: links/anchors/fences checked across README/docs,11 per-subsystem
  payload guides contain Mermaid diagrams. Diagrams were not rendered.
- scientific-testing skill validates at canonical and both discovery paths.
- git diff --check: PASS.

## Findings resolved

An initial test incorrectly discarded valid drain-completed words and assumed
short source intervals had finished warm-up before drain; corrected to count real
source completions throughout drain. Probe validation caught an accepted-message
domain registration mismatch; corrected and enabled-probe tests now reject
unexpected callback errors. Compiler derives and test iterator/allocation lints
were resolved. No outstanding validation failures.

## Remaining work

SRC-02/SRC-03 PRBS with independent vectors, then RUN configurable-duration
orchestration with initial1-second default, checked64-bit sample accounting,
bounded results, cancellation/progress/partial outcomes. Existing streams have
no per-second reset. Dynamic activation/mixed loading, nonideal acquisition,
external interoperability and Windows/macOS runtime verification remain separate.
