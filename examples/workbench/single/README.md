# Single capture set

Open `workbench.merlin.json`: one modulation-profile PNM, one RxMER PNM and one
DS channel-estimate PNM, with PRBS15 comparison enabled and a 1 ms pilot-estimated
run. All three originals are adjacent to the WB and embedded in it with SHA-256.

The set uses modem `00:ca:00:00:00:01`, channel 34 and 25 kHz spacing. The 7,480
measurement centers span 640.000–826.975 MHz. DPD compatibility is validated by
the same Rust import/request APIs used by the application.

This is a matching frequency grid, not a synchronized capture event. The
association tolerance is explicitly 90,000 seconds. See [provenance](provenance.json),
[LICENSE](LICENSE) and [NOTICE](NOTICE) for source identity and attribution.
