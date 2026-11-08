---
name: public-api
description: Design and review DOFT public Rust APIs so standalone tools and future Python bindings can share the same scientific engine. Use when adding or changing public scientific models, operations, errors or job interfaces.
---

# Public scientific API

DOFT owns a standalone Rust scientific engine. A future Python package and optional
Qt interface must call the same algorithms. Implement functionality first; Python
bindings, packaging and UI work are deferred until explicitly requested.

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
