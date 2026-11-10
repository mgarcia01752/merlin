# Downstream completion-group validation

Current scope override: on2026-11-09 the user removed external interoperability
from minimum acceptance. QA-05 now records completed internal closed-loop tests;
see [scope and evidence](../closed-loop-validation/design.md). External requirements
below describe the earlier batch, not current acceptance gates.

Historical batch record. For the later WAVE-RX-05 completion and current QAM
checks, see [receiver acquisition](../receiver-acquisition/design.md) and
[QAM reference review](../qam-completion/external-vectors.md#pypnm-text-table-review).

Host: Ubuntu22.04 x64; project-local Rust/Cargo1.85.1. VERSION remains0.1.0.
Nothing was committed, pushed, tagged, published, installed or system-provisioned.
At that earlier milestone QAM-04 was excluded/deferred. The preceding cell/QAM-symbol terminology edits
were preserved. Workflow files are prepared locally only.

## Task status and evidence

| Task | Local result | Remaining evidence |
| --- | --- | --- |
| NCP-03 | Implemented bounded dynamic full-word data/NCP generations; real PLC DPD recovery; actual payload/NCP transport across a change at both spacings, zero recovered bit errors | Further same-parity replacement needs reacquisition; OPT and dynamic shortened words are outside the supported path |
| LAYOUT-03 | Implemented generated additional pilots; both FFT sizes, density, exclusions/PLC/poor bins, edge guards, isolated centers, predefined coverage and repeatability pass | No spectral-mask or field-device certification claimed |
| WAVE-RX-05 | Implemented bounded received-only timing/fractional-CFO/pilot-phase acquisition; noisy smooth-channel tests and corrected payload I/Q checks pass | Parent stays open for integer-carrier/unknown-numerology and broader-channel verification |
| INPUT-02 | Explicit producer/revision allowlist, shared Rust API and request schema2; actual0.3.6 exports, additive fields, malformed/drifted requests and legacy CLI tests pass | New producer versions require deliberate fixture verification |
| Former external QA-05 proposal | Bounded provenance/hash admission and integrity/path tests pass | Now optional; current QA-05 closes through internal source/control/payload comparisons |
| QA-06 | Native workflow/matrix and CLI smoke tooling prepared; YAML parsed locally | Windows/macOS execution waits for later workflow push |
| QA-07 | CLI archive/DEB built locally; metadata/checksums/resources/licenses and native binary smoke checked | PKG/MSI execution, all native install/upgrade/uninstall lifecycles and publication remain unverified |

## Checks

Focused Rust tests cover eleven new cases across dynamic_profiles (3),
continuous_pilot_placement (3), nonideal_acquisition (2), pnm_json_contract (2)
and the additional CLI import-contract case (1). Existing fixed-path rejection,
transport, import and waveform suites are retained. The timing test permits two
samples of CP-boundary uncertainty under noise; the CFO error is below2Hz and
held-out pilot/data error ratios are below0.01 for the specified test conditions.
These bounds describe those experiments, not universal receiver performance.

Six packaging/release helper tests and one synthetic external-admission test
cover version mismatch, no-clobber, symlink rejection, license/resource staging,
checksum collection/tampering, tagged documentation links and bounded vector
admission. Dependency notices were collected from88 pinned native Cargo metadata
packages; include full source notices and the shared upstream MIT notice for
termination_attrib. Tooling performs no DSP calculations.

The full local gate includes version/helper checks, Bash/ShellCheck, formatting,
warnings-denied Clippy, workspace tests, fixture-preparation tests, no-default
feature tests, release CLI build and actual channel/import/file/probe smokes.
The final-source full local gate passed, including the new component/CLI tests.
Its retained local log is `docs/development/local/completion-group/final-gate.log`.

Both release/CI YAML files parse locally. No actionlint binary is available and
GitHub has not executed them. Release and scientific-testing skills validate
through their canonical paths and both discovery links. Changed Markdown target
links and Git whitespace checks are reviewed at completion.

## Native assets

Generated material stays ignored under `docs/development/local/completion-group/`.
The final archive/DEB were rebuilt from the tested release binary, checksummed,
and the extracted DEB binary passed native CLI smoke checks without installation.
The Linux archive and DEB include the CLI, public examples/guides, root LICENSE/
NOTICE, adapted BCH MIT notice, complete PyPNM QAM license/attribution and Cargo
dependency notice manifest/full texts. Archive inspection excludes local private
inputs, PDFs, toolchains and build outputs. Package generation installs nothing.
Independent checksums detect corruption, not publisher authentication. The native
packages are unsigned; per-OS user instructions and release links document this.

## Normative source review

Pilot density/merged-frequency formulas were checked visually against the local
PHY I20 PDF p.170; edge/coverage/disturbance rules use pp.169–171. NCP C/N/U and
announcement pattern use PHY I20 §8.3.4.1 Table53, pp.180–182. Private PDF renders
remain ignored. No local specification or capture is included as a release input.
