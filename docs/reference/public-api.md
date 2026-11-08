# Public API

DOFT's scientific entry points are public Rust APIs. The standalone CLI calls
those same functions. Future Python bindings and an optional Qt presentation
component will adapt inputs and results without duplicating scientific logic.

| Public module | Responsibility |
| --- | --- |
| `scenario::{grid, observation}` | Exact identity, source provenance and full-resolution observations |
| `scenario::association` | Explicit-tolerance timestamp association |
| `scenario::document` | Versioned decoded scenario inputs |
| `profile::candidate` | Captured assignments and editable hypotheses |
| `profile::{evaluation, document}` | Temporal screening, candidate documents and role verification |
| `signal_processing::{modulation, noise}` | QAM, soft demapping and residual-noise models |
| `simulation::{config, runner, results, source}` | Bounded trial orchestration and reproducible source state |
| `types::ofdm_modulation` | Normative constellation registry and implementation status |
| `fec_codec::{parameters, bch, ldpc}` | Mode dimensions and individual codecs |
| `fec_codec::concatenated` | Encode, reverse-decode and compare without a channel model |

Canonical modules are shown above. Existing imports under `math`, `profile::model`,
`profile::association`, `profile::simulation` and
`profile::document::ScenarioDocument` remain supported by re-exports. Existing
root `fec_codec::{encode_bch, decode_bch, LdpcCodec, ...}` imports also remain valid.
No duplicate scientific implementations are kept for compatibility.

New APIs should use documented units, typed input/results, structured recoverable
errors and explicit ownership/lifetime contracts. Library functions must not exit
the process, depend on GUI objects, require Python objects or emit results through
console text. Consumers preserve missing values and original scientific results.

Progress callbacks and cancellation are Rust job interfaces. A future binding
will manage Python interpreter locking, callback delivery and buffer lifetimes.
No Python dependency, binding package or stable C ABI is introduced now.

See the [public-api skill](../../skills/public-api/SKILL.md) for API development
requirements and [JSON contracts](documents.md) for the existing serialization.
