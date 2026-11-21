# OFDMA channel, reference planes and observations

Active feature detail, subordinate to [canonical architecture](../../../architecture.md#unified-channel-and-measurement-reference-planes).
The [requirements baseline](../../../requirements.md) owns mandatory behavior; the
[official checklist](../../checklist.md) owns completion. [DOC-ORG-01](../../checklist.md#doc-org-01) accepts
documentation only. Current integration evidence is never inferred from source checkmarks.

## Contract and current support

Local channel execution remains planned under INT-US-CH-01; measured association/
playback additionally requires INT-PNM-02. The retained source results are historical.
Direction-specific channel state/admission remains separate from shared numerical
kernels and from the accepted downstream channel implementation.

The frequency-domain observation planes are `Y[k] = X[k] C[k] H[k] + N[k]`:
X is transmitted before correction, C is applied transmitter correction, H is the
physical response, N is output-referred impairment and Y is received observation.
Known received pilots estimate effective C×H. A corrective bank alone is not a
measurement of H; an explicit inverse-C hypothesis would be a separately tagged model.

## Ordered stage boundaries

| Stage | Input → output and invariant |
| --- | --- |
| CH-01 | Indexed samples → validated admitted blocks; preserve rate, origin, finite values, guards/tails and continuity. |
| CH-02 | Admitted samples → exact unity transport; preserve bit patterns/signed zeros and consume no RNG. |
| CH-03 | Transport → causal response output with carried FIR state and exact tail/drain. |
| CH-04 | Response → output-referred AWGN or admitted measured residuals, with separate configured/observed metrics. |
| CH-05 | Impaired stream → integer timing/constant phase/CFO using absolute sample state. |
| CH-06 | Indexed output → receiver delivery and passive bounded observations carrying run/configuration/coverage identity. |

Nonunity order is causal FIR → residual/noise → integer timing/phase/CFO. Initial
RxMER residual synthesis admits only unity H and every scheduled C bank, with timing
disabled. Preflight future updates before consuming source. Do not relax this source
restriction merely because downstream supports a different composition.

## State, measured semantics and limits

FIR memory, seeded draws, phase/sample coordinates and drain survive arbitrary chunk
boundaries. Rejected calls, preview, observers and exhausted detail budgets must not
advance hidden scientific state. Total work/retained observations remain bounded by
accepted requests; missing measurements stay unknown, not zero residual noise.

Measured RxMER is an observation at its declared plane. Its association, analytical
screening and modeled residual realization remain separate operations. Preserve
available/excluded grids, timestamps, cohort and policy coverage. Pilot/RxMER metrics
must say which samples/reference plane and normalization they actually summarize.

Broader C/H/RxMER composition, inverse-C response inference, fractional clock and
phase-noise models remain deferred in INT-DEFER-01. Their design does not enable
execution; no private source capture is copied here.

## Planned acceptance and norms

Verify bit-exact unity, hand convolution, impulse/tail coordinates, output noise
variance, signed phase/CFO, chunk equivalence, invalid-call atomicity and passive
observer invariance. Measured tests additionally exercise complete schedule rejection,
compatibility, partial coverage and explicit unsupported combinations.

PHY 3.1 I20 §7.4.17 and §9.4.8 distinguish corrective/probe and upstream RxMER planes.
Dependent OSSI capture interpretation retains its own pinned source; the PHY is not
alone a binary-format definition. Independent source and local integrated evidence
must remain separate.

## Requirement, task and implementation traceability

Each task link contains its authoritative acceptance criteria. Existing tests are
linked; nonexistent local paths are explicitly planned, not passing evidence.

| Stable requirement | Delivery / acceptance | Implementation and test owner |
| --- | --- | --- |
| [REQ-US-CH-01](../../../requirements.md#req-us-ch-01) | [INT-US-CH-01](../../checklist.md#int-us-ch-01) | Module: `crates/ofdma/src/channel/stream.rs` (planned); Test: `crates/ofdma/tests/channel_models.rs` (planned) |
| [REQ-US-CH-02](../../../requirements.md#req-us-ch-02) | [INT-US-CH-01](../../checklist.md#int-us-ch-01) | Module: `crates/ofdma/src/channel/stream.rs` (planned); Test: `crates/ofdma/tests/channel_models.rs` (planned) |
| [REQ-US-CH-03](../../../requirements.md#req-us-ch-03) | [INT-US-CH-01](../../checklist.md#int-us-ch-01) | Module: `crates/ofdma/src/channel/stream.rs` (planned); Test: `crates/ofdma/tests/channel_models.rs` (planned) |
| [REQ-US-CH-04](../../../requirements.md#req-us-ch-04) | [INT-US-CH-01](../../checklist.md#int-us-ch-01) | Module: `crates/ofdma/src/channel/residual.rs` (planned); Test: `crates/ofdma/tests/rxmer_channel.rs` (planned) |
| [REQ-US-CH-05](../../../requirements.md#req-us-ch-05) | [INT-US-CH-01](../../checklist.md#int-us-ch-01) | Module: `crates/ofdma/src/channel/stream.rs` (planned); Test: `crates/ofdma/tests/channel_models.rs` (planned) |
| [REQ-US-CH-06](../../../requirements.md#req-us-ch-06) | [INT-US-CH-01](../../checklist.md#int-us-ch-01) | Module: `crates/ofdma/src/instruments/observer.rs` (planned); Test: `crates/ofdma/tests/matched_metrics.rs` (planned) |

Related delivery gates: [INT-US-CH-01](../../checklist.md#int-us-ch-01), [INT-PNM-02](../../checklist.md#int-pnm-02), [INT-DEFER-01](../../checklist.md#int-defer-01).

## Supporting records and validation evidence

- [unified-backend/references/upstream-stage-contracts.md#ch-01-input-admission](../unified-backend/references/upstream-stage-contracts.md#ch-01-input-admission).
- [unified-backend/references/upstream-stage-contracts.md#ch-06-output-and-observations](../unified-backend/references/upstream-stage-contracts.md#ch-06-output-and-observations).
- [unified-backend/references/extension-designs-channel.md](../unified-backend/references/extension-designs-channel.md).
- [unified-backend/references/upstream-stage-contracts.md#ch-02-unity-transport](../unified-backend/references/upstream-stage-contracts.md#ch-02-unity-transport).
- [unified-backend/references/upstream-stage-contracts.md#ch-03-channel-response](../unified-backend/references/upstream-stage-contracts.md#ch-03-channel-response).
- [unified-backend/references/upstream-stage-contracts.md#ch-04-residual-error](../unified-backend/references/upstream-stage-contracts.md#ch-04-residual-error).
- [unified-backend/references/upstream-stage-contracts.md#ch-05-timing-and-phase](../unified-backend/references/upstream-stage-contracts.md#ch-05-timing-and-phase).

Normative lookup: [pinned PHY 3.1 I20 guidance](../../../../skills/docsis-phy-3-1-i20/SKILL.md).
Retained source records describe their original checkout and limitations; test
paths above do not claim a new pass. Final integrated backend acceptance stays open.
