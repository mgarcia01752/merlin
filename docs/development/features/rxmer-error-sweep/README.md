> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/rxmer-error-sweep/`
> Logs, captures and specification images are not published.

# RxMER error-boundary experiment

All science executes in Rust. `run.py`, `stress.py` and `refine.py` only invoke
Rust CLI binaries and collect their JSON result fields. All output paths are new.
Original decoded captures are unchanged. Seeds, duration, profile and supplied
response stay fixed. Read `summary.json` for the ordered stress results.

Preparation:

```bash
target/release/examples/prepare_impairment_runs \
 examples/measurements/single-channel NEW_DIRECTORY --stress-mer-db 11
target/release/ofdm-fec channel \
 --input NEW_DIRECTORY/rxmer-stress-request.json \
 --scenario NEW_DIRECTORY/rxmer-stress-scenario.json \
 --output NEW_DIRECTORY/rxmer-stress-result.json
```

The stress band is700MHz inclusive to727MHz exclusive and lowers only available
static data readings.1072/7480 positions change. PLC648–654MHz, continuous pilots,
nulls, source grids, epochs and source hashes remain unchanged. Assertions checked
these invariants, invalid option rejection and no-clobber. Core LTE strength is
separate from the stress target. The first generator build used an LTE settings
object also in the stress sidecar; it was corrected to null before refined11/10dB
runs. Actual stress scenarios and requests use the intended target, not that
unused sidecar object. No waveform/codecs changed during the experiment.

First tested errors:11dB (483/3614928 bits), with12dB error-free. At9dB all254
payload words hit50 LDPC iterations. All runs completed; no PLC bit errors or NCP
rejection. These are short deterministic runs, not a general DOCSIS threshold.

Validation: release example build, rustfmt, warnings-denied example Clippy,
actual CLI sweep, fixture preservation and invalid/no-clobber checks. No system
packages installed. Linux only. Full workspace gate passed before this fixture
option change; not rerun for this fixture-only experiment.
