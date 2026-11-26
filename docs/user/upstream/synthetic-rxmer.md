# Synthetic CMTS RxMER

Generate offline upstream RxMER files in the CCAP-OSSI format using native Rust
or the installed native Python package. Generation creates data for testing;
it does not contact a CMTS, perform averaging or run a PHY simulation.

The [bundled requests and outputs](../../../examples/measurements/upstream/synthetic-rxmer/README.md)
cover both 25 kHz and 50 kHz carrier spacing. Each output directory contains:

- `synthetic-cmts-rxmer.bin`: the standard binary input.
- `synthetic-cmts-rxmer.bin.merlin.json`: synthetic origin, original request,
  decoded metadata/readings, conversion counts and the binary SHA-256.

Keep the sidecar with the binary. The standard binary header has no field that
distinguishes generated data from a field measurement.

## Generate a file

From the MERLIN repository root, choose a new output directory whose parent
already exists. Both examples refuse to overwrite an existing directory,
including an empty one.

```bash
scripts/cargo-local.sh run -p pnm-input --example generate_upstream_rxmer --locked -- \
  examples/measurements/upstream/synthetic-rxmer/us-50khz.json \
  local/synthetic-rxmer-rust
```

With MERLIN's native wheel installed in your Python environment:

```bash
python examples/api/python/generate_upstream_rxmer.py \
  examples/measurements/upstream/synthetic-rxmer/us-50khz.json \
  local/synthetic-rxmer-python
```

Substitute `us-25khz.json` and a different new directory for the other spacing.
Both examples call the same Rust generator and print its provenance JSON.
If a write fails, a new incomplete directory may remain; success is reported
only after both files are written.

## Request and conversion

Use the versioned request format in the examples. Identity, timestamp, upstream
interface, RPD, CM MAC, Pre-Eq state, number of averages and physical grid are
explicit inputs. No identities or times are discovered automatically. An
integrated CCAP uses an all-zero RPD ID and port zero. Number of averages is
positive unsigned-byte metadata, not a claim that probes were executed.

The `values` object accepts either:

```json
{"kind":"flat","mer_db":35.0,"count":512,"missing_subcarriers":[160,161]}
```

or explicit contiguous readings, with `null` for an unavailable position:

```json
{"kind":"samples","readings_db":[23.75,null,35.0,63.5]}
```

Physical indices begin at `first_active_subcarrier`; missing positions stay in
the sequence. The extent must fit the selected 4096-carrier 25 kHz or
2048-carrier 50 kHz grid. The file stores a quarter-dB octet per position:
0–63.5 dB maps to 0–254, and 255 means unavailable. Values are rounded to the
nearest quarter dB, with positive half-ties rounded up. The default
`range_policy: "reject"` rejects finite out-of-range readings; explicit `"clamp"`
clips them and records the changes. Nonfinite values always reject.

Requests are limited to 256 KiB, sequences to 4096 entries, and CCAP identifiers
to 255 UTF-8 octets without embedded NUL. Unknown fields, unsupported request
versions, duplicate missing indices and invalid metadata reject through shared
structured API errors. No resampling, channel fitting or FEC is performed.

Rust owns `pnm_input::upstream::{SyntheticRxmerRequest, SyntheticRxmerValues,
SyntheticRxmerCapture}`. `generate(&request)` accepts typed inputs and
`from_json(&str)` is the bounded transport adapter. Python exposes
`merlin.upstream.SyntheticRxmerCapture.from_json(request)`;
`original_bytes()` and `to_json()` return owned binary/provenance results.

## Interpretation and parser follow-up

The file describes upstream probe RxMER at the CMTS receiver, associated with
the declared Pre-Eq state. It is not a physical-channel response H or a transmit
correction C. Generation alone does not admit a measured experiment: that still
requires the [explicit association contract](measurements.md).

The format authority is CableLabs `CM-SP-CCAP-OSSIv3.1-I18-200610`,
sections 7.3.2.5 and 7.3.5.5, Table 410. MERLIN generates major 2/minor 0
`0x504E4E69` with its existing explicit most-significant-octet-first profile.
The [design and validation record](../../development/features/upstream-synthetic-rxmer/design.md)
pins the source and distinguishes this adapter profile from unspecified
wire-convention assumptions.

A future RAVEN parser can consume these standard files as synthetic fixtures.
RAVEN changes and a Qt Designer are separate follow-ups. The generated examples
are not field interoperability evidence and do not replace independent parser
test vectors.
