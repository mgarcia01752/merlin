> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/single-channel-transmission/`
> Logs, captures and specification images are not published.

# Fixed single-channel transmission

## Accepted scope

One downstream OFDM channel; occupied bandwidth capped at192MHz. Initial
integration uses256-QAM data cells and a noiseless unity-gain path. Fixed CMTS-like
settings selected once per run. One second denotes simulated observation time.
No bonded channels or dynamic reconfiguration.

## Implemented foundation

transmission/settings.rs owns validated sample-clock numerology. FFT follows
spacing; CP and RP are independent enumerated sample lengths with RP<CP. Depth
is1..16 at25kHz or1..32 at50kHz. Occupied bandwidth is24..192MHz in6MHz steps.
transmission/timing.rs calculates whole periods and remainder in one second.
The one-second observation includes23529 default periods and3584 spare ticks.
Settings API is independent of the optional simulation feature.
Pinned PHY I20 page135 was rendered and CP/RP tables visually inspected.

## Confirmed timing

PHY I20 section7.5.8.1 delegates to7.4.10.1;7.4.10.2.1 explicitly states
N+Ncp period despite N+Ncp+Nrp window length, due to overlapping windows.
Do not double count roll-off. Detailed sources and equations are in the user guide.

## Pending integration

Explicit frequency reference, band-edge/internal exclusions, PLC placement,
continuous/scattered pilots and NCP roles. Do not derive payload carrier count
from occupied bandwidth alone. Verify256-QAM labels, demux/twist and randomizer.
Implement symbol packing, persistent time interleaver and per-symbol frequency
interleaver, inverse receiver, waveform CP/windowing and streaming truth counters.
No changes to existing16-QAM CLI or its JSON contracts in this foundation.

Freeze settings snapshot in future runner. Track sample time and128-symbol phase.
Warm-up and drain are separate from the one-second observation; exact lengths
remain pending implementation. Tag filler/pending cells and compare only complete
payload blocks. Bound memory and work; stream rather than retain204.8million IQ
samples. Keep cancellation/progress, plus explicit full-channel support status.

## Validation

Test independent4K/8K timing, valid/invalid depth and CP/RP values, occupied
bandwidth boundaries, and roll-off independence of symbol cadence. These tests
validate configuration/timing, not waveform or noiseless full-channel recovery.
