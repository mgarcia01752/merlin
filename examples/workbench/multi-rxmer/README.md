# Multiple RxMER captures, one channel response

Open `workbench.merlin.json`: one modulation-profile PNM, five RxMER files and
one DS channel-estimate PNM. The default combines RxMER as an equal-weight mean
of linear residual error power. It does not average dB values or invent capture
time evolution. The WB uses PRBS15 comparison and a 1 ms pilot-estimated run.

Baseline and synthetic 12/11/10/9 dB stress variants share modem, channel, epoch
and every frequency-grid coordinate. The variants deliberately change only
selected residual error values; they are not successive measured captures.
Receiver errors are expected possibilities, not invalid example configuration.

Original files are adjacent to and embedded in the WB. Provenance, original
timestamps, hashes, [LICENSE](LICENSE) and [NOTICE](NOTICE) are retained. See
[provenance](provenance.json) and the [stress input explanation](../../measurements/rxmer-fec-stress/README.md).
