> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/backend-stages/validation.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# Backend stage validation — 2026-11-20

[Design and independent cases](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/backend-stages/design.md) ·
[Architecture](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/architecture.md) · [Official checklist](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/checklist.md)

This record covers the implemented bounded **synthetic scheduled-data backend**.
It is internal source/algorithm/closed-loop evidence, not external interoperability,
RF calibration, modem certification, live MAC behavior or captured-PNM decoding.
The earlier [LDPC source audit](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/ldpc-source-audit/validation.md) remains valid.
The user's index was preserved throughout implementation. The user separately
saved the preceding LDPC audit as `eaa011d`; this backend remains uncommitted.

## Actual implementation and independent evidence

| Boundary | Actual owners / independent cases |
| --- | --- |
| Shared configuration | `configuration/`, `types/`: checked immutable identity/units/counts, source enums, atomic rejection, exact 2K/4K geometry. |
| PRBS | `source/`: literal first bits, period/ones count, chunk/seed/range/continuity and independent chronological verifier. |
| TX-01/02 | `grant/`, `transmitter/framing.rs`: literal699/700/281/3421/11881 thresholds, borrowing/residue and conservation sweep; actual source-verified standalone codec integration. |
| TX-03 / RX-07 | `bit_processing/randomizer.rs`: independent first64-mask positions, soft sign inverse and reset/chunk behavior. |
| TX-04 / RX-06 | `signal_processing/qam*` / `demapping.rs`: independent AnnexA labels at every supported order, mean energies/uniqueness for all labels, literal distances/ties/variance/erasure/clipping. |
| TX-05 / RX-05 | `resources/`, `pilot_sequence.rs`: independently transcribed all14 masks/boost exceptions/PN signs; strict interleaver cases and exhaustive bounded bijection checks. |
| TX-06 | `signal_processing/pre_eq.rs`, `measurements/pre_eq.rs`: complete-domain normalization, phase/excluded/reset cases, snapshots and explicit older corresponding-probe bases with symbol boundaries. |
| TX-07 / RX-03 | `transform.rs`: direct complex sums, DC/positive/negative tones, impulse, Parseval and inverse checks for both lengths. |
| TX-08 | cyclic extension/window/assembly and guard owners: independently numbered samples, half-sample coefficients, distinct adjacent overlap, exact finish/tail and real guard timing. |
| CH-01..06 | single `channel/stream.rs`, `residual.rs` plus run/observers: literal FIR/delay/rotation/tone, signed-zero/subnormal bypass, invalid-state rollback, finite bounds, actual output/tails. |
| Residual model | independent physical-bin normal draws before active masking, known-pilot absolute power reference, actual useful FFT values, chunk/retry invariance; no hidden target-to-RX shortcut. |
| RX-01/02/04 | received framing/estimation: literal CP-versus-useful gaps, missing symbol availability, complex LS anchors, bounded linear brackets, no extrapolation/deep-null fake unity. |
| RX-08/09 | received-only codec/deframer: real failed decisions, fill validity and actual iterations; source comparison outside RX; cancellation retains completed words. |
| End-to-end | all14 pilot patterns, all ordinary QAM orders2..12, both numerologies, multiple frames/grants, nonunity C and synthetic H, destructive/empty/truncated/cancelled outcomes. |
| Controls | real worker pause/resume versus uninterrupted noisy FIR decisions; pre-start pause, stop-from-paused, explicit active-time budget test; actual native SIGINT and Busy/session restoration. |
| Instruments/results | bounded actual QAM/SA; tap/unit/identity/retention and observer neutrality; literal20dB/empty/zero-error MER states; planned/pending/unverified coverage and owned lifetimes. |
| Generic JSON | 108 atomic source field/row dispositions, independent valid/invalid/conditional/enum fixtures, strict bounded parser, original complete inventory, actual Rust/native JSON load/run parity. |

Source control: PHY I20 SHA-256
`304c26937f5beaf7ffddc88158abacead4ead2bc29e6acaae5226898c9f716bc`.
Original pages48,62–63,103–105,197–203,206 were inspected during this stage work,
with earlier reviewed source pages retained for LDPC, IFFT, assembly and Pre-EQ.
MULPI I25 source boundaries/original rendered pages and every applicable/reserved/
inapplicable field are in the [field audit](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/us-configuration-json/coverage.md).
A misread512-QAM test label was corrected by re-reading original Fig109:
label0x70 is (15,1). Production reference data was unchanged by that correction.
MULPI review also established the zero-loading/pilot-pattern0 admission rule.

Numerical acceptance uses declared absolute tolerances for floating transforms,
energy and distances. Source/bit/permutation/pilot-coordinate comparisons are exact.
Pinned same-host noise/chunk/pause parity is exact; no cross-platform bitwise normal/
transform promise is made. Shared tables alone are not independent evidence;
source-derived anchors and independent arithmetic are retained alongside round trips.

## Local commands and evidence

Host: Linux x86_64, Python3.10.12, Rust/Cargo1.85.1. All generated logs, source
PDFs/extractions/images, isolated validation tools and wheels stay ignored.
The native binding is a separate optional workspace with its own checked lock.

| Command / gate | Recorded outcome |
| --- | --- |
| `scripts/validate.sh` | Pass: 88 Rust tests, 5 Git-helper tests, 19 setup-tooling tests, workspace formatting and Clippy. Doctest/example targets compile successfully. |
| `python3 tools/python/check_native.py` | Pass: actual native build/import and nine runtime tests, including separate Rust-process parity for typed, measurement, generic JSON and synthetic-playback JSON runs. |
| `scripts/cargo-local.sh fmt --manifest-path bindings/python/Cargo.toml --all -- --check` | Pass for the separate optional native workspace. |
| `CARGO_TARGET_DIR=target/python scripts/cargo-local.sh clippy --manifest-path bindings/python/Cargo.toml --all-targets --locked -- -D warnings` | Pass; the named native constructor has a narrowly documented argument-count style exception. |
| Draft2020-12 structural schema validation using isolated jsonschema4.25.1 | Schema and all four producer fixtures pass; Rust additionally checks source/cross-field rules. |
| `PYTHONPATH=target/python-tools python3 -B tools/python/build_wheel.py` with existing maturin1.8.3 | Pass: optimized CPython3.10 Linux x86_64 wheel, 170 package entries, 79 attributed dependencies, packaged source-notice hashes checked, all nine actual wheel runtime tests pass. No installation/publication or portability claim. |
| `scripts/cargo-local.sh run -p morgana-cli --locked -- upstream examples/configuration/<fixture>.json` | Pass: 50kHz, 25kHz and synthetic-playback fixtures recover 1976, 4216 and 1976 source bits respectively, with zero errors. Controls-inspection fixture rejects execution with `UnsupportedCapability` as designed. |

The core gate includes guidance/resource closure, version identity, Git/setup
unit checks, formatting, all workspace tests/doctests/examples and Clippy.
Optional Python runtime/tooling remains outside core-only builds. Failed gates
were corrected and rerun; a passing compile alone never closed scientific stages.

Source documents retain the exact original bytes and SHA-256 separately from
canonical JSON; typed reconfiguration clears stale document provenance. Spectrum
results carry actual run/configuration/grant identity, sample origin, clock,
analysis window and units. Independent constant-tone/Parseval tests cover both
FFT lengths and reject joining samples from different grants.

Private evidence logs are `final-gate.log`, `native-final-tests.log`,
`native-clippy.log` and `wheel-validation.log` under ignored
`docs/development/local/backend-stages/`. The wheel remains under ignored
`target/python-wheels/`. At the user's request, generated Cargo dev/release
outputs are cleaned after final verification; dependency/toolchain downloads,
logs and the tested wheel are retained.

Cleanup completed with Cargo's profile-specific commands: 6046 dev files
(962.8 MiB), 2358 native-dev files (about 1 GiB) and 903 native-release files
(299.9 MiB). Temporary native import trees were also removed. No source/index,
dependency/toolchain download, private evidence log or tested wheel was deleted.
The final documentation check passes 20 skills and 971 local links; a separate
anchor/dependency review passes all 59 requirement links and 100 task records.

## Capture research and remaining scope

Read-only primary-source research located additional upstream format definitions:

- CCAP-OSSI3.1 I18-200610 §7.3.2.5 and §7.3.5.5, Table410 pp419–420:
  `PNNi`, major2 header, exact CM/CCAP/RPD/ifIndex/frequency metadata, quarter-dB
  RxMER with0xff missing. Source PDF SHA-256
  `a74611960b377d8f23e9d9929af6c272c671164eba858a087432e7ce66c0d0aa`.
  [Primary source](https://account.cablelabs.com/server/alfresco/8a23d1b7-fbd5-4da0-b128-6da766fc9737).
- CM-OSSI3.1 I27-250219 AnnexD.2.13, Tables77/78 pp189–192:
  `0x504e4e07` last update versus `0x504e4e06` applied coefficients, s1.14 versus s2.13,
  exact subgrid/origin/identity metadata. SHA-256
  `29f0ebf32c4c38610b1b9c10a418ffff4045c34def0ffbe4e85fc674cb5b91de`.
  [Primary source](https://account.cablelabs.com/server/alfresco/1519efdc-d9ae-4d0b-a017-9a1e457bcbcb).

These findings are **research, not implemented import acceptance**. In particular,
a last-update file alone does not establish Set versus Adjust or its corresponding
probe/base bank; byte-level Boolean/control/association behavior and lawful
independent valid/invalid/boundary fixtures still need a dedicated adapter gate.
No downstream decoder or externally supplied device capture is an acceptance
prerequisite, and neither is silently substituted for those source checks.
Synthetic targets/replay are explicitly labeled normalized baseband and retain
their synthetic originals, rather than claiming measured CM/channel/time provenance.

Binary PNM/configuration capture import and measured association, broad C/H/RxMER
composition, ranging/probing/profile-testing waveforms, blind acquisition/fast
tracking, finite-file/MAC payloads, GUI, fresh-machine setup, hosted CI and other
native platforms remain unchecked. The repository's imported PDE guidance still
has unresolved public redistribution terms; no publication is performed or claimed.
Scientific wheel scope excludes that guidance and private evidence. Source
package/release/installer delivery requires its own reviewed acceptance.
