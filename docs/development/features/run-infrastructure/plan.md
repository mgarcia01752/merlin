> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/run-infrastructure/`
> Logs, captures and specification images are not published.

# Finish RUN and infrastructure before impairments

Scope: RUN-04/06/07/08, ARCH-05/06/07 and QA-04. Preserve existing numerical
contracts and public APIs. No new waveform impairment, QAM-mode, pilot-layout,
nonideal acquisition, Python or Qt work. User diagram is the later reference:
PRBS -> TX -> channel -> RX -> LDPC statistics/QAM analysis/BER. RxMER and DS
channel estimates enter the channel later; modulation profile configures TX.

Order: expose independently callable parity/twist/demux and reverse boundaries;
bounded typed JSONL observer export; source/noise observation invariance using the
existing coding experiment (not new waveform noise); source-window accounting;
cooperative job cancellation, budgets, progress and partial results; channel CLI
and bounded non-overwriting output/export; examples/guides/math and full gate.

Payload admission/window classification uses received source-symbol boundaries,
never TX queue IDs in RX. Keep global PRBS comparison continuous even for excluded
words. Strict whole-codeword window accounting: crossing words classified,
complete words contribute payload bits/BER/rate; do not apportion coded bits to
information bits speculatively. Preserve complete summary/selected words on stop.
