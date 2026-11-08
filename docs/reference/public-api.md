# Public API

DOFT's scientific entry points are public Rust APIs. The standalone CLI calls
those same functions. Future Python bindings and an optional Qt presentation
component will adapt inputs and results without duplicating scientific logic.

| Public module | Responsibility |
| --- | --- |
| `profile::model` | Exact grids, provenance, observations and editable candidates |
| `profile::association` | Explicit-tolerance timestamp association |
| `profile::evaluation` | Candidate generation, temporal screening and rates |
| `profile::simulation` | Bounded real-codec trials, progress and cancellation |
| `profile::document` | Versioned decoded scenarios and candidate documents |
| `types::ofdm_modulation` | Normative constellation registry and implementation status |
| `fec_codec` | BCH/LDPC encoding, decoding and parity validation |

New APIs should use documented units, typed input/results, structured recoverable
errors and explicit ownership/lifetime contracts. Library functions must not exit
the process, depend on GUI objects, require Python objects or emit results through
console text. Consumers preserve missing values and original scientific results.

Progress callbacks and cancellation are Rust job interfaces. A future binding
will manage Python interpreter locking, callback delivery and buffer lifetimes.
No Python dependency, binding package or stable C ABI is introduced now.

See the [public-api skill](../../skills/public-api/SKILL.md) for API development
requirements and [JSON contracts](documents.md) for the existing serialization.
