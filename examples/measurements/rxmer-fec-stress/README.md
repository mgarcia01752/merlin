# RxMER FEC stress scenarios

Open these binary PNM files directly in PNM Viewer Toolkit. They contain the
exact per-carrier RxMER inputs used for the FEC error-boundary experiment.

| File | Impaired-band RxMER | Post-FEC bit errors | Mean LDPC iterations |
| --- | ---: | ---: | ---: |
| `rxmer-baseline.bin` | Original readings | — | — |
| `rxmer-stress-12db.bin` | 12 dB | 0 | 6.22 |
| `rxmer-stress-11db.bin` | 11 dB | 483 | 9.93 |
| `rxmer-stress-10db.bin` | 10 dB | 28,187 | 35.26 |
| `rxmer-stress-9db.bin` | 9 dB | 74,575 | 50.00 |

The synthetic variants lower available readings on 1,072 static data positions
in 700–727 MHz. The remaining readings, pilots and PLC neighborhood at
648–654 MHz retain their baseline values. Each file has 7,480 readings.
The PNM files store RxMER, not the BER results shown above; those results came
from separate 3 ms, 256-QAM, seed-41 channel simulations comparing 3,614,928 bits.

All variants retain the original MAC `00:ca:00:00:00:01`, channel 34, frequency
grid and capture time. These are alternative synthetic scenarios, not consecutive
captures. View each separately; time-series import may treat them as duplicates.
PNN4 version 1.0 encodes RxMER in quarter-dB steps. No rounding was necessary:
`pnm-decode` verified every exported sample against the corresponding simulator
input. See `provenance.json` for source identity and output hashes.

## Reproduce one variant

From the repository root, using new output paths:

```bash
scripts/cargo-local.sh run -p docsis-ofdm-fec --release --locked \
  --example prepare_impairment_runs -- \
  examples/measurements/single-channel development/export-rxmer-11 \
  --stress-mer-db 11

scripts/cargo-local.sh run -p docsis-ofdm-fec --release --locked \
  --example export_rxmer_pnm -- \
  examples/measurements/rxmer-fec-stress/rxmer-baseline.bin \
  development/export-rxmer-11/rxmer-stress-scenario.json \
  development/rxmer-stress-11db.bin

pnm-decode development/rxmer-stress-11db.bin \
  --output development/rxmer-stress-11db.json
```

The offline Rust exporter validates the original binary identity/grid/time,
requires exactly representable values, preserves the header and rejects existing
destinations. It is a fixture tool, not a production binary-input adapter.

Acknowledgment: PyPNM - Proactive Network Maintenance Toolkit.
Original source attribution and license are retained in `NOTICE` and `LICENSE`.
