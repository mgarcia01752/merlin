# Upstream 50 kHz: synthetic measured 40 dB CMTS RxMER with admitted Pre-Eq

Use File → Open and select `workbench.merlin.json`. The Upstream tab loads
an idle, configured project. Press Start to run; open PRBS-RX to inspect
the recovered-source comparison and FEC results. No capture hardware is used.

`measured-experiment.json` is the exact embedded source, copied from `examples/measurements/upstream/measured/capture-driven.json`.
The saved runtime options retain its grant, sample, chunk, timeout and tap budgets.
Receiver instruments retain one channel estimate, four soft cells and one
LDPC input-metric history. These presentation budgets do not change science.

All observations are synthetic. Measured-plan examples preserve UCD, RxMER,
Pre-Eq originals, association/probe context and the distinction between correction C,
physical channel H and effective C×H. They demonstrate the admitted composition;
they are not real CMTS measurements or external interoperability evidence.

[Bundle index](../README.md) · [All workbenches](../../README.md)
