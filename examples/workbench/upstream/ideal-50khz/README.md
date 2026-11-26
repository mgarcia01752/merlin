# Upstream 50 kHz: ideal synthetic channel

Use File → Open and select `workbench.merlin.json`. The Upstream tab loads
an idle, configured project. Press Start to run; open PRBS-RX to inspect
the recovered-source comparison and FEC results. No capture hardware is used.

`configuration.json` is the exact embedded source, copied from `examples/cli/upstream/us-50khz.json`.
The saved runtime options retain its grant, sample, chunk, timeout and tap budgets.
Receiver instruments retain one channel estimate, four soft cells and one
LDPC input-metric history. These presentation budgets do not change science.

All observations are synthetic. Measured-plan examples preserve UCD, RxMER,
Pre-Eq originals, association/probe context and the distinction between correction C,
physical channel H and effective C×H. They demonstrate the admitted composition;
they are not real CMTS measurements or external interoperability evidence.

[Bundle index](../README.md) · [All workbenches](../../README.md)
