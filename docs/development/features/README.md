# Feature development records

The [selected Rust/Python namespace mapping](high-level-api/namespace-design.md) documents the thin facade rationale, exact Cargo owners and installed-wheel compatibility gates.

Engineering responsibilities organize this index; established feature directories
and their original paths remain intact. The [architecture](../../architecture.md),
[requirements baseline](../../requirements.md) and [official checklist](../checklist.md)
are authoritative. [DOC-ORG-01](../checklist.md#doc-org-01) records the documentation
reorganization; [acceptance and exact preservation counts](unified-backend/validation.md#doc-org-01-functional-documentation-acceptance)
do not imply new runtime acceptance.

Current functional records identify local owners, requirements, tasks and evidence.
Historical records keep their original findings, paths and limits; consult current
canonical task states before interpreting old pending/complete statements. Private
evidence stays under ignored `docs/development/local/` and is not linked as a shipped artifact.

New functional owners: [FEC](fec/design.md), [DSP](signal-processing/design.md) ([validation](signal-processing/validation.md)),
[OFDMA configuration](ofdma/configuration.md), [TX](ofdma/transmitter.md),
[RX](ofdma/receiver.md), [channel](ofdma/channel.md), [Pre-Eq](ofdma/pre-equalization.md),
[upstream PNM](pnm-measurement-inputs/upstream.md) and [paired APIs](high-level-api/upstream.md).

The [backend master prompt review](unified-backend/master-prompt-review.md) maps
the current instruction source to canonical requirements and tasks. Its
[API compatibility matrix](high-level-api/compatibility-matrix.md) records current
session operations and proposed namespaces; final operation/runtime coverage remains
with the open API and backend acceptance gates.

The [unified Qt Workbench](unified-qt-workbench/design.md) now owns the authorized
UI phase, including its [public-API boundary](unified-qt-workbench/api-gap-matrix.md)
and [official UIU tasks](../checklist.md#unified-qt-workbench-delivery).
Its [testing strategy](unified-qt-workbench/testing-strategy.md) distinguishes
focused commits, task acceptance and full integration gates.
Existing downstream history is retained. The [source UI design](unified-backend/references/extension-designs-ui.md)
is historical guidance; the unified phase still delivers one native application.

Reference navigation: [development status](../status.md),
[subordinate UI checklist](../ui-checklist.md),
[generated Rust/Python API reference](../../reference/api-reference.md) and
[its design owner](high-level-api/design.md#api-doc-01-generated-high-level-rust-and-python-references).
Workbench guides: [channel measurements](../../user/channel-measurements.md),
[closed-loop execution](../../user/closed-loop-workbench.md) and
[receiver instruments](../../user/receiver-workbench.md).

## FEC mechanisms and distinct code families

- **fec-shortening**: [design.md](fec-shortening/design.md) · [validation.md](fec-shortening/validation.md).
- **fec**: [design.md (active functional owner)](fec/design.md).
- **ncp-codec**: [architecture.md](ncp-codec/architecture.md).
- **plc-ldpc**: [architecture.md](plc-ldpc/architecture.md).
- **sparse-encoder**: [design.md](sparse-encoder/design.md).

## Signal processing and numerical conventions

- **channel-widths**: [design.md](channel-widths/design.md).
- **qam-completion**: [design.md](qam-completion/design.md) · [external-vectors.md](qam-completion/external-vectors.md) · [validation.md](qam-completion/validation.md).
- **qam-demapper-performance**: [design.md](qam-demapper-performance/design.md).
- **qam-mapping**: [architecture.md](qam-mapping/architecture.md).
- **qam256-coding-loop**: [architecture.md](qam256-coding-loop/architecture.md).
- **signal-processing**: [design.md (active functional owner)](signal-processing/design.md).

## Downstream OFDM, channels and simulation

- **carrier-layout**: [architecture.md](carrier-layout/architecture.md).
- **cell-interleaving**: [architecture.md](cell-interleaving/architecture.md).
- **cell-randomizer**: [architecture.md](cell-randomizer/architecture.md).
- **channel-runner**: [plan.md](channel-runner/plan.md) · [validation.md](channel-runner/validation.md).
- **channel-tilt**: [design.md](channel-tilt/design.md).
- **closed-loop-validation**: [design.md](closed-loop-validation/design.md) · [validation.md](closed-loop-validation/validation.md).
- **completion-group**: [design.md](completion-group/design.md) · [external-channel-vectors.md](completion-group/external-channel-vectors.md) · [validation.md](completion-group/validation.md).
- **file-payload-source**: [design.md](file-payload-source/design.md) · [implementation.md](file-payload-source/implementation.md).
- **impairment-group**: [plan.md](impairment-group/plan.md) · [validation.md](impairment-group/validation.md).
- **independent-channel-estimation**: [design.md](independent-channel-estimation/design.md) · [implementation.md](independent-channel-estimation/implementation.md).
- **lte-ingress**: [plan.md](lte-ingress/plan.md) · [validation.md](lte-ingress/validation.md).
- **paired-constellation**: [implementation.md](paired-constellation/implementation.md) · [plan.md](paired-constellation/plan.md).
- **payload-transport**: [plan.md](payload-transport/plan.md) · [validation.md](payload-transport/validation.md).
- **pilot-channel-estimation**: [implementation.md](pilot-channel-estimation/implementation.md) · [plan.md](pilot-channel-estimation/plan.md).
- **pilot-plc-signals**: [architecture.md](pilot-plc-signals/architecture.md).
- **plc-block-interleaving**: [architecture.md](plc-block-interleaving/architecture.md).
- **plc-completion**: [plan.md](plc-completion/plan.md) · [validation.md](plc-completion/validation.md).
- **plc-message-blocks**: [architecture.md](plc-message-blocks/architecture.md).
- **plc-randomization**: [architecture.md](plc-randomization/architecture.md) · [validation.md](plc-randomization/validation.md).
- **prbs-source**: [plan.md](prbs-source/plan.md) · [validation.md](prbs-source/validation.md).
- **receiver-acquisition**: [design.md](receiver-acquisition/design.md) · [validation.md](receiver-acquisition/validation.md).
- **receiver-configuration**: [plan.md](receiver-configuration/plan.md) · [validation.md](receiver-configuration/validation.md).
- **rf-power-budget**: [design.md](rf-power-budget/design.md).
- **run-infrastructure**: [plan.md](run-infrastructure/plan.md) · [validation.md](run-infrastructure/validation.md).
- **single-channel-transmission**: [architecture.md](single-channel-transmission/architecture.md).
- **symbol-packing**: [architecture.md](symbol-packing/architecture.md) · [design.md](symbol-packing/design.md).
- **waveform-direction**: [notes.md](waveform-direction/notes.md).
- **waveform**: [plan.md](waveform/plan.md) · [validation.md](waveform/validation.md).

## Upstream OFDMA

- **ofdma**: [channel.md (active functional owner)](ofdma/channel.md) · [configuration.md (active functional owner)](ofdma/configuration.md) · [pre-equalization.md (active functional owner)](ofdma/pre-equalization.md) · [receiver.md (active functional owner)](ofdma/receiver.md) · [transmitter.md (active functional owner)](ofdma/transmitter.md).

## PNM and measurement processing

- **pnm-measurement-inputs**: [README.md](pnm-measurement-inputs/README.md) · [profile-view.md](pnm-measurement-inputs/profile-view.md) · [upstream.md (active functional owner)](pnm-measurement-inputs/upstream.md).
- **pnm-reference-skill**: [design.md](pnm-reference-skill/design.md).
- **rxmer-designer**: [design.md](rxmer-designer/design.md) · [pnm-export-plan.md](rxmer-designer/pnm-export-plan.md) · [validation.md](rxmer-designer/validation.md).
- **rxmer-error-sweep**: [README.md](rxmer-error-sweep/README.md).

## Public Rust and Python APIs

- **high-level-api**: [design.md](high-level-api/design.md) · [requirements.md](high-level-api/requirements.md) · [upstream.md (active functional owner)](high-level-api/upstream.md) · [validation.md](high-level-api/validation.md).
- **python-adapter**: [design.md](python-adapter/design.md).

## Workbench and presentation

- **about-menu**: [design.md](about-menu/design.md).
- **channel-workbench**: [design.md](channel-workbench/design.md).
- **cl-notch-workbench**: [comparison-sim-1.json](cl-notch-workbench/comparison-sim-1.json) · [comparison-sim-1.md](cl-notch-workbench/comparison-sim-1.md) · [design.md](cl-notch-workbench/design.md).
- **contextual-help**: [design.md](contextual-help/design.md).
- **fec-monitor**: [design.md](fec-monitor/design.md).
- **mouse-zoom**: [design.md](mouse-zoom/design.md).
- **popup-close-controls**: [design.md](popup-close-controls/design.md).
- **profile-performance**: [design.md](profile-performance/design.md).
- **qam-markers**: [design.md](qam-markers/design.md).
- **qam-mer**: [design.md](qam-mer/design.md).
- **qam-receiver-mode**: [design.md](qam-receiver-mode/design.md).
- **qam-run-results**: [design.md](qam-run-results/design.md).
- **qt-closed-loop**: [design.md](qt-closed-loop/design.md) · [validation.md](qt-closed-loop/validation.md).
- **qt-source-organization**: [design.md](qt-source-organization/design.md).
- **readme-workbench-diagram**: [design.md](readme-workbench-diagram/design.md).
- **receiver-instruments**: [design.md](receiver-instruments/design.md).
- **receiver-workbench**: [design.md](receiver-workbench/design.md) · [validation.md](receiver-workbench/validation.md).
- **responsive-workbench-open**: [design.md](responsive-workbench-open/design.md).
- **restore-workbench-inputs**: [design.md](restore-workbench-inputs/design.md).
- **run-elapsed-time**: [design.md](run-elapsed-time/design.md).
- **rxmer-selection**: [design.md](rxmer-selection/design.md).
- **rxmer-workbench**: [design.md](rxmer-workbench/design.md) · [validation.md](rxmer-workbench/validation.md).
- **symbol-timing-popup**: [design.md](symbol-timing-popup/design.md).
- **terminology**: [design.md](terminology/design.md).
- **tooltip-delay**: [design.md](tooltip-delay/design.md).
- **transmitter-graphs**: [design.md](transmitter-graphs/design.md).
- **transmitter-spectrum**: [design.md](transmitter-spectrum/design.md) · [validation.md](transmitter-spectrum/validation.md).
- **transmitter-workbench**: [design.md](transmitter-workbench/design.md).
- **ui-copy**: [design.md](ui-copy/design.md).
- **user-guide-captures**: [design.md](user-guide-captures/design.md) · [png-audit.md](user-guide-captures/png-audit.md).
- **verification-panel**: [design.md](verification-panel/design.md).
- **visual-workspace**: [design.md](visual-workspace/design.md) · [plan.md](visual-workspace/plan.md) · [validation.md](visual-workspace/validation.md).
- **workbench-configuration**: [design.md](workbench-configuration/design.md).
- **workbench-examples**: [design.md](workbench-examples/design.md).

## Integration, requirements and engineering history

The frozen documentation plan/inventory preserve proposal-time wording. The
[current routing](unified-backend/source-inventory.json) and
[DOC-ORG-01 evidence](unified-backend/validation.md#doc-org-01-functional-documentation-acceptance)
record the approved execution and its documentation-only acceptance.

- **checklist-audit**: [plan.md](checklist-audit/plan.md) · [validation.md](checklist-audit/validation.md).
- **checklist-naming**: [plan.md](checklist-naming/plan.md).
- **development-workspace**: [plan.md](development-workspace/plan.md).
- **downstream-requirements**: [design.md](downstream-requirements/design.md) · [traceability.json](downstream-requirements/traceability.json) · [validation.md](downstream-requirements/validation.md).
- **engineering-guidance**: [design.md](engineering-guidance/design.md) · [source-inventory.json](engineering-guidance/source-inventory.json) · [validation.md](engineering-guidance/validation.md).
- **project-architecture**: [plan.md](project-architecture/plan.md).
- **project-rename**: [notes.md](project-rename/notes.md).
- **rust-only-signal-processing**: [architecture.md](rust-only-signal-processing/architecture.md).
- **simulation-documentation**: [plan.md](simulation-documentation/plan.md) · [validation.md](simulation-documentation/validation.md).
- **source-organization**: [architecture.md](source-organization/architecture.md).
- **unified-backend**: [baseline.md](unified-backend/baseline.md) · [design.md](unified-backend/design.md) · [feature-documentation-inventory.json](unified-backend/feature-documentation-inventory.json) · [feature-documentation-plan.md](unified-backend/feature-documentation-plan.md) · [fec-validation.md](unified-backend/fec-validation.md) · [reuse-review.md](unified-backend/reuse-review.md) · [scientific-source-review.json](unified-backend/scientific-source-review.json) · [source-inventory.json](unified-backend/source-inventory.json) · [source-review.md](unified-backend/source-review.md) · [validation.md](unified-backend/validation.md).
- **upstream-skills**: [design.md](upstream-skills/design.md) · [source-inventory.json](upstream-skills/source-inventory.json) · [validation.md](upstream-skills/validation.md).

## Development tooling and delivery

- **automatic-updates**: [design.md](automatic-updates/design.md).
- **build-cache**: [design.md](build-cache/design.md).
- **command-timing**: [design.md](command-timing/design.md).
- **development-setup**: [design.md](development-setup/design.md).
- **docs-only-push**: [design.md](docs-only-push/design.md).
- **example-layout**: [design.md](example-layout/design.md) · [validation.md](example-layout/validation.md).
- **parallel-local-tests**: [design.md](parallel-local-tests/design.md).
- **quick-local-tests**: [design.md](quick-local-tests/design.md).
- **release-asset-names**: [design.md](release-asset-names/design.md).
- **release-resume**: [design.md](release-resume/design.md).
- **rust-logging**: [architecture.md](rust-logging/architecture.md).
- **security-checks**: [design.md](security-checks/design.md).
- **ubuntu-desktop**: [design.md](ubuntu-desktop/design.md) · [local-validation.md](ubuntu-desktop/local-validation.md).
- **windows-desktop**: [design.md](windows-desktop/design.md) · [validation.md](windows-desktop/validation.md).

- **domain-limits**: [design and acceptance](domain-limits/design.md).

## Historical source evidence archive

All 12 records below remain byte-identical at their original locations. Functional
owners above adapt current contracts; source checkmarks/commands never become MERLIN passes.

- **unified-backend/references**: [backend-stages-validation.md](unified-backend/references/backend-stages-validation.md) · [capture-formats-validation.md](unified-backend/references/capture-formats-validation.md) · [extension-designs-channel.md](unified-backend/references/extension-designs-channel.md) · [extension-designs-ranging.md](unified-backend/references/extension-designs-ranging.md) · [extension-designs-receiver.md](unified-backend/references/extension-designs-receiver.md) · [extension-designs-ui.md](unified-backend/references/extension-designs-ui.md) · [foundation-validation.md](unified-backend/references/foundation-validation.md) · [ldpc-source-audit-validation.md](unified-backend/references/ldpc-source-audit-validation.md) · [remaining-delivery-validation.md](unified-backend/references/remaining-delivery-validation.md) · [rxmer-capture-validation.md](unified-backend/references/rxmer-capture-validation.md) · [upstream-stage-contracts.md](unified-backend/references/upstream-stage-contracts.md) · [us-configuration-json-coverage.md](unified-backend/references/us-configuration-json-coverage.md).

[OFDMA implementation and validation](ofdma/design.md) records the active integration units.
