# Paired Rust and native Python API review

[Skill](../SKILL.md) · [Current API architecture](../../../docs/architecture.md#api-rust-01-basic-rust-api-and-optional-native-adapters)

Use this when planning or changing shared APIs, including explicitly scoped
upstream work. Keep MERLIN's current Rust imports, Python package, CLI behavior
and saved-workbench contract. Current exports and executed parity belong to the
[single operation matrix](../../../docs/development/features/high-level-api/compatibility-matrix.md).
The upstream Rust owner is `crates/ofdma/src/api`; native conversions live in
`bindings/python/src/upstream`. This procedure alone establishes no new support.

## Inventory and file placement

Inspect current public exports, downstream callers, optional binding manifests,
Python declarations, error conversion and parity tests before extending a surface.
Keep a feature table mapping each Rust operation to its Python signature, shared
validation owner, result/error category and acceptance case. Identify whether
each operation is implemented or proposed. Name implementation, declaration and
test files with Add (planned), Edit or Reuse dispositions. JSON serialization
alone is not a typed high-level API.

Retain independently validated direction-specific configurations rather than
one giant parameter structure. Converge operation conventions only where their
semantics agree. Distinguish inventory discovery, measurement admission/playback,
preview and batch/run execution. Discover IDs through Rust, never fixed Python
catalogs in examples promising complete coverage. Loading/configuration stays
idle until an explicit run; rejected inputs preserve accepted state.

## Proposed declarations versus working bindings

During architecture work, keep proposed Rust declarations and Python signatures
in labeled documentation fences. They are neither callable APIs nor a stability
promise. A type stub, `todo!()` or successful no-op does not establish support.
Do not create an importable scaffold merely to satisfy a design diagram.

Implement and test the public Rust operation first. Expose it through the existing
optional native adapter after shared validation and domain semantics are established.
Preserve the project's pins unless a separately justified dependency change is
required. Keep `.pyi`/`py.typed` declarations, if shipped, consistent with actual
exports, defaults, return types and exceptions. An import/signature check alone
does not demonstrate scientific behavior.

## Conversion, ownership and lifecycle

Rust owns units, domain validation, association, transformations, execution and
result schemas. Python owns checked argument/buffer conversion and structured
exception mapping. Separate native Python argument errors from shared domain
categories; do not match human-readable error strings. Use Python `None` and
exceptions rather than Rust Option/Result syntax in Python-facing examples.

Document copies, mutability and buffer lifetime. Returned results must remain
usable after caller input release, later calls and session disposal according
to the declared ownership contract. Never infer zero-copy safety from a wrapper.

For long work, release the interpreter lock in the native adapter. Define mutable
session serialization, Busy/error behavior and cleanup before exposing concurrent
calls. Cancellation must remain reachable from another thread. Bound progress
delivery; never call Python from a worker holding core locks. Specify safe pause,
resume and cancel boundaries, preserved completed work and elapsed-time policy.
Unsupported lifecycle controls remain explicitly unavailable.

For example, a proposed Rust `preview(&self) -> Result<Preview, Error>` might map
to Python `preview(self) -> Preview`, with a structured domain error for missing
inputs. This is an illustrative mapping, not an existing MERLIN signature or a
new error name. Neither preview should start work or mutate accepted state.

## Acceptance across consumers

Cover negative/overflowing counts, nonfinite values, incompatible grids, invalid
IDs, limits and unsupported modes through the shared domain contract. Preserve
original bytes, hashes, identities, missing values and source provenance. Never
silently discard a persisted setting, substitute a guessed capture or run on load.

Acceptance needs a Rust-only build independent of Python/Qt, actual native import
and signatures, valid/invalid conversions, retained-result ownership and paired
deterministic runs against direct Rust calls. Compare selected inputs, settings,
error categories, seeds, provenance, status and completed/partial outcomes, with
explicit numerical tolerances. Test interpreter-lock release, cancellation and
pause/resume natively when exposed. Matching JSON strings, parsed source or mock
Python calculations cannot substitute for this evidence.

For persistence changes, apply the complete
[saved-workbench compatibility contract](../SKILL.md#saved-workbench-compatibility)
including UI-produced recipes through Rust and Python. Manual conversion is a
remaining gap. Update both languages' documentation/examples and keep pending
native, wheel, installed and hosted checks visibly distinct.
