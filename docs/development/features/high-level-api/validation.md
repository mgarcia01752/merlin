# High-level Rust and Python API validation

Validated locally on 2026-11-14 on Ubuntu 22.04.5 x86_64, Rust 1.85.1 and
CPython 3.10, using maturin 1.8.3 and patched PyO3 0.30.1. These are local
checkout results, not hosted CI, published packages or external modem evidence.
The original handoff archive is unchanged. No commit, push, publication or
system-wide installation was performed.

## Implemented contract

The [requirements mapping](requirements.md) and [design](design.md) describe the
canonical `docsis_ofdm_fec::api`, optional `pnm-input` capture loader and separate
`bindings/python` adapter. Existing simulation, signal processing and FEC code
is reused. The high-level API does not copy Qt orchestration or introduce another
engine. Rust owns scientific validation, residual-power operations and outcomes.

Tests cover binary and JSON profile/RxMER/channel imports, content-based kind
validation, bounded input/storage, separate original/intermediate provenance,
profile import/create/edit, exact grids and protected roles, OCD origins and
explicit bandwidth, CP/RP/depth defaults, independent capture collections,
controller-scoped IDs and atomic rejection. All four RxMER selections and
explicit/unity/nearest-time channel selection are implemented. Tests cover
missing contributors, duplicate epochs, unequal interpolation intervals, exact
original endpoints, nearest-time ambiguity and no extrapolation.

Loading and `prepare()` produce no simulation report or FEC outcomes. Only an
explicit run dispatches the existing engine. Direct-engine comparisons use the
same resolved settings/seeds and actual counters. Cancellation/runtime tests
preserve started results, unavailable denominators and not-started requests.
Complete execution status does not imply FEC success.

## Executed checks

| Check | Result and measured duration |
| --- | --- |
| `scripts/test-local.sh all`, before starting Python implementation | 395/395 workspace tests, 4/4 preparation-example tests, 136/136 no-default-feature tests; no skips; doctests passed. Total including compilation: **15m 48s**. |
| `scripts/cargo-local.sh test -p pnm-input --locked`, after rename and API refinements | **14/14 passed**, **71.20s** test time; doctests passed. |
| Direct engine parity after expanding unity/measured-response coverage | **1/1 passed**, **73.67s**; both response policies matched the direct engine. |
| Workspace Clippy, all targets, locked, warnings denied | Passed; **32.69s** including compilation. |
| Separate Python-workspace Clippy, all targets, locked/offline, warnings denied, Qt-free namespace | Passed; **0.57s** final warm check (an earlier compiled check took 9.51s). |
| Root and Python-workspace rustfmt checks; `git diff --check` | Passed. |
| Fixture-backed Rust `high_level` example | Complete with actual FEC/profile results; also built and run in the Qt-free namespace. |
| Rust `policy_parity` example | Generated fresh actual reference results for `Each`, `Individual`, `AverageAll`, `InterpolateAt`. |
| Installed native Python unittest suite against that Rust reference | **7/7 passed**, **12.283s** test time, **12.506s** command wall time. |
| Release/version tooling tests | **5/5 passed**, **0.361s**; current package/lock versions agree at 0.2.9. |
| Core, Qt and Python locked dependency audit | Zero advisories and zero informational warnings in all three graphs; no suppressions. |
| Rustdoc for high-level core and capture loader | Generated successfully in **7.27s**; six existing intra-doc-link warnings outside the new API modules. |
| Public-API skill and discovery links | Skill validation passed; both discovery symlinks resolve to the canonical skill. |

The full repository runner preceded later API-only refinements. Those refinements
and the crate rename were checked with the targeted API suite and Clippy; the
unchanged simulation/DSP/FEC engine was not subjected to a second full runner.
The final direct-engine test additionally exercises both a unity response and
an explicitly selected measured response, comparing actual status, FEC counters,
BER, rate and matching scopes.

Python checks include binary/JSON inputs, manual OCD/profile hypotheses, structured
errors, immutable values, copied options, foreign IDs, runtime continuation,
non-overwriting export, cross-thread cancellation, Ctrl-C partial results, Busy
protection, interpreter-lock release and ownership after controller deletion.
Counter/status parity is exact. Floating comparisons use absolute tolerances
of 1e-12 for ratios/power sums and 1e-6 for MER in dB and payload bit rate.
Elapsed host time and controller-scoped identities are excluded from parity.

## Independence verification

A bubblewrap mount namespace exposed system `/usr`, `/lib` and `/lib64`, the
MERLIN sources and pinned Cargo/Rust toolchain caches. A host audit found all
76 matching Qt library paths under `/lib/merlin/lib` and its `/usr/lib/merlin/lib`
alias; both installed-app directories were hidden with empty mounts. The
checkout's Qt app folder was also hidden, and external home/SDK directories
were absent. Qt environment
variables were cleared, network access was disabled and Qt build tools were
unavailable. The Rust CLI and input examples built offline; the CLI printed
`ofdm-fec 0.2.9` and the fixture-backed Rust example completed. The separate
Python adapter also passed Clippy in this Qt-free namespace.

A second namespace omitted access to the shared Rust toolchain/cache directories
and used only the local Python environment plus system paths. Checks confirmed
`cargo`, `rustc`, `qmake` and `qtpaths` were unavailable. The wheel was installed
with `pip --no-index --no-deps`, imported and executed successfully. `ldd` showed
only the platform loader, libc, libm and libgcc_s; no Qt dependency. The final checks explicitly verified the bundled QtCore library was absent
through both system aliases. The actual Python example completed with 82 observation-window codewords, zero corrected,
zero uncorrectable and BER 0 for that fixture. All seven Python tests then passed
in this same isolation against the fresh Rust policy reference.

This proves local dependency isolation and offline wheel installation. It does
not establish pristine-OS provisioning, portable manylinux compatibility or
runtime behavior on another operating system.

## Local delivery artifact

Ignored local artifact:
`local/python-wheels/merlin_ofdm-0.2.9-cp310-cp310-linux_x86_64.whl`

- Size: **1,537,105 bytes**.
- SHA-256: `63e76a13129ed193053d2a44a6d8e1227d203cefe433bb507fc3a2db6494cd69`.
- Adjacent `.whl.sha256` matches the artifact.
- ZIP integrity check passed. The wheel contains 88 locked dependency notice
  records, project LICENSE/NOTICE and embedded BCH/QAM license/notice assets.
- Patched PyO3 0.30.1 notices are included. The earlier experimental 0.27.2
  build is not a delivery artifact; its advisories were resolved by upgrading.

The helper `tools/python/build_wheel.py` regenerates notices before building.
The wheel is native CPython 3.10 Linux x86_64; no other interpreter ABI or
manylinux guarantee is implied. Source and local wheel remain unpublished.

## Remaining verification and explicit scope

Windows, macOS, other architectures and Python versions remain unverified.
Qt presentation was not changed; no new Qt/native-installer lifecycle checks
were required or claimed. Rustdoc's six pre-existing warnings remain outside
this milestone. External modem interoperability is not established by the
closed-loop fixtures. GNU Octave, continuous timeline playback, full CMTS MAC
services and simultaneous channels remain outside the agreed milestone.

Logs and isolation command manifests are retained under ignored
`docs/development/local/api-handoff/`; they are not distributed evidence in a
fresh clone. Reproduce the workflows using the tracked
[Rust guide](../../../user/api/README.md),
[Python guide](../../../user/python/README.md),
[Rust tests](../../../../crates/pnm-input/tests/high_level_api.rs) and
[Python tests](../../../../bindings/python/tests/test_parity.py).

## API-EXAMPLES-01 follow-up

Runnable source is centralized under `examples/api/rust` and
`examples/api/python`, indexed by [examples/api/README.md](../../../../examples/api/README.md).
Cargo checked both registered Rust example targets after relocation. Both
Rust and installed native Python workflow examples completed from their new
locations; Python again reported 82 codewords, zero uncorrectable and BER 0.
Rustfmt, updated relative links and `git diff --check` passed. Source behavior
and fixture bytes were preserved, so the full engine suite was not repeated.

## API-INTEGRATION-02 delivery-tooling follow-up

The separate `tools/python/update_integration_doc.py` generator now owns the
installation guide. Git save/branch push invoke refresh; bump releases refresh
and include the guide in their validated candidate/commit. Existing tags and
dry runs check without writes. Exact resume compares both metadata and guide
against committed template inputs. These paths preserve prior Git guards.

Final local tooling checks: 55 Git workflow tests passed in 27.198s, four
integration-document tests passed in 0.032s, and five release/version tests
passed in 0.242s. Tests used disposable checkouts and local-only bare origins;
no actual project commits, tags or pushes were executed. ShellCheck, Bash
syntax, Python compilation, generated-document checks, 665 changed/new Markdown
relative links, both updated skills and discovery links, and diff whitespace
checks passed. The simulation engine did not change; its suite was not rerun.

Archive ignore checks passed for ZIPs, compressed tar files and Python wheels.
No archive files were tracked; local inputs were not deleted by this change.
The required maintenance and archive policies are recorded in the
[acceptance requirements](requirements.md#mandatory-downstream-installation-document-maintenance).

## Profile catalog discovery (API-INPUT-02)

Rust-first profile discovery is implemented and verified in the current checkout:
`Dpd::profile_ids_from_bytes` for decoded JSON and `CaptureLoader::profile_ids`
for binary/JSON. Python file/byte discovery delegates to this path. The real
channel-193 example discovers 0, 3 and 4 and runs all three without hardcoded IDs.

Checks: 16 Rust API tests, 136 analysis-only tests, 7 Python API tests, all four
Rust/Python policy parity modes, workspace/adapter Clippy and formatting passed.
A native Linux CPython 3.10 wheel was rebuilt with notices and installed only
in the local test environment. See the [design validation](design.md#api-input-02-discover-captured-profile-identities-in-rust)
for timings and example counters. Windows/macOS/other interpreters are unverified;
no wheel was published and no system installation was performed.

## Loaded DPD catalog member (API-INPUT-03)

Static discovery remains available. Rust Dpd::profile_ids() and Python
dpd.profile_ids expose the complete validated catalog retained during import,
independently of the selected profile. Reading needs no file or decoding.
17 Rust API tests and 8 Python API tests passed, including access after source
removal, created/edited metadata and ownership. Four policy parity modes still
match the preserved Rust baseline. Workspace/adapter Clippy and formatting
passed; the native Ubuntu/CPython 3.10 wheel was rebuilt locally. See
[design validation](design.md#api-input-03-retained-profile-id-member-on-loaded-dpd)
for timings. Other platforms remain unverified; no artifacts were published.
