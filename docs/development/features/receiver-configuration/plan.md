> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/receiver-configuration/`
> Logs, captures and specification images are not published.

# RX-CONFIG-01

Implement encapsulated channel reconstruction, profile projection, bounded
three-descriptor collection, complete immutable configuration and readiness state.
Reuse existing OCD validation by returning the decoded channel instead of
maintaining a duplicate geometry decoder. Packet inputs must retain valid MAC
HCS/FCS; channel/source identities are pinned per acquisition session. Bootstrap
spacing stays supplied reference knowledge, not hidden TX configuration.

Pinned local MULPI I25 §6.4.40/PDF197–199; §6.4.41/PDF200–203 establishes
independent OCD/per-profile change counts, default/specific precedence regardless
of TLV order, complete assignment and uniform NCP modulation. Existing PHY rules
remain enforced by CarrierLayout and InterleaverSettings. No equality requirement
between OCD and DPD counters. No dynamic change activation: changed single-part
messages within an accepted session require an explicit acquisition reset.

Status Waiting/Ready/Rejected describes admission/readiness. Keep last valid
configuration on rejection but close the data gate until valid cached messages
restore readiness (or reset/reacquire). Immutable fields prevent partial config
from appearing ready. Accept all six message orders and identical repeats.
Collect only three messages; reject conflicts, source/channel/spacing mismatches,
incomplete coverage, out-of-grid assignments or nonuniform NCP loading atomically.

Add a simulation acquisition composition receiving only equalized local PLC IQ,
then FEC/MC reassembly/configuration admission. Return completed decoder results
and per-message rejection diagnostics. No actual payload receiver/waveform added.
Separate per-subsystem user design pages with Mermaid; APIs stay in rustdoc/API
reference. Extend actual FEC/QAM example/integration to use received settings.
Run module/state/rejection and real multi-frame acquisition tests, full gate,
analysis-only Clippy and skill/doc validation. Update architecture/checklist.
