# US-RXMER-SYN-01: Synthetic CMTS RxMER files

Implement an offline generator and paired Rust/Python examples on the current
backend baseline. This initial step does not add the upstream Qt Designer or
change RAVEN. A future RAVEN parser consumes the standard bytes; no sibling
checkout, parser service or device connection is required by MERLIN.

## Contract and evidence

Authority: CableLabs `CM-SP-CCAP-OSSIv3.1-I18-200610`, sections 7.3.2.5,
7.3.3.1 and 7.3.5.5, Tables 409/410, printed pages 396–397 and 418–420.
Pinned PDF SHA-256:
`a74611960b377d8f23e9d9929af6c272c671164eba858a087432e7ce66c0d0aa`.
Table 410 was visually inspected; the private PDF stays ignored under
`docs/development/local/upstream-synthetic-rxmer/`.

Generate major 2/minor 0 `0x504E4E69` files with the existing explicit
most-significant-octet-first adapter profile. The header is 297 bytes; data uses
one quarter-dB octet per physical carrier, including unavailable `0xFF` readings.
Preserve 25/50 kHz grids and explicit CM/CCAP/RPD/interface/time identities.
Number of averages is a two-byte file field, but generator requests respect
Table 409's unsigned-byte control domain and require a positive count. Metadata
does not mean MERLIN executed probes or their leaky-integrator averaging.

The Rust request supports a flat sequence with explicit missing physical carriers
or explicit per-carrier values. Reject invalid grids, unsupported spacing,
nonfinite values, duplicate missing positions and resource excesses. Existing
Reject/Clamp export policy controls finite out-of-range readings; nearest-quarter
rounding uses positive half-ties up. Preserve the request and conversion summary.
Every result includes synthetic provenance, exact binary hash and OSSI identity.
The binary alone does not encode origin: keep its provenance sidecar when exporting.

## Owners and API parity

| Owner | Disposition / responsibility |
| --- | --- |
| `crates/pnm-input/src/common/rxmer.rs` | Add shared quarter-dB conversion/limits; reuse from downstream export and upstream generation. |
| `crates/pnm-input/src/upstream/rxmer_format.rs` | Add single header/offset owner reused by decoder and generator. |
| `crates/pnm-input/src/upstream/synthetic_rxmer.rs` | Add typed bounded request and immutable generated artifact; no I/O or simulation. |
| `bindings/python/src/upstream/synthetic_rxmer.rs` | Add thin native conversion; Rust owns validation, generation and provenance. |
| Rust/Python examples and synthetic request JSON | Add exclusive new output directory containing binary and sidecar; never overwrite user files. |
| `tools/testing/native_api.py` | Generate both fresh Rust reference pairs for the existing complete installed-wheel gate; retain their paths in run evidence. |
| Existing decoder/export tests | Reuse independent authored hex/scaling vectors and retain downstream behavior. |

Rust `SyntheticRxmerCapture::generate(&SyntheticRxmerRequest)` is typed;
`from_json(&str)` is its bounded transport adapter. Python
`merlin.upstream.SyntheticRxmerCapture.from_json(request)` invokes that same
operation. Rust `capture()` exposes the existing decoder observation; Python
returns owned binary bytes. Both `to_json()` operations return the synthetic
provenance artifact. Construction neither configures nor starts a simulation.
Rust exports `MAX_SYNTHETIC_RXMER_REQUEST_BYTES`; Python exposes the same value
as `SyntheticRxmerCapture.MAX_REQUEST_BYTES`, with no duplicated numerical limit.
The [operation matrix](../high-level-api/compatibility-matrix.md#subsequent-focused-extensions)
adds all four Rust methods and four native methods/constants while retaining
historical acceptance records unchanged.

Architecture: [PNM ownership](../../../architecture.md#unified-pnm-and-capture-admission).
Requirement: [REQ-US-RXMER-SYN-01](../../../requirements.md#req-us-rxmer-syn-01).
Task: [US-RXMER-SYN-01](../../checklist.md#task-us-rxmer-syn-01).

Acceptance requires literal header/scaling expectations independent of generation,
both spacings, missing carriers, range policies, invalid/resource limits, immutable
provenance, original decoder regression and downstream export regression. Compare
actual installed native Python output with the independently executed Rust example;
check signatures, structured errors, ownership and no-clobber example behavior.
Use targeted tests, formatting, affected Clippy and documentation/traceability
checks. Full FEC/Qt suites and release packaging are outside this change.

RAVEN follow-up: independently implement/validate the same Table 410 profile with
these synthetic files as additional fixtures, retaining unsupported-version and
unknown metadata behavior. Generated fixtures are not field interoperability
evidence and do not replace independent literal parser vectors.

## Executed validation

Local acceptance passed on Linux x86_64 with CPython 3.10. Commands reuse the
existing MERLIN-local Cargo cache, target and Rust 1.85.1 toolchain. Evidence:
[named cases, source hashes, commands and timings](test-results.json).

| Executed check | Observed result |
| --- | --- |
| `scripts/cargo-local.sh test -p pnm-input --locked` | 73 passed in 163.12 s; includes downstream workbench, capture/export and upstream admission regressions. |
| `scripts/cargo-local.sh test -p pnm-input --test upstream_synthetic_rxmer --test upstream_rxmer --test rxmer_export --locked` | Final 16 cases passed in 1.90 s, including the subsequently added shipped-fixture test and expanded version rejection cases. Production Rust behavior was unchanged after the complete crate run. |
| Installed fresh native wheel: synthetic generation, existing upstream RxMER decoder, downstream export and selected namespace/typing tests | Nine cases passed, with separate Rust reference processes for both generated grids and the original decoder fixtures. |
| Rust and Python examples, both grids | Identical binary and sidecar bytes; repeat invocations reject existing directories and preserve every output hash. |
| `clippy -p pnm-input --all-targets --locked -- -D warnings` | Passed, 17.10 s. |
| Binding manifest `clippy --all-targets --locked -- -D warnings` | Passed, 30.23 s. |
| Workspace and binding-manifest `fmt --all -- --check` | Passed. |
| Fresh rustdoc and installed operation inventories | Four new Rust declarations match compiler signatures; all 377 native matrix entries match the current installed inventory. Historical behavior records remain historical; affected native tests were rerun. |
| Native acceptance-runner tooling tests | Two passed; fresh references are wired into the existing full wheel gate. |
| Feature links, requirement/checklist/architecture and operation matrix | Passed. |

The final native wheel was rebuilt after its export audit found that a public
module constant did not fit the documented class-based export contract. Moving
the same Rust limit onto the native class resolved that failure; final installed
tests and inventory were rerun. No failed run is recorded as accepted.

Full workspace FEC/PHY/CLI, Qt/QML and release packaging were not required:
no engine, session, CXX-Qt registration, project serialization, CLI or packaging
contract changed. Complete affected adapter tests and downstream conversion
regressions were executed. Other platforms, hosted CI and release publication
were not executed for this local implementation.

Delivered usage and fixtures: [generator guide](../../../user/upstream/synthetic-rxmer.md),
[paired examples](../../../../examples/api/README.md#synthetic-cmts-rxmer-in-both-languages)
and [binary/provenance pairs](../../../../examples/measurements/upstream/synthetic-rxmer/README.md).
