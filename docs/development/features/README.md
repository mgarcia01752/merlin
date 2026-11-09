# Feature development records

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
| carrier layout | [architecture](carrier-layout/architecture.md) |
| cell interleaving | [architecture](cell-interleaving/architecture.md) |
| cell randomizer | [architecture](cell-randomizer/architecture.md) |
| channel runner | [plan](channel-runner/plan.md) · [validation](channel-runner/validation.md) |
| checklist audit | [plan](checklist-audit/plan.md) · [validation](checklist-audit/validation.md) |
| checklist naming | [plan](checklist-naming/plan.md) |
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
| pnm measurement inputs | [README](pnm-measurement-inputs/README.md) |
| prbs source | [plan](prbs-source/plan.md) · [validation](prbs-source/validation.md) |
| project architecture | [plan](project-architecture/plan.md) |
| project rename | [notes](project-rename/notes.md) |
| qam completion | [QAM-02/03 design](qam-completion/design.md) · [validation](qam-completion/validation.md) · [QAM-04 evidence needed](qam-completion/external-vectors.md) |
| qam mapping | [architecture](qam-mapping/architecture.md) |
| qam256 coding loop | [architecture](qam256-coding-loop/architecture.md) |
| receiver configuration | [plan](receiver-configuration/plan.md) · [validation](receiver-configuration/validation.md) |
| run infrastructure | [plan](run-infrastructure/plan.md) · [validation](run-infrastructure/validation.md) |
| rust logging | [architecture](rust-logging/architecture.md) |
| rust only signal processing | [architecture](rust-only-signal-processing/architecture.md) |
| rxmer error sweep | [README](rxmer-error-sweep/README.md) |
| simulation documentation | [plan](simulation-documentation/plan.md) · [validation](simulation-documentation/validation.md) |
| single channel transmission | [architecture](single-channel-transmission/architecture.md) |
| source organization | [architecture](source-organization/architecture.md) |
| symbol packing | [architecture](symbol-packing/architecture.md) |
| visual workspace (planned) | [plan](visual-workspace/plan.md) |
| waveform | [plan](waveform/plan.md) · [validation](waveform/validation.md) |
| waveform direction | [notes](waveform-direction/notes.md) |
