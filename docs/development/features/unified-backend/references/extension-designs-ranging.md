> **Historical source reference; not MERLIN acceptance.** Preserved from
> [the exact source document](https://github.com/PyPNMApps/morgana/blob/69b38071cd3ebebc4eb96849f82b7871ab8da5ec/docs/development/features/extension-designs/ranging.md) at `69b38071cd3ebebc4eb96849f82b7871ab8da5ec`.
> Original commands, paths, statuses and artifact names describe that source only.
> [MERLIN architecture](../../../../architecture.md#unified-backend-integration),
> [requirements](../../../../requirements.md) and the
> [official checklist](../../../checklist.md#unified-backend-integration) govern integration.
> No source-only pass, design checkbox, cleanup or publication authorization transfers.

# Ranging, preamble and probe design

ARCH-RANGING-01 closes a design gate for REQ-SYS-04/REQ-CFG-04. Scheduled-data
TX-01 continues to use no invented generic preamble. Initial ranging, fine ranging
and probing remain separately tagged, unsupported execution modes until their
own implementation and acceptance. A PRBS15 payload is none of these signals.

## Pinned source applicability

PHY3.1 I20 §7.4.15 pp88–101 defines three distinct procedures: initial admission
signal, fine timing/power signal, and steady-state/admission probes. Initial
ranging §7.4.15.1 uses a configured BPSK preamble and the defined O-INIT-RNG-REQ
data encoding/CRC/randomization; it is not an ordinary scheduled-data LDPC grant.
Fine ranging §7.4.15.2 has its own duplicated symbol construction, preamble,
shortening/puncturing and randomization. Probing §7.4.15.3 uses the specified
12-bit pilot PRBS, not the ordinary-data pilot sequence or payload PRBS15.
PHY §7.4.17 relates received probes to corresponding-bank updates.

MULPI I25 §6.4.3 Tables28/29 specifies configured preamble/burst fields,
§6.4.4–5 the MAP/P-MAP contexts, and the RNG-RSP controls the Set/Adjust
operation. Source IUC3/4 ranging descriptors, CAT1 probe controls and ordinary
IUC5/6/9..13 data descriptors retain their distinct applicability. The existing
108-row source coverage and strict JSON admission preserve these controls while
rejecting their execution. PHY4.0 applicability must use its explicit delegated
endpoint; no FDX/FDD procedure equivalence is assumed from a shared code family.

## Owners and planned files

Create `crates/phy/src/ranging/` with `configuration.rs`, `initial.rs`, `fine.rs`,
`probe.rs`, `acquisition.rs` and `report.rs`. Create independently sourced
`crates/phy/tests/ranging_vectors.rs` and `crates/ldpc/tests/ranging_codes.rs`.
Extend `crates/ldpc` only with separately verified ranging matrices/puncturing;
do not coerce ordinary upstream N/K tables. Reuse safe Rust transforms, sample
stream/resource units and whole-bank normalization only where source semantics
match. Extend facade configuration/preview/run and native wrappers after those
owners work; schema/type declarations and paired examples cover each new mode.

## Interfaces and stage flow

Initial request carries explicit preamble bits/offset/length, zone bounds,
Nir/Mir, permitted guard/power/timing, CM MAC/downstream channel ID and trial
budget. Flow is message construction → source CRC/LDPC → source randomization
and padding → configured BPSK preamble/data placement → separately defined
initial-ranging transform/extension → received samples. Keep its exact source
unit/timing normalization and region construction local to this owner.

Fine request carries its distinct Nfr, configured preamble prefix and defined
fine-ranging data/framing controls. Flow uses its own FEC/puncturing and paired
symbols before its specified transform/extension. It cannot reuse an ordinary
grant merely by changing the IUC. Exclusion/unused handling follows the fine
ranging source rules rather than packing a sparse signal into contiguous bins.

Probe request carries explicit P-IE frame/symbol/start/skip/stagger/power/EQ
controls and the applied bank identity used for that corresponding probe.
Generate the source-defined probe PRBS at every physical carrier, then apply
mask/power/EQ control without compacting exclusions. Produce probe observations
with exact CM/channel/grid/time/bank provenance. Probe RxMER calculation and
corrective-update derivation remain separate operations; raw measured update
replay does not imply a live ranging controller or an RF measurement instrument.

Received acquisition searches only a bounded allowed interval/frequency set and
correlates against the configured preamble derived independently from source
configuration. Refine timing/CFO from the received repetitions/pilots appropriate
to that mode; publish score, ambiguity and coverage. Injected delay/H/C and the
expected encoded message never enter RX. Validate decoded CRC/framing separately
from external original-message comparison; false locks and valid wrong messages
remain observable failures.

## Bounds, lifecycle and independent acceptance

Cap zone/resource expansion at4096 carriers and36 symbols per physical frame,
source bursts/trials at256 and total samples at the existing16777216 job budget.
Acquisition shares the receiver design's finite candidate/product bounds and
safe cancellation points. Validate source-specific Nir/Nfr/preamble/guard and
power conditions before allocation; reject modes without a complete admitted
chain. Reports separate no-lock, ambiguous, partial, CRC, parity and source-match
outcomes with actual processing iterations/support. No live scheduler, network
capture/control or registration state machine is introduced.

Before implementation, independently transcribe/render each ranging matrix,
CRC/preamble mapping, puncturing, duplication and PRBS first-bit vector from the
pinned pages. Verify literal expected sequences, signs, physical-bin maps and
sample origins independently of production constants. Exercise both numerologies,
all applicable source bounds, exclusions, wrong preamble/CM/CRC, low evidence,
signed timing/CFO, interference, false locks, maximum work and cancellation.
Only then accept an actual source→TX→channel→received-acquisition→decode loop
with independent original comparison and shared Rust/native parity. No design
checkbox or successful ordinary-data loop satisfies that runtime gate.
