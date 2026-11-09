> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/payload-transport/`
> Logs, captures and specification images are not published.

# CELL-02 / WAVE-RX-06

Reuse existing packer, actual NCP codec, data/NCP randomizer, pilot reference,
cell interleavers, waveform TX/RX and concatenated short8/9 codec.
Promote interleaved_noiseless test orchestration into TX symbol assembly,
RX control hypotheses, bounded payload collection and separate stream facades.
RX configuration is obtained from the decoded PLC readiness owner and checked
on each receive. Fixed ProfileA256-QAM and uniform positive NCP only; reject
unsupported loading/activation/tagging, never substitute. Dynamic switching stays
NCP-03. Independent descriptor counts are not equated to one another.

Payload session origin is an explicit128-symbol boundary after PLC acquisition.
Local interleaver/packer/randomizer indices start0 there; absolute sample/channel
and source identities retain the origin. Interleavers do not reset per codeword
or buffer. Explicit BPSK startup/drain samples fill unavailable TX branch history;
receiver suppresses the known M-1 pair warm-up, not individual TX truth flags.
The reference begins a newly synchronized payload session after acquisition,
not blind acquisition of an already-running payload stream.

Validation: independent existing primitive tests remain. New tests for supported
configuration, received NCP-only boundaries, rejected/CRC/selector/continuation
errors and preserved diagnostics, both spacings/modulations, max depth, phase128,
queue bounds, startup/drain, observation invariance, actual payload waveform
recovery after PLC configuration. No PRBS or one-second runner in this group.
Duration will be a later parameter; no per-second reset or fixed1s API added.

Pinned local PHY I20 7.5.5.3,7.5.6,7.5.14.4 and8.3.4.1-3; reused implementations
retain existing independent vectors. PHY4.0 I07 delegates FDD to PHY3.1.
