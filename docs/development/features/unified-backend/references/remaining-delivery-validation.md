> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/remaining-delivery/validation.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# Remaining delivery evidence

Status is updated only as each official gate passes. The 2026-11-20 work extends
the prior synthetic/inspection acceptance; it does not imply hosted execution,
publication, interoperability or GUI implementation.

## PREEQ-FORMAT-01

Inspected original CM-OSSI I27 pp189/191 tables77/78 and the full pp187–192
definitions locally. Source SHA-256:
`29f0ebf32c4c38610b1b9c10a418ffff4045c34def0ffbe4e85fc674cb5b91de`.
PHY I20 §7.4.17 pp106–108 establishes reset/normalization, Set replacement and
Adjust multiplication using the corresponding probe bank. The file alone has
no Set/Adjust/base/status fields: they are required external context.

`scripts/cargo-local.sh test -p morgana --test pre_eq_capture --locked` passed
2 tests. Independent literal expected values cover unity, 1.125-j0.25, both
signed endpoints and one-LSB signs for each scale; malformed lengths/grids,
every truncated prefix, unsupported major/type/spacing, vendor minor bytes,
bounded empty observations and immutable originals are covered. Fixtures are
authored from the documented layouts and fixed-point notation; no production
serializer, modem recordings or external interoperability vectors were used.

## Capture model admission

Eight focused Rust tests pass for literal UCD semantics, ordered
multipart reconstruction, association failure atomicity, power-domain playback,
snapshot/initialize/adjust admission and actual full source recovery from the
authored captured-plan fixture. Actual native parity passes17 tests, including all
measured power policies, failed admission atomicity and cancelled provenance.
Final aggregate/package gates below pass. Both25/50kHz captures execute real
Rust/native experiments; the4K fixture includes mixed and zero loading. Tests
cover two explicitly based updates, an older corresponding probe, initial/default/
snapshot/Set/Adjust semantics, independent Pre-EQ without invented CCAP fields,
interior exclusion zeros and rejection of nonzero excluded updates. Zero excluded
records remain in exact source bytes; only their derived update addresses are omitted.
Early test corrections fixed a test fragment split inside a TLV and a canonical
JSON comparison that incorrectly treated omitted optional fields as exact source
text. Exact original plan bytes are tested separately and remain unchanged.

## Distribution and platform boundaries

Root engineering guidance includes adopted private instructional material whose
redistribution permission is unresolved. Scientific distributions exclude those
paths and retain all actual scientific/dependency notices. Public repository or
guidance redistribution remains unapproved. Local artifacts are not publication.

## Aggregate API and scientific acceptance

On Ubuntu22.04 amd64, pinned Rust1.85.1 and CPython3.10:

- `scripts/validate.sh`:109 Rust tests,5 Git-helper,19 setup and3 package/license
  tests; formatting, examples/doctests and workspace Clippy pass.
- `python3 -B tools/python/check_native.py`:17 actual native runtime tests pass,
  including independent Rust-process comparisons for both measured numerologies,
  original source/control provenance, rejected load state, owned results, all
  measured power policies and cancelled provenance. Existing pause/Busy/SIGINT/
  interpreter-release/FEC compatibility tests remain passing.
- Native `cargo fmt --manifest-path bindings/python/Cargo.toml -- --check` and
  locked all-target Clippy with `-D warnings` pass through the project wrapper.
- `PYTHONPATH=target/python-tools python3 -B tools/python/build_wheel.py`:
  actual optimized artifact passes the same17 runtime tests;170 archive entries,
  79 attributed host-reachable dependencies, exact full-license/notices hashes.
- The scientific source archive was extracted under an isolated generated tree
  without Git, skills, private evidence or another project's source. Using only
  the retained standard registry/toolchain caches, offline locked workspace tests
  pass all109 cases; its actual captured-plan example recovers source. Package
  member/hash verification reports205 entries. The first package trial failed
  because its Cargo-declared LDPC oracle was absent; the reviewed allowlist now
  includes that actual scientific file, and the complete extraction gate passes.

These close WAVE-CH-CAPTURE-01 and API-01 for the documented source-derived
adapter profiles and supported joint models, not arbitrary vendor/exporter files.
The supplied generic context remains original; actual UCD-projected scientific
configuration and measured provenance retain their distinct report fields.
Provenance serialization borrows the validated candidate and has a separate
16MiB bound checked before session replacement; malformed/oversized originals
never become silently accepted defaults. Receiver truth isolation remains intact.

Reviewed artifacts, retained under ignored target directories:

| Artifact | SHA-256 |
| --- | --- |
| `target/python-wheels/morgana_ofdma-0.1.0-cp310-cp310-linux_x86_64.whl` | `2f649af3301a1e6a6bdcaa5b8ee2c76cc3444a3544adeecab44593848e6376cc` |
| `target/source-packages/morgana-0.1.0-scientific-final.tar.gz` | `bb20e3e31aaceb8508d2d0b638e190cd43536129f12cdd1bb58f87c72658a097` |

Both are local validation artifacts, not installed/published products. The source
manifest matches the actual current allowlisted files byte-for-byte. Root
instructions/development plans are deliberately outside its distribution scope.
The final archive refresh corrects two inspection-guide status paragraphs; exact
manifest comparison proves every scientific/build/test/fixture input is identical
to the205-entry archive that passed extracted109-test and actual-run acceptance.
No Cargo re-test was needed for those documentation-only changes.
See the [distribution inventory](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/reference/distribution.md). Exact
license selections preserve required Unicode/LLVM terms; unknown expressions
fail review. No GPL presentation implementation or external checkout is bundled.

## Fresh target setup

SETUP-04 ran actual normal-user provisioning in two independent QEMU guests,
each2 virtual CPUs/2048MiB, software TCG (no shared host kernel/KVM). Downloaded
official [Ubuntu cloud images](https://cloud-images.ubuntu.com/) were verified
against their SHA256SUMS and detached signatures using the installed Ubuntu
cloud-image keyring. Signing fingerprint:
`D2EB44626FDDC30B513D5BB71A5D6C4C7DB87C81`.

| Declared target | Image SHA-256 | Actual result |
| --- | --- | --- |
| Ubuntu22.04 amd64 | `77d19e53656f87c95f1f6c6be59c789083b3f77fa08f9d88011ee04085c61f87` | Fresh check reports missing build-essential/local Rust; real apply, subsequent checks, repeated apply and offline metadata pass; success markers and clean guest power-off. |
| Ubuntu24.04 amd64 | `04e8f2461a16a44e78a370284471f29616cdd9baf1893df25807b71f985b6c94` | Same actual fresh/installed/repeated lifecycle passes with its own package/toolchain state; success markers and clean guest power-off. |

Each fresh guest uses user `morgana`, not root, for the installer. A loopback-only
temporary NoCloud seed server supplies an exact repository snapshot; no source
checkout/caches are mounted into the guest. Before apply, `--check` returns1
and `--dry-run` returns0 without creating local roots. Explicit apply installs
only missing declared apt prerequisites through sudo and checksum-pinned local
rustup/Rust1.85.1/rustfmt/Clippy, then fetches locked host-target dependencies.
Afterwards `--check` returns0, repeated apply returns0, a final check returns0
and `cargo metadata --locked --offline --no-deps` succeeds. Both console logs
contain `MORGANA_SETUP_PASS` and `MORGANA_VM_SUCCESS` before power-off.

Harness: QEMU virtio disk/network, no inbound port forwarding,12GiB sparse
overlay, `-smbios type=1,serial=ds=nocloud-net;s=http://10.0.2.2:61785/` and
cloud-init normal-user commands. Seed usage follows the
[NoCloud reference](https://docs.cloud-init.io/en/latest/reference/datasources/nocloud.html).
The compatibility `nocloud-net` alias is deprecated on the current Noble image;
both source-image cloud-init versions actually accepted it. No host apt transaction,
toolchain/shell-profile modification, global application install or other-project
change occurred. Provisioning validates setup, not native GUI installation or
full scientific runtime on each guest. The canonical installer files and locks
are unchanged by this work, so the exercised setup implementation is the final one.

## Design gates and foundation delivery

DEV-UI-PLAN-01, RX-EXT-01, CH-EXT-02 and ARCH-RANGING-01 close only their stated
design criteria. Reviewed records define respective owners/planned files, pinned
source applicability, algorithms/reference planes, finite budgets, lifecycle,
failure cases and independent future acceptance. UI and wider waveform/model
execution stay unsupported; no empty executable stubs were added.

The required Master CI definition reuses the actual local validation/native/wheel/
source owners on the two declared Ubuntu targets. Its checkout reference was
verified against the official action's Git tag; permissions are read-only and
jobs/time/concurrency bounded. Local YAML parsing and equivalent local commands
pass. No hosted execution result or remote change is claimed. GitHub's
[secure-use guidance](https://docs.github.com/en/actions/reference/security/secure-use)
supports pinning action commits and restricted token permissions.

FOUNDATION-03 acceptance is the independent local scientific delivery and required
CI definition, not publication, hosted execution or Windows/macOS lifecycles.
Those are outside this accepted delivery and remain explicitly unverified.
Requirements, architecture, supported API inventory, runnable paired guides,
examples and artifact scope are reconciled; earlier feature records retain their
historical evidence and link the subsequent completion record.

## Cleanup and final handoff

After every validation consumer finished, Cargo cleanup removed:

- Root dev profile:4117 files,724.1MiB.
- Optional native dev profile:2029 files,1003.1MiB.
- Optional native release profile:896 files,299.6MiB.
- Isolated extracted-source dev profile:3239 files,574.4MiB.

All three root/native build profiles have zero remaining files. Operation-owned
native import trees, extracted-source trees, superseded test exports and both
stopped VM disk/base-image files were removed. The loopback seed server was
stopped. Original inputs, validation/reference logs, VM console/seed/checksum/
signature evidence, downloaded Cargo sources/toolchains, validation tools and
the final tested wheel/source artifacts were retained. No other checkout or
user-selected export was cleaned. The filesystem reports51GiB available after
cleanup; this is a current host observation, not a fixed required capacity.

Final review covers staged/unstaged/untracked changes, local paths/anchors,
requirement coverage and checklist dependency closure. The fourteen formerly
unfinished IDs are checked only for their stated acceptance scopes. Future GUI,
wider receiver/channel and ranging execution require implementation acceptance;
their completed design gates do not advertise runtime support. No commit, push,
tag, publication or host package installation was executed; the user's Git index
and existing HEAD are preserved. This closes DEV-HANDOFF-01.

Final navigation/traceability review:20 skills and1087 local links pass the
repository gate;758 local fragment anchors resolve. All59 requirement anchors
are linked;65 explicit dependency lists have no missing IDs or cycles. Comparison
against the existing HEAD proves exactly the original fourteen unfinished task
IDs transitioned to checked, with no task IDs dropped or new unchecked records.
