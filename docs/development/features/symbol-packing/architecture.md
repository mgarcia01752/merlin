> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/symbol-packing/`
> Logs, captures and specification images are not published.

# Stateful uniform256-QAM symbol packer

## Scope

Pack fixed2025-cell short8/9 data codewords across successive pre-interleaving
symbol domains. Caller provides the correct per-symbol inverse-interleaved
scattered-pilot placeholders. Physical layouts cannot substitute for that input.
Fixed NI per job; exclude PLC/CP/exclusions before this component. No automatic
pilot generation, physical time/frequency interleavers, NCP wire encoder or IFFT.

## Evidence

Local PHY I20 sections7.5.5.5.1–2 PDF125–128,7.5.14.4 PDF163–164,
8.3.4.2–3 PDF182–184. Pointers count eligible data positions includingzero-loading,
not SP; data ascends, NCP descends.48bits/NCP block, plusCRC encoded asoneblock.
Atmost10active starts per8K symbol or anytwo consecutive4K symbols. One optional
terminalNULL orZ marker; markerlastbeforeCRC. NULL permitted when adding it
forces a data continuation to the next symbol. Initial NCP profile has no
zero-loaded NCP cells; uniformQPSK/16QAM/64QAM supported for capacity calculation.

## Plan algorithm

For each candidate1..11 NCP data entries, reserve(entries+CRC)*48/bits cells
at high end. Preview queued words in ascending remaining data slots up to active
start budget. Emit starts with data-position rank, then Z marker iff filler;
otherwise requireNULL forzero starts, optionally allowoneNULL whenexact capacity
needs it. Select smallest valid reservation. All validation/preview precedes
queue mutation. NoSP consumes a pointer rank. No wire bytes invented.

## Lifecycle

Own boundedqueue<=16words; labelsowned andvalidated0..255,length2025. IDsmonotonic.
FixedsymbolNI andconsecutivegeneratedindices. Partialwordoffset carried; noreset
atcodeword boundary. Invalidcalls leavequeue/indicesunchanged. Outputownedplans
retain IDs/offsets as test provenance, not a receiver-side substitute for NCP.
NCP wire encoding/CRC needednextbefore claiming receiver codeword discovery.

## Acceptance

Continuation/multiple starts/filler/NULL/exact reservation/SP skipping, domains
bounds, no state changes onerrors, and rolling4Kbudget. Map/randomize per symbol,
advance reserved NCP cells with synthetic test labels (not realNCP messages),
soft inverse then reassemble using oracle IDs/offsets and decode actualFEC words.
All original payloads must recover withzeroerrors; LDPCiterations retained.

## Completed validation

Implemented model/planner/queue modules, compact summary JSON, public example,
five boundary/lifecycle tests and one cross-symbol FEC integration test.
The integration recovered four14232-bit payloads with zero errors across a
128-symbol randomizer boundary. It uses diagnostic boundary oracles and synthetic
NCP clock labels; real NCP decoding remains pending.

Full scripts/check-before-push.sh passed on the local Linux host: helper tests,
ShellCheck/syntax, rustfmt, Clippy with warnings denied, workspace tests,
analysis-only tests, release CLI build and AWGN smoke. Example JSON accounting
and changed Markdown links were checked. Windows/macOS remain unverified.
See checks.log and example.json in this directory for local evidence.
