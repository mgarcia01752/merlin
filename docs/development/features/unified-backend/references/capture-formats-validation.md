> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/capture-formats/validation.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# UCD capture format and inspection validation — 2026-11-20

[Source/independent cases](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/capture-formats/design.md) ·
[Architecture](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/architecture.md#upstream-configuration-capture-inspection) ·
[Official checklist](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/checklist.md#7-complete-configuration-json-and-pnm-admission) ·
[User guide](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/user/capture-inspection.md)

US-CONFIG-FORMAT-01 and US-CONFIG-INSPECT-01 cover the exact unfragmented
Type51/version5 DA-through-payload envelope. Full semantic projection,
multipart assembly, transport/CRC validation, PNM RxMER and Pre-EQ decoders
remain pending. No externally supplied modem dataset or external interoperability
claim is part of this acceptance.

## Source and independent acceptance

The pinned MULPI I25 SHA-256, source sections and frozen fixture hashes are in
the source/fixture record. Original pp100/101/107/114 were rendered and inspected;
§6.2.1.2 p83 and Tables26/27 pp103/105 were checked in extracted context. The
version5 multipart rules distinguish unsupported complete-fragment handling from
invalid sequence bounds; no fragment is silently admitted as a full descriptor.

Five Rust capture tests establish literal byte lengths, addresses, fixed fields,
raw snapshot/range/profile values, exact ordered offsets and independent frozen
hashes; every truncated prefix and explicit malformed outer/nested field reject.
The valid byte-budget boundary is65549, one byte more rejects; exactly1024 TLVs
admit and1025 reject. Nested attributes share that budget with burst wrappers.
Unknown/duplicate/zero-length values remain recoverable. Original caller mutation,
drop, clones and serialization leave the accepted source unchanged.

These tests do not claim that arbitrary structurally admitted fields form a
conforming complete channel. Empty/duplicate/reserved raw values stay inspectable
without becoming accepted simulation settings. No bandwidth, grant, measured-CM
identity or measurement time is synthesized. The source profile/CMTS execution
validators and existing simulation schemas remain unchanged.

## Local checks

Host: Linux x86_64, Python3.10.12, Rust/Cargo1.85.1. No new dependency, system
installation, source checkout dependency, unsafe code or Python parser is added.
The existing optional adapter remains independent of core-only builds.

| Command | Outcome |
| --- | --- |
| `scripts/cargo-local.sh test -p morgana --test ucd_capture --locked` | Pass, five independent source/structure/resource/lifetime cases. |
| `scripts/cargo-local.sh run -p morgana --example inspect_ucd --locked` | Pass, actual standalone Rust consumer of both frozen byte fixtures. |
| `scripts/validate.sh` | Pass, 93 Rust tests, 5 Git-helper tests, 19 setup tests, formatting, examples/doctest targets and workspace Clippy; 24.86s. |
| `python3 -B tools/python/check_native.py` | Pass, 11 actual native runtime tests; 45.21s including cold native rebuild. Includes paired independent Rust capture-process output, structured categories/fields/offsets and original/export lifetimes. Existing simulation/parity/control cases remain passing. |
| `CARGO_TARGET_DIR=target/python scripts/cargo-local.sh clippy --manifest-path bindings/python/Cargo.toml --all-targets --locked -- -D warnings` | Pass, 19.90s; separate native formatting also passes. |
| `PYTHONPATH=target/python-tools python3 -B tools/python/build_wheel.py` | Pass, 66.06s: actual optimized CPython3.10/Linux x86_64 wheel; 170 entries, 79 attributed dependencies with retained full notices/hashes, all 11 native artifact tests pass. |

Private logs and original page renders remain under ignored
`docs/development/local/capture-formats/`. A source-only inspector does not alter
the admitted simulation configuration, job state, RNG or source position.
No commit, push, publication or installer lifecycle is executed.

The wheel remains a local host artifact at ignored
`target/python-wheels/morgana_ofdma-0.1.0-cp310-cp310-linux_x86_64.whl`.
It was rebuilt with the current inspector and tested through the actual native
exports, rather than inferred from the earlier synthetic-backend wheel. Other
Python/platform/source-distribution and installed behavior remain unverified.

Requested post-test cleanup completed after all consumers finished. Cargo removed
3676 root-dev files (632.8 MiB), 2000 native-dev files (975.9 MiB) and 896
native-release files (297.2 MiB); temporary native import trees were also removed.
All three Cargo profiles are empty. Source/index, toolchain/dependency downloads,
private validation logs and the tested wheel remain retained. The final
traceability review passes all 102 task IDs/dependencies/cycles and 59 requirement
links; every implemented architecture path exists.
