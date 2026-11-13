# Feature development records

- [RxMER Designer](rxmer-designer/design.md) · [validation](rxmer-designer/validation.md)

- [Graphical transmitter test points](transmitter-graphs/design.md)

- [Responsive workbench loading](responsive-workbench-open/design.md)
- [RxMER capture selection and CWB capacity](rxmer-selection/design.md)

- [Theme-aware README workbench diagram](readme-workbench-diagram/design.md)

- [Transmitter workbench (UI-TX-03)](transmitter-workbench/design.md)

This index includes ongoing feature plans and historical design/validation
records moved from the former root workspace. The [paired constellation](paired-constellation/implementation.md) group
implements CONST-01 through CONST-05 using supplied H/reciprocal correction. Historical records describe the state at the
time each task was performed. Earlier statements about pending stages are
historical; consult the [current checklist](../checklist.md),
[status](../status.md) and [canonical architecture](../../architecture.md).

Continue development here: add or update `features/<feature>/plan.md` and related
reviewable design/validation notes. Keep each algorithm's maintained user guide
under `docs/user/`; do not turn historical reports into competing architecture.
Private source documents and generated evidence belong in the matching ignored
`docs/development/local/<feature>/` directory. Those local artifacts are absent
from a fresh clone and are deliberately not Markdown link targets.

| Work area | Records |
| --- | --- |
| local command timing | [design](command-timing/design.md) |
| parallel local tests | [design](parallel-local-tests/design.md) |
| user-facing UI wording | [design](ui-copy/design.md) |
| Ubuntu desktop/update notification | [design](ubuntu-desktop/design.md) |
| workbench example bundles | [design](workbench-examples/design.md) |
| recovered-data panel | [design](verification-panel/design.md) |
| mouse chart navigation | [design](mouse-zoom/design.md) |
| receiver instruments | [design](receiver-instruments/design.md) |
| popup close controls | [design](popup-close-controls/design.md) |
| carrier layout | [architecture](carrier-layout/architecture.md) |
| cell interleaving | [architecture](cell-interleaving/architecture.md) |
| cell randomizer | [architecture](cell-randomizer/architecture.md) |
| channel runner | [plan](channel-runner/plan.md) · [validation](channel-runner/validation.md) |
| checklist audit | [plan](checklist-audit/plan.md) · [validation](checklist-audit/validation.md) |
| checklist naming | [plan](checklist-naming/plan.md) |
| documentation-only push | [design](docs-only-push/design.md) |
| development workspace | [plan](development-workspace/plan.md) |
| fec shortening | [design](fec-shortening/design.md) · [validation](fec-shortening/validation.md) |
| file payload source | [design](file-payload-source/design.md) · [implementation](file-payload-source/implementation.md) |
| impairment group | [plan](impairment-group/plan.md) · [validation](impairment-group/validation.md) |
| lte ingress | [plan](lte-ingress/plan.md) · [validation](lte-ingress/validation.md) |
| ncp codec | [architecture](ncp-codec/architecture.md) |
| paired constellation | [plan](paired-constellation/plan.md) · [implementation](paired-constellation/implementation.md) |
| payload transport | [plan](payload-transport/plan.md) · [validation](payload-transport/validation.md) |
| receiver channel estimation | [plan](pilot-channel-estimation/plan.md) · [implementation](pilot-channel-estimation/implementation.md) |
| pilot plc signals | [architecture](pilot-plc-signals/architecture.md) |
| plc block interleaving | [architecture](plc-block-interleaving/architecture.md) |
| plc completion | [plan](plc-completion/plan.md) · [validation](plc-completion/validation.md) |
| plc ldpc | [architecture](plc-ldpc/architecture.md) |
| plc message blocks | [architecture](plc-message-blocks/architecture.md) |
| plc randomization | [architecture](plc-randomization/architecture.md) · [validation](plc-randomization/validation.md) |
| pnm measurement inputs | [README](pnm-measurement-inputs/README.md), [visual DPD](pnm-measurement-inputs/profile-view.md) |
| prbs source | [plan](prbs-source/plan.md) · [validation](prbs-source/validation.md) |
| project architecture | [plan](project-architecture/plan.md) |
| Python adapter (after Qt, planned) | [API-01 class proposal](python-adapter/design.md) |
| project rename | [notes](project-rename/notes.md) |
| qam completion | [QAM-02/03 design](qam-completion/design.md) · [validation](qam-completion/validation.md) · [QAM-04 evidence needed](qam-completion/external-vectors.md) |
| qam mapping | [architecture](qam-mapping/architecture.md) |
| qam256 coding loop | [architecture](qam256-coding-loop/architecture.md) |
| receiver configuration | [plan](receiver-configuration/plan.md) · [validation](receiver-configuration/validation.md) |
| run infrastructure | [plan](run-infrastructure/plan.md) · [validation](run-infrastructure/validation.md) |
| security checks | [design](security-checks/design.md) |
| rust logging | [architecture](rust-logging/architecture.md) |
| rust only signal processing | [architecture](rust-only-signal-processing/architecture.md) |
| rxmer error sweep | [README](rxmer-error-sweep/README.md) |
| simulation documentation | [plan](simulation-documentation/plan.md) · [validation](simulation-documentation/validation.md) |
| single channel transmission | [architecture](single-channel-transmission/architecture.md) |
| source organization | [architecture](source-organization/architecture.md) |
| symbol packing | [architecture](symbol-packing/architecture.md), [build hygiene](symbol-packing/design.md) |
| Development setup | [installer and pinned source design](development-setup/design.md) |
| Qt simulation workbench (UI-01 partial) | [design](visual-workspace/design.md) · [validation](visual-workspace/validation.md) · [plan](visual-workspace/plan.md) · [UI checklist](../ui-checklist.md) |
| Transmitter spectrum | [UI-TX-01/02 design](transmitter-spectrum/design.md) |
| waveform | [plan](waveform/plan.md) · [validation](waveform/validation.md) |
| waveform direction | [notes](waveform-direction/notes.md) |
| downstream completion group | [design](completion-group/design.md) · [validation](completion-group/validation.md) · [external vector needs](completion-group/external-channel-vectors.md) |
| synthetic channel widths | [design and validation](channel-widths/design.md) |
| receiver acquisition | [WAVE-RX-05 design](receiver-acquisition/design.md) · [validation](receiver-acquisition/validation.md) |
| closed-loop minimum acceptance | [design](closed-loop-validation/design.md) · [test evidence](closed-loop-validation/validation.md) |

Build tooling proposal: [BUILD-01–04 content-based cache](build-cache/design.md).

Channel Estimate/RxMER workbench: [design](rxmer-workbench/design.md) · [validation](rxmer-workbench/validation.md) · [user guide](../../user/channel-measurements.md).

Receiver Workbench navigation: [design](receiver-workbench/design.md) · [validation](receiver-workbench/validation.md) · [user interaction](../../user/receiver-workbench.md).

Qt closed-loop execution and workbench persistence: [design](qt-closed-loop/design.md) · [validation](qt-closed-loop/validation.md) · [user guide](../../user/closed-loop-workbench.md).

- [Current native user-guide captures (UI-DOC-01)](user-guide-captures/design.md)

- [Online terminology guide (UI-DOC-02)](terminology/design.md)

- [Basic Channel Simulator staging (UI-CH-01, implemented)](channel-workbench/design.md)

- [Additional channel tilt (WAVE-CH-TILT-01, implemented)](channel-tilt/design.md)

- [Channel conditioning (UI-RF-01) and RF power follow-ups (RF-POWER-01)](rf-power-budget/design.md)

- [Qt source organization (UI-ORG-01 / UI-ORG-02)](qt-source-organization/design.md)

- [Receiver profile performance and independent profile runs](profile-performance/design.md)

- [Automatic Ubuntu updates](automatic-updates/design.md): UI-UPDATE-02.

- [WB configuration and shared CI](workbench-configuration/design.md): UI-WB-CONFIG-01, UI-WB-CONTROLS-01, UI-BUTTONS-01, CI-CONSOLIDATE-01 and CI-DESKTOP-APT-01.

- [Readable FEC Monitor](fec-monitor/design.md): UI-FEC-02.

- [Run elapsed time and profile progress](run-elapsed-time/design.md): UI-RUN-TIME-01.

- [Exact indexed QAM demapping](qam-demapper-performance/design.md): QAM-PERF-01.

- [Received QAM scatter dots](qam-markers/design.md): UI-QAM-DOTS-01.

- [Contextual help and spectrum details](contextual-help/design.md): UI-HELP-01, UI-SA-DETAILS-01.

- [Interrupted release recovery](release-resume/design.md): RELEASE-RESUME-01.

- [Quick local test subset](quick-local-tests/design.md): DEV-TEST-QUICK-01.

- [Release asset filenames](release-asset-names/design.md): RELEASE-ASSET-01.

- [About and dropdown menu cleanup](about-menu/design.md): UI-ABOUT-01.

- [Full symbol timing popup](symbol-timing-popup/design.md): UI-SA-TIMING-02.

- [Calculated constellation MER](qam-mer/design.md): UI-QAM-MER-01.

- [Restore workbench inputs](restore-workbench-inputs/design.md): UI-INPUT-RESTORE-01.

- [Run-specific QAM results and TX-reference MER](qam-run-results/design.md): UI-QAM-RUNS-02.

- [Independent receiver estimation through a channel with memory](independent-channel-estimation/design.md): CH-EST-RX-07..12 (implemented for bounded static channels).

- [Windows desktop MSI](windows-desktop/design.md): native UI packaging, icons,
  Windows adapter boundaries and CI lifecycle gates; macOS deferred.

- [CLI versus desktop examples](example-layout/design.md): EXAMPLES-01,
  documented use cases and separate desktop/CLI packaging inventories.
