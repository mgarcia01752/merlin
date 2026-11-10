# SRC-04..06: exact-byte payload source

Status: implemented and validated; SRC-04..06 complete. Qt remains planned.

The [task design](design.md) breaks down the approach by SRC-04, SRC-05 and SRC-06.

The source remains outside FEC/PHY algorithms. `simulation/source/file/` owns
bounded immutable input, MSB-first file bits, finite optional PRBS15 fill,
independent verification and recovered bytes. Channel orchestration selects
PRBS15 or file traffic; EOF stops enqueueing new words while symbols continue
through observation/completion/drain. The CLI loads paths and exports bytes;
the Rust API remains path-independent.

Implemented policy: maximum16MiB original input and16MiB-equivalent total meaningful
bits; empty files allowed; explicit target bits for optional fill; reject targets
below original length, excessive targets and invalid fill seeds before work.
Fill starts from its own declared seed at the first bit after the original file,
with no codeword reseeding. Incomplete final BCH payloads use zero padding outside
meaningful BER. Recovered export preserves original-length bytes only; partial
jobs export only complete recovered bytes and report partial status. Raw payload
bytes are not serialized in settings/results or logged.

No minimum/automatic channel-derived target is introduced. Those future size
rules remain deferred. File selection in Qt is planned, not implemented here.

## Validation

- Four source tests independently verify file MSB order, preserved binary/text
  bytes, PRBS recurrence across words, injected segment errors, padding exclusion,
  empty/equal targets, exact payload boundaries, bounds/seeds and rejected-state
  invariance.
- Four actual waveform tests cover both spacings, independent file/fill recovery,
  EOF idle, empty/short input, insufficient duration, early/mid-run cancellation,
  bounded recovered prefixes and noisy observer-disabled/enabled/exhausted
  equivalence. The session retains frozen file settings when the caller changes
  its configuration afterward.
- Two real CLI tests verify binary/text round trips, equality with shared Rust
  simulation summaries, metadata-only JSON, no-clobber and rejected input/target/
  seed/option combinations. Existing continuous PRBS tests remain unchanged.
- The pre-push gate now includes a release file round trip using
  `examples/channel-file.json` and a byte-for-byte `cmp` of the recovered message.
- Full pre-push gate passes on the local Linux host:32 helper tests, Bash and
  ShellCheck, formatting, warnings-denied all-target Clippy, workspace tests,
  analysis-only tests, fixture example, release CLI build and all simulation
  smoke checks, including byte-for-byte file recovery. Ten new Rust source,
  waveform and CLI tests pass within that gate.
- Final launcher alignment uses the existing Cargo wrapper for the file smoke
  check, respecting custom Cargo target directories. Bash/ShellCheck and the
  release file round trip pass again after this tooling-only adjustment.
  Windows/macOS runtime checks remain unverified.

Generated validation logs are kept locally under
`docs/development/local/file-payload-source/`; they are intentionally untracked.

The maintained [user guide](../../../user/simulation/file-source/README.md)
links generation and verification diagrams/math; the [API reference](../../../reference/public-api.md)
records reusable byte-oriented entry points. Source probes retain the existing
`source.payload` stage ID. No Qt UI, new FEC mode, MAC service, dependency or
mandatory input alignment is introduced.

## Release demonstration

The tracked `examples/channel-file.json` request and97-byte message produced a
complete pilot-estimated channel run:776 original bits plus19,224 appended PRBS15
bits,20,000 meaningful bits total, zero bit errors in both segments. The final
information payload contained8,464 coding-padding bits, excluded from BER and
recovered output. The97 recovered bytes matched the input with `cmp`.
The local result and recovered file sit beside the full-gate log in the ignored
validation directory above. Toolchain: project-local Cargo/Rust1.85.1 on Linux.
