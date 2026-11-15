---
name: public-api
description: Design and review DOCSIS OFDM Channel Sim public Rust APIs so standalone tools and optional Python bindings can share the same simulation core. Use when adding or changing public simulation models, operations, errors or job interfaces.
---

# Public simulation API

DOCSIS OFDM Channel Sim owns a standalone Rust simulation core. The native Qt
interface and optional Python package must call the same algorithms. The canonical high-level API lives under `crates/ofdm/src/api`; the optional
`bindings/python` adapter calls it directly, independently of Qt and the CLI.

- Put reusable signal-processing operations in crates/ofdm or crates/fec-codec, exposed
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

See the [high-level API guide](../../docs/user/api/README.md) and
[Python guide](../../docs/user/python/README.md). Rust-only consumers retain
the existing low-level APIs; no stable C ABI is promised.
