> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/rxmer-capture/validation.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# Upstream RxMER decoder validation — 2026-11-20

[Source and independent cases](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/rxmer-capture/design.md) · [User guide](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/user/rxmer-captures.md) ·
[Architecture](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/architecture.md#rxmer-input) ·
[Official checklist](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/checklist.md#7-complete-configuration-json-and-pnm-admission)

RXMER-FORMAT-01/RXMER-INSPECT-01 pass for the explicit bounded major2 upstream
CCAP profile. Measured association/power playback remains RXMER-INPUT-01.
Interpretation boundaries remain part of the delivered contract, not silently
converted into model readiness.

## Actual source and independent evidence

The pinned CCAP-OSSI I18 SHA-256 and field-offset matrix are in the source/fixture
records. Original pp396/397/419 were visually inspected, with pp418–420 and608
reviewed in context. PHY I20 §9.4.8 provides the upstream equalized-probe reference;
its scope does not establish this OSSI file layout.

Six Rust tests use two pre-implementation authored byte fixtures with independent
frozen hashes. They verify both spacings, exact metadata fields, contiguous raw/
optional readings, literal quarter-dB anchors, all256 byte encodings, null versus
zero, every truncated prefix, trailing bytes, hostile count/physical-grid bounds,
empty data, exact4393-byte limit and one byte beyond it. All vendor/PreEq byte
values and opaque ID/padding are preserved. Clones, input mutation/drop, export
and failed parsing do not change the accepted original.

The source averages-width/model-domain mismatch is retained explicitly: Table410
allocates two file bytes while Table409 uses UnsignedByte. Fixture0x0123 is a raw
width/order vector, not a valid capture-control request. The25kHz fixture uses1.
No source field is silently narrowed to fit the object model. Byte ordering is
an explicit adapter profile of ordered hexadecimal fields; raw flag plus optional
object-model0/1 rendering does not assert a separate normative file truth encoding.
These gaps remain requirements for future semantic measured-input admission.

Native tests run the real extension and compare both exported observations with
a separately executed Rust consumer. They verify raw and decoded getters, exact
original SHA-256, unknown metadata, source-offset errors, byte cap and owned
original/readings/export lifetimes. No Python parsing/scientific arithmetic,
external project dependency, copied parser or new crate dependency is introduced.

## Local gates

Host: Linux x86_64, Python3.10.12, Rust/Cargo1.85.1. Exact command logs and private
PDF/page evidence remain ignored under `docs/development/local/rxmer-capture/`.

| Command | Actual outcome |
| --- | --- |
| `scripts/cargo-local.sh test -p morgana --test rxmer_capture --locked` | Pass, six independent fixture/encoding/extent/metadata/ownership tests. |
| `scripts/validate.sh` | Pass, 99 Rust tests, 5 Git-helper and19 setup tests; workspace formatting, examples/doctest targets and Clippy; 27.29s. |
| `python3 -B tools/python/check_native.py` | Pass, 13 actual native tests, including independent Rust RxMER consumer; 53.84s with native build. Existing UCD/simulation/control/parity cases pass. |
| Optional native formatting | Pass using `scripts/cargo-local.sh fmt --manifest-path bindings/python/Cargo.toml --all -- --check`. |
| `CARGO_TARGET_DIR=target/python scripts/cargo-local.sh clippy --manifest-path bindings/python/Cargo.toml --all-targets --locked -- -D warnings` | Pass, 23.15s. |
| `PYTHONPATH=target/python-tools python3 -B tools/python/build_wheel.py` | Pass, 69.83s: optimized CPython3.10/Linux x86_64 wheel, 170 entries, 79 attributed dependencies with full notices/hash checks; all13 actual artifact runtime tests pass. |

The rebuilt wheel remains a local host artifact, not publication or portability
evidence. Requested cleanup completed after all validation consumers finished:
3831 root-dev files (645.3 MiB), 2003 native-dev files (979.6 MiB) and896
native-release files (297.5 MiB) were removed through Cargo's profile-specific
cleanup. Temporary native import trees were also removed. All three output
profiles are empty; original inputs, source/index, dependency/toolchain downloads,
validation logs and the tested wheel remain retained. Final traceability review
passes103 task IDs/dependencies/cycles and59 requirement links; implemented
architecture paths exist and the source/guidance candidate excludes private data.
No device/network capture, commit, push, publication or system installation is
executed. No external interoperability, complete vendor-layout support or measured
simulation claim follows from these authored protocol fixtures.
