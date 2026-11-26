# Upstream 25 kHz: ideal synthetic channel

Use File → Open and select `workbench.merlin.json`. The Upstream tab loads
an idle, configured project. Press Start to run; open PRBS-RX to inspect
the recovered-source comparison and FEC results. No capture hardware is used.

`configuration.json` is the exact embedded source, copied from `examples/cli/upstream/us-25khz.json`.
The saved runtime options preserve its grant, sample, chunk and timeout limits.
Observation retention explicitly selects six taps: TX output, RX input, TX grid,
TX Pre-Eq, RX FFT and equalized RX cells. Its 24576-point aggregate budget is
six times the 4096-sample FFT length, providing one full window per tap for
the spectrum and QAM instruments. The original source document stays unchanged.
Receiver instruments retain one channel estimate, four soft cells and one
LDPC input-metric history. These presentation budgets do not change science.

All observations are synthetic. Measured-plan examples preserve UCD, RxMER,
Pre-Eq originals, association/probe context and the distinction between correction C,
physical channel H and effective C×H. They demonstrate the admitted composition;
they are not real CMTS measurements or external interoperability evidence.

[Bundle index](../README.md) · [All workbenches](../../README.md)
