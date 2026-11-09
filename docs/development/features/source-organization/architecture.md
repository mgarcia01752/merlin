> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/source-organization/`
> Logs, captures and specification images are not published.

# Source organization refactor

User approved the proposed three-crate module organization. Preserve public Rust
imports using re-exports and preserve JSON/numeric contracts and seeded draw order.

- fec-codec: parameters, BCH, LDPC and concatenated encoding/reverse decoding.
- ofdm: shared domain types, signal processing, simulation source/runner/results,
  scenarios/association and candidate profile evaluation.
- CLI: startup, arguments, focused commands and bounded JSON I/O.

The focused codec loop must run without QAM, noise, profiles or capture files.
PRBS is not added in this refactor; retain the existing seeded random source and
its shared RNG stream with AWGN. The source directory will own PRBS when defined.
No empty placeholder modules or new algorithm variants are introduced.

Validation: snapshot baseline JSON from the existing binary for noiseless, AWGN,
decoder failure, measured noise and candidate analysis. Compare after refactoring,
excluding elapsed time only. Add codec-loop and canonical public-path tests, retain
all existing tests and run analysis-only, Clippy, formatting and full push gate.
No commits, pushes, system installation or edits to other repositories.
