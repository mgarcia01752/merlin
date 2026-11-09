> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/lte-ingress/`
> Logs, captures and specification images are not published.

# LTE downlink ingress fixture preparation

Scope: engineering frequency-domain approximation, not LTE waveform synthesis or
MODEL-04 completion. Shared Rust numerical API owns the envelope and residual
power combination. Fixture tooling checks exact identity/grid/profile coverage,
keeps timestamps unchanged and emits existing decoder-shaped MER plus a sidecar.

Reference: ETSI TS136104 V17.5.0 sections5.5/5.6, table5.5-1 and table5.6-1.
Band12 downlink729–746MHz, chosen737.5MHz center/10MHz nominal width. Flat9MHz
core represents50 resource blocks times180kHz; cosine shoulders are engineering
assumptions and not an LTE emission mask. No RF calibration, DC notch, traffic
loading, timing, LTE waveform or FEC guarantee is inferred.

User clarification: noiseless waveform plus RxMER-derived residual only. Add no
separate thermal/system floor. The derived LTE fixture already combines baseline
and LTE interference; apply its RxMER once in future channel integration.

Checks: independent equal-power/shoulder numerical anchors, boundaries/symmetry,
frequency-coordinate invariance, nulls/invalid values, grid/identity mismatches,
real fixture regeneration, unchanged originals, output no-clobber and hashes.
