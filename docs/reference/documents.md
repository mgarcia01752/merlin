# JSON and library contracts

Public models are in `docsis_ofdm_fec::profile::{model,document,evaluation,simulation}`.
The CLI uses serde representations directly; JSON additions belong to Rust.
Unknown fields are rejected in versioned documents/settings rather than guessed.

[scenario.json](../../examples/scenario.json) contains:

- `schema_version: 1`, explicitly selected `protocol`, individual `frames`,
  optional captured `profiles` and `channels`.
- An exact `key`: six MAC octets, channel ID and grid origin, first index, spacing
  in Hz and count. Frequencies are checked u32 Hz; no resampling is implicit.
- Source name, SHA-256, byte count, capture epoch, kind and format major/minor.
  Consumers must compute hashes from retained original bytes. Synthetic examples
  label their provenance; their identity values do not authenticate captures.
- Per-frame MER in dB with JSON null for unavailable readings; channel IQ retains
  null exclusions. Simulation arrays remain full resolution.

The candidate document uses `document_kind: "docsis_ofdm_fec_candidate"` and
`schema_version: 1`. It contains `candidate`, `settings` and optional `evaluation`.
A candidate retains grid, source provenance, profile identity, roles, bit loading
and captured/proposed origin. The schema is internal, not a universal “PMA file”.

Loaded protected roles must be reverified against supplied captured profiles
through `verify_captured_roles`; saved results must be recalculated. Neither
successful JSON parsing nor a supplied hash proves the source file's authenticity.
The standalone CLI does not retain raw binary files because it receives decoded
inputs; a capture-loading application owns original bytes and format validation.

Simulation JSON records named stages, seed, settings, source, selected carrier
offsets, trial counts, correction/failure/iteration statistics, confidence bounds,
assumptions and experimental validation status. Preserve the full result when
exporting. Do not extrapolate a selected-frame run to other times or modems.

CLI input/output JSON is limited to16MiB. Library scenarios allow at most4096
observations and4million total samples; each grid permits at most65536 carriers.
Candidates contain at most4096 source records, with names/hashes bounded by
validation. Large source collections should be decoded incrementally by a
consumer's bounded scanner; no archive extraction is implemented in this library.

The [PNM importer](../user/impairments/pnm-import.md) hashes exact decoded JSON
bytes and preserves independent source epochs. Its request paths are CLI-owned;
the library receives borrowed bytes. Channel requests optionally include
`impairments` (exclusive noiseless/AWGN/RxMER mode plus optional supplied H) or
`measurement` selection paired with `--scenario`. Outputs retain the resolved
settings, actual diagnostic MER, BER, LDPC iteration histogram/mean and decoder
failure counts. See [experiments](../user/impairments/experiments.md).

## Optional paired constellation output

Channel schema1 requests may opt in with `constellation`; a missing/null value
keeps the previous disabled behavior. The result omits `constellation` when
disabled and otherwise serializes the shared Rust paired-result schema1.
Selection/count/status and per-pair raw/corrected I/Q plus physical/source
coordinates are documented in the [paired-constellation guide](../user/impairments/paired-constellation.md).
This is an optional channel-analysis output, not a change to decoded PNM JSON.
