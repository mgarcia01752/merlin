# Single-channel measurement demo

Sanitized decoded captures copied from PNM Viewer Toolkit using `pnm-decode
0.3.6`. See [provenance](provenance.json) for source paths, revision, binary
hashes, decoded JSON hashes and individual capture times. Upstream
[LICENSE](LICENSE) and [NOTICE](NOTICE) are retained.

These decoder files are inputs to `ofdm-fec import-pnm`, not channel requests or
scenario documents. Use [import.json](import.json) to produce a scenario, then
[channel-measured.json](channel-measured.json) to select profile0, RxMER and response.
See the [experiment commands](../../../docs/user/impairments/experiments.md).

| File | Purpose |
| --- | --- |
| [rxmer.json](rxmer.json) | Unchanged decoder output; baseline readings |
| [channel_estimation.json](channel_estimation.json) | Unchanged complex coefficient observations |
| [modulation_profile.json](modulation_profile.json) | Unchanged captured profiles; select a profile explicitly later |
| [rxmer-reduced-6db.json](rxmer-reduced-6db.json) | Synthetic: subtract 6 dB from each available reading |
| [rxmer-center-notch-12db.json](rxmer-center-notch-12db.json) | Synthetic: subtract 12 dB within the middle 10% of array positions |
| [rxmer-lte-downlink.json](rxmer-lte-downlink.json) | Synthetic LTE-inspired ingress at737.5MHz using linear-power combination |
| [lte-ingress-model.json](lte-ingress-model.json) | LTE settings, alignment precheck and separate original timestamps |

All three captures use modem `00:ca:00:00:00:01`, channel 34, zero frequency
631100000 Hz, first index 356 and spacing 25000 Hz. RxMER and coefficients have
7480 entries: centers run from 640000000 to 826975000 Hz. Profile assignments
are expanded and checked by the current importer. Shared headers alone
do not prove role/assignment compatibility with a simulation mode.

Capture epochs differ: RxMER 1380970, channel estimate 1391100, profile 1466967.
The profile is 85997 seconds newer than RxMER. This is a spatially matching demo
set, not a simultaneous measurement. Do not silently relax timestamp association
or claim it represents one measured channel state. Its captured high-order
profiles also do not establish support beyond the current uniform16/256-QAM chain.

Synthetic variants retain original headers and timestamps for comparison. They
are alternate scenarios, not new captures over time. Only the `values` array
changes; null readings remain null. The notch covers offsets [3366, 4114),
carrier indices [3722, 4470), with affected centers 724150000–742825000 Hz.
No channel estimate, carrier role or modulation assignment is modified.
These MER edits do not introduce a temporal/burst model or guarantee FEC failure.

## Regenerate the variants

All fixture transformations run in Rust. From the repository root, choose an
existing empty output directory; the generator refuses existing destinations:

The input directory must also contain `channel_estimation.json` and
`modulation_profile.json`. Exact identity/grid and profile run-length counts are
checked before any output is written. Mismatches are rejected, never relabeled.

```bash
mkdir -p docs/development/local/pnm-measurement-inputs/regenerated
scripts/cargo-local.sh run -p docsis-ofdm-fec --example prepare_rxmer_variants --locked -- \
  examples/measurements/single-channel/rxmer.json \
  docs/development/local/pnm-measurement-inputs/regenerated
```

Compare regenerated files with the linked snapshots. The original reduction
fixtures use \(M'_k=M_k-\Delta_k\) in dB. The new
[LTE approximation](../../../docs/user/measurement-inputs/lte-ingress.md) combines
existing residual power and added interference before converting back to MER.
It uses a10MHz envelope,9MHz flat core and−30dB core interference/signal power.
No separate thermal floor is added; the resulting MER is applied only once in
the waveform simulation. This preparation itself adds no waveform noise and
computes no decoder outcomes. The rectangular notch remains a separate test case.

Original sanitized binaries are retained locally under ignored
`docs/development/local/pnm-measurement-inputs/originals/`. To decode a binary again with
PNM Viewer Toolkit, use `pnm-decode INPUT.bin --output NEW_OUTPUT.json`.
