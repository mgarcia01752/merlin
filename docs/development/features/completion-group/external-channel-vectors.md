# QA-05: independent full-channel evidence still required

No external full-channel known-truth dataset is available in this checkout.
Locally generated encoder/receiver round trips, pinned arithmetic and synthetic
noisy-channel tests are separate evidence. They cannot close QA-05. QAM-04 remains
excluded/deferred and its existing high-order vector requirements stay unchanged.

Obtain one reproducible dataset from an independent CMTS/reference implementation,
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
scientific payloads. No external-vector replay adapter or scientific comparison
has been implemented from an absent format. Once actual artifacts arrive, add a
Rust replay adapter for their specified encoding and compare raw transforms,
control, soft decisions, corrected samples, recovered payload and decoder outcomes
against that independent truth. Only then can QA-05 be checked.
