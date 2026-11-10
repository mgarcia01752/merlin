# Optional independent full-channel interoperability

On2026-11-09 the user clarified that MERLIN is a closed loop using its own TX,
channel and RX. QA-05 now closes through [internal end-to-end validation](../closed-loop-validation/validation.md).
Independent instrument/reference-tool data is not part of minimum acceptance.
The requirements below are retained only for optional future interoperability
work. The existing admission tool checks provenance/integrity, not simulation
results, and is not a required minimum workflow.

If that optional scope is requested later, obtain one reproducible dataset from an independent CMTS/reference implementation,
with redistribution permission or keep it under ignored `docs/development/local/`.
The producer must supply all four roles below; ordinary PNM measurements do not
contain transmitted PRBS truth and therefore are insufficient by themselves.

| Role | Required content |
| --- | --- |
| `waveform_samples` | Received complex I/Q at204.8Msamples/s; byte order, f32/f64 or integer scaling, interleaved I/Q order, sample origin and exact count |
| `transmission_settings` | Pinned PHY issue, FFT/spacing, CP/RP, physical frequency origin, exclusions, continuous pilots, PLC placement and phase convention; include time/frequency interleaving and profile changes |
| `expected_payload` | Exact transmitted bytes/bits, bit order, PRBS polynomial/seed or known file, codeword boundaries, supported full/shortened coding and expected recovered bytes |
| `expected_control` | Independent OCD/DPD packets and NCP/PLC decoded truth, C/N/U timing, counters, CRC/FEC expectations and received start/continuation positions |

Also record independent producer name/version, exact source/reference and SHA-256
and byte count for every file. Channel/noise settings and expected pre/post-CH-EST
planes must be explicit. Request error-free and impaired cases at both spacings,
with codeword boundaries crossing symbol/interleaver/reset/profile-change points.
Do not silently adapt ordering/scaling until a test passes; document every adapter.

## Bounded manifest admission

`tools/validation/admit_external_vectors.py` accepts manifest revision1 with exactly
`schema_version`, `producer`, `producer_version`, `reference`, and `files`. Each of
the four `files` records contains `role`, portable relative `path`, lowercase
`sha256` and positive integer `bytes`. Paths cannot escape or use symlinks. Manifest
size is at most1MiB and each file at most64MiB. Longer datasets must be partitioned
into independently described bounded cases before import.

```bash
python3 tools/validation/admit_external_vectors.py \
  --manifest docs/development/local/external-vectors/manifest.json \
  --report docs/development/local/external-vectors/admission.json
```

The report status is deliberately `admitted_unexecuted`: it checks declared
provenance/file integrity only. It does not authenticate the source or interpret
simulation payloads. No external-vector replay adapter or simulation comparison
has been implemented from an absent format. Once actual artifacts arrive, add a
Rust replay adapter for their specified encoding and compare raw transforms,
control, soft decisions, corrected samples, recovered payload and decoder outcomes
against that independent truth before claiming external interoperability.

## Scope clarification

Synthetic and measurement-driven closed-loop tests are sufficient for the current
MERLIN minimum. Existing tests compare recovered source bits, file bytes and
control information against the corresponding original values. The absence of
an external waveform/payload/control bundle is not an open minimum task.
