> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/impairment-group/`
> Logs, captures and specification images are not published.

# Five-step impairment group

1. MODEL-04 sample-domain seeded AWGN with absolute unitary-reference noise power.
2. Independent normalization/statistical tests and actual waveform/FEC outcomes.
3. INPUT-01 current pnm-decode JSON importer, explicit time tolerance, hashed JSON provenance.
4. Per-carrier RxMER residual Gaussian synthesis through IFFT/CP/window and actual RX.
5. MODEL-05 supplied H channel response plus known-reference equalizer. A symbol-diagonal
   synchronized channel approximation, not arbitrary linear multipath or blind estimation.

No hidden thermal floor, combined AWGN+RxMER mode or duplicate residual injection.
Current uniform256-QAM profile support stays enforced. Import all profile orders,
but reject unsupported applied payload orders. Preserve nulls; reject missing active
carrier measurements; never relabel grids. CLI paths are adapters, algorithms are Rust.
Use direct stage outputs plus existing channel/RX test points; document each subsystem.

Evidence: local CM-SP-CM-OSSIv3.1-I27-250219 AnnexD.2.6 PDF166 explicitly
reports H; reciprocals typically form equalizer coefficients. Text inspected, no
rendered-PDF verification claimed. Channel estimates scaled to approximately unit
average power, so this is relative response, not absolute calibrated path loss.
