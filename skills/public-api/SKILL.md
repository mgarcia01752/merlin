---
name: public-api
description: Design and review DOCSIS OFDM Channel Sim public Rust APIs so standalone tools and optional Python bindings can share the same simulation core. Use when adding or changing public simulation models, operations, errors, job interfaces or saved-workbench JSON contracts.
---

# Public simulation API

When planning or extending shared Rust/Python operations, read the
[paired API procedure](references/paired-api.md) for operation/signature mapping,
non-executable planning declarations, native ownership and lifecycle parity.
It applies to explicitly scoped upstream work while preserving the existing
downstream surface and saved-workbench compatibility below.

For unified backend work, the [canonical adapter design](../../docs/architecture.md#unified-rust-python-and-cli-apis)
retains `crates/ofdm/src/api` and `bindings/python`; planned upstream operations
belong in `crates/ofdma/src/api`, with native input admission in `crates/pnm-input`.
Follow that planned/current distinction rather than retaining source facade names
or forcing upstream algorithms into downstream-specific models. Final paths and
API/user documentation require INT-DOC-02 acceptance after actual integration.

The [selected namespace mapping](../../docs/development/features/high-level-api/namespace-design.md)
plans a thin Cargo/library `merlin` facade and independent `merlin-api` contract
leaf, plus one real Python package with a private native extension. These owners
remain planned until the API delivery gates; preserve the existing downstream
exports and root native `merlin.Simulation` without an early placeholder facade.

DOCSIS OFDM Channel Sim owns a standalone Rust simulation core. The native Qt
interface and optional Python package must call the same algorithms. The canonical Basic Rust API lives under `crates/ofdm/src/api`; the optional
`bindings/python` adapter calls it directly, independently of Qt and the CLI.

- Put shared numerical kernels in crates/dsp, directional processing in crates/ofdm
  or the explicitly scoped crates/ofdma, and codecs in crates/fec-codec, exposed
  through documented public Rust modules. CLI or GUI code adapts inputs/results;
  it must not contain a second implementation of a calculation.
- Use meaningful domain types, explicit units, stable enums with documented
  unsupported cases, and typed result models. Keep measured, estimated and
  simulated values distinguishable. Preserve null readings and carrier roles.
- Keep public inputs independent of Python, Qt, filesystem paths and transport.
  Use slices for borrowed numeric input and owned results where practical.
  A binding may own arrays or opaque session handles; do not force callers to
  construct self-referential objects or access private state.
- Document ownership, lifetimes, mutation and thread-safety. Simulation inputs
  remain unchanged unless a clearly named editing operation documents mutation.
  Do not promise zero-copy access without proving buffer lifetime and layout.
- Return Result for recoverable failures. Prefer structured errors with stable
  categories and details so a wrapper can map exceptions without parsing text.
  No process exit, print-driven results or caller-input panics in library APIs.
- Keep work limits, cancellation, progress and partial-result behavior explicit.
  Python wrappers manage the interpreter lock and callback delivery;
  simulation routines must not depend on that lock or on interpreter objects.
- Keep binding dependencies optional and in the separate `bindings/python` crate.
  Do not add PyO3 or Python runtime dependencies to the reusable core or CLI.
  Keep scientific validation and calculations in Rust; no Python sidecars.
  Do not freeze a C ABI or expose Rust's memory layout merely to prepare bindings.
- Represent versions and assumptions explicitly in exported documents. Keep JSON
  as one adapter to typed models, not the only route to signal-processing operations.
- Add public-API tests covering valid/invalid input, units, missing samples,
  resource bounds and reusable results. Check Rust-only builds remain independent
  of bindings and UI dependencies. Keep native Python parity, cancellation and result-ownership tests current.

All signal processing and FEC must remain Rust. Any future wrapper only converts
inputs, dispatches work and returns Rust-derived results. Follow coding-style and
scientific-testing for documentation banners, independent evidence and validation.

For Qt infrastructure reuse, follow coding-style's PNM Viewer reuse guidance.
Inspect that checkout read-only and adapt presentation components into MERLIN.
Keep viewer types, Qt types and GPL UI code out of the independent Apache-2.0
simulation API; adapters consume MERLIN models rather than importing viewer
analysis/capture ownership. CLI and simulation builds remain Qt-independent.

See the [Basic Rust API guide](../../docs/user/api/README.md) and
[Python guide](../../docs/user/python/README.md). Rust-only consumers retain
the existing low-level APIs; no stable C ABI is promised.

## Python-facing language contract

Apply coding-style's language-specific API example rules. The **Basic Python
API** shares Rust validation and results, but exposes Python syntax: optional
arguments accept a value or `None`, and failures raise structured exceptions.
Do not leak Rust `Option`/`Result` syntax into Python usage or signatures.
For example, the existing override is
`OcdOverrides(bandwidth=BandwidthHz(OCCUPIED_BANDWIDTH_HZ))`.
Omit `bandwidth` or pass `None` to leave that override unset. Do not advertise
`bandwidth_hz=` unless that parameter is actually implemented and tested.

## Saved-workbench compatibility

Saved workbenches are shared inputs for the Qt UI, Basic Rust API and Basic
Python API. Apply this contract whenever changing workbench JSON (`*.merlin.json`,
`wb.json` or equivalent), its schema, defaults, embedded payloads, save/load code,
or UI settings persisted into it.

- Every supported simulation setting and capture representation saved by the UI
  must load and run through both APIs without manual conversion. This includes
  RxMER Designer recipes, not only materialized binary or decoded JSON captures.
- Implement decoding, recipe materialization, validation and simulation semantics
  in shared Qt-independent Rust code, exposed through the Rust API and consumed
  by Python. Do not duplicate scientific calculations in Python or depend on Qt
  to reconstruct saved inputs.
- Preserve profiles, selections, receiver modes, grids, missing values, seeds,
  provenance, defaults and execution limits. Presentation-only state may be
  ignored by headless callers; simulation settings must not be silently dropped
  or replaced. Loading must remain idle until an explicit run call.
- Add or update a UI-produced saved-workbench fixture and exercise it through
  both Rust and Python loaders. Verify resolved inputs/settings and a bounded,
  deterministic engine run with matching status, FEC outcomes and partial-result
  behavior where affected. Test changed recipes against the shared Designer
  output. Record test elapsed times; a successful UI reopen alone is insufficient.
- Preserve previously supported schema versions or provide explicit versioned
  migration and structured rejection for unsupported versions/payloads. Cover
  affected invalid inputs and bounds, and update both API docs/examples when
  their behavior changes.
- Treat a UI/API compatibility gap as unfinished work. Record the gap and its
  regression case; do not describe a manual conversion workaround as full
  saved-workbench support or mark the change complete before both APIs pass.
