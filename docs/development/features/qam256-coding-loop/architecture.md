> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/qam256-coding-loop/`
> Logs, captures and specification images are not published.

# Short8/9 uniform256-QAM coding loop

Scope: verified parity/twist/demux and standalone noiseless cell-stream loop.
Not a full physical OFDM-symbol or1second waveform simulation. Existing16QAM CLI
and golden scientific metrics must remain unchanged.

Evidence: local PHY I20 section7.5.4.3.1 Table24/25 PDF116 and7.5.5.2.1 Table32
PDF121 delegates short256 to ETSI EN302769 V1.2.1 clauses6.1.3/6.2.1,
pages32–38 Table10(a) PDF36. Rendered PHY116 and ETSI33/36. Rows2025,cols8,
twists0,0,0,1,7,20,20,21. 256demux[7,3,1,5,2,6,4,0]. No padding:16200/8=2025.
Bit-interleaver API supports16and256 initially; reject others until their
padding/rate-specific rules are independently implemented.

Reusable BlockLoop owns codec+permutation. Each block trial starts synchronized
data/NCP randomizers at phase0; no claim of continuous OFDM framing across
independent trials. Retain LDPC iterations/success and truth errors in typed JSON
result, with finite reference variance for softbits but no injected noise.

Validation: independent first/second/last256cell source-index vectors, full
bijection, impulse routing, inverseLLR magnitudes and reject invalid inputs.
Legacy16permutation must be identical. Run allzero/edge/seeded payloads through
FEC→bitprocessing→randomizer→QAM→inverse→FEC and compare14232payload bits.
No actual NCP content, pilot placement/packing, temporal/frequency interleaving,
IFFT/window or new full-channel CLI mode. Documentation must state boundaries.
