# DOCSIS 3.1 PHY I20 Topic Map

Use this map first, then open the smallest relevant section file from `section-index.md`.
The PDF remains authoritative.

| Topic | Primary references | Notes |
|---|---|---|
| SC-QAM upstream/downstream PHY | `sections/06-phy-sublayer-for-sc-qam.md` | SC-QAM modulation, framing, transmit/receive, power, fidelity, downstream support. |
| OFDM/OFDMA spectrum and channel-band rules | `sections/07-2-upstream-and-downstream-frequency-plan.md` | Downstream/upstream spectrum and channel-band rules. |
| OFDM/OFDMA numerology | `sections/07-3-ofdm-numerology.md` | Downstream OFDM and upstream OFDMA numerology and subcarrier clocking. |
| Upstream OFDMA processing | `sections/07-4-upstream-transmit-and-receive.md` | FEC, randomization, interleaving, QAM mapping, IDFT, cyclic prefix/windowing, fidelity, transmitter output, receiver capability. |
| Upstream ranging PHY behavior | `sections/07-4-upstream-transmit-and-receive.md` | Initial/fine ranging PHY waveforms, timing, FEC, exclusions, probing, pilots, pre-equalization. |
| Upstream pilots and pre-equalization | `sections/07-4-upstream-transmit-and-receive.md` | Pilot structures and upstream pre-equalization. |
| Downstream OFDM processing | `sections/07-5-downstream-transmit-and-receive.md` | FEC, QAM mapping, interleaving, IDFT, cyclic prefix/windowing, fidelity and CM receiver requirements. |
| PLC | `sections/07-5-downstream-transmit-and-receive.md` | Physical Layer Link Channel structure, placement, synchronization and encoding. |
| NCP and downstream profiles | `sections/07-5-downstream-transmit-and-receive.md`, `sections/08-phy-mac-convergence.md` | NCP encoding and PHY-MAC mapping; profile-related codeword/data mapping. |
| Modulation and FEC | `sections/07-4-upstream-transmit-and-receive.md`, `sections/07-5-downstream-transmit-and-receive.md`, `sections/annex-a-qam-constellation-mappings.md` | LDPC/FEC behavior and QAM constellation mapping. |
| Power, CNR/MER, RF and fidelity limits | `sections/06-phy-sublayer-for-sc-qam.md`, `sections/07-4-upstream-transmit-and-receive.md`, `sections/07-5-downstream-transmit-and-receive.md` | Verify all numeric limits in the PDF tables before relying on extraction. |
| Exclusion bands and subcarriers | `sections/07-2-upstream-and-downstream-frequency-plan.md`, `sections/07-4-upstream-transmit-and-receive.md` | Active/excluded subcarrier behavior and ranging interactions. |
| Timing and synchronization | `sections/07-3-ofdm-numerology.md`, `sections/07-4-upstream-transmit-and-receive.md`, `sections/07-5-downstream-transmit-and-receive.md` | Subcarrier clocking plus upstream/downstream time/frequency synchronization. |
| PHY-MAC convergence | `sections/08-phy-mac-convergence.md` | Upstream profiles/minislots/subslots and downstream MAC-frame/codeword/NCP mapping. |
| Proactive Network Maintenance (PNM) | `sections/09-proactive-network-maintenance.md` | DS symbol capture, spectrum, NPR, channel estimate, constellation, RxMER, FEC, histogram, power; corresponding US PNM measurements. |
| QAM constellation mappings | `sections/annex-a-qam-constellation-mappings.md` | Explicit normative annex. |
| RFoG mode | `sections/annex-b-rfog-operating-mode.md` | Explicit normative annex. |
| European/Chinese SC-QAM variants | `sections/annex-c-european-sc-qam.md`, `sections/annex-d-chinese-sc-qam.md` | Explicit normative regional modifications. |
| CRC-24 | `sections/annex-e-crc-24.md` | Explicit normative CRC definition. |
| Sample interleaver / FEC code | `sections/appendix-i-downstream-frequency-interleaver-sample-c.md`, `sections/appendix-iii-upstream-interleaver-sample-c.md`, `sections/appendix-iv-fec-codeword-selection-algorithm.md` | Informative sample code only. |
| Candidate-profile SNR margin algorithm | `sections/appendix-vi-snr-margin-candidate-profile.md` | Informative suggested algorithm; do not treat as normative. |
| Tables, figures, formulas and conformance values | Relevant section plus authoritative PDF | Always verify layout-sensitive data against the PDF. |
