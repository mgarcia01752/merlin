---
name: public-api
description: Design and review DOCSIS OFDM Channel Sim public Rust APIs so standalone tools and future Python bindings can share the same simulation core. Use when adding or changing public scientific models, operations, errors or job interfaces.
---

# Public scientific API

DOCSIS OFDM Channel Sim owns a standalone Rust simulation core. The native Qt
interface and future Python package must call the same algorithms. Qt UI work is
the next phase; Python bindings remain deferred until UI-01 through UI-04.

- Put reusable scientific operations in crates/ofdm or crates/fec-codec, exposed
  through documented public Rust modules. CLI or GUI code adapts inputs/results;
  it must not contain a second implementation of a calculation.
- Use meaningful domain types, explicit units, stable enums with documented
  unsupported cases, and typed result models. Keep measured, estimated and
  simulated values distinguishable. Preserve null readings and carrier roles.
- Keep public inputs independent of Python, Qt, filesystem paths and transport.
  Use slices for borrowed numeric input and owned results where practical.
  A binding may own arrays or opaque session handles; do not force callers to
  construct self-referential objects or access private state.
- Document ownership, lifetimes, mutation and thread-safety. Scientific inputs
  remain unchanged unless a clearly named editing operation documents mutation.
  Do not promise zero-copy access without proving buffer lifetime and layout.
- Return Result for recoverable failures. Prefer structured errors with stable
  categories and details so a wrapper can map exceptions without parsing text.
  No process exit, print-driven results or caller-input panics in library APIs.
- Keep work limits, cancellation, progress and partial-result behavior explicit.
  Future Python wrappers manage the interpreter lock and callback delivery;
  scientific routines must not depend on that lock or on interpreter objects.
- Keep future binding dependencies optional and in a separate adapter crate.
  Do not add PyO3, a Python runtime, Python sidecars or Python calculations now.
  Do not freeze a C ABI or expose Rust's memory layout merely to prepare bindings.
- Represent versions and assumptions explicitly in exported documents. Keep JSON
  as one adapter to typed models, not the only route to scientific operations.
- Add public-API tests covering valid/invalid input, units, missing samples,
  resource bounds and reusable results. Check Rust-only builds remain independent
  of bindings and UI dependencies. Add actual Python tests when bindings exist.

All signal processing and FEC must remain Rust. Any future wrapper only converts
inputs, dispatches work and returns Rust-derived results. Follow coding-style and
scientific-testing for documentation banners, independent evidence and validation.

For Qt infrastructure reuse, follow coding-style's PNM Viewer reuse guidance.
Inspect that checkout read-only and adapt presentation components into MERLIN.
Keep viewer types, Qt types and GPL UI code out of the independent Apache-2.0
scientific API; adapters consume MERLIN models rather than importing viewer
analysis/capture ownership. CLI and scientific builds remain Qt-independent.
