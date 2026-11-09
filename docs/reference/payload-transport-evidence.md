# Payload transport evidence and support boundary

| Pinned source | Requirement / ownership | Implementation and validation |
| --- | --- | --- |
| CM-SP-PHYv3.1-I20-230419 §7.5.5.3, PDF123–125 | Data/NCP shared cell randomizer and128-symbol resets | Reuse existing independently verified CellRandomizer; new TX assembly and separate RX hypotheses preserve clocks across words/filler |
| PHY3.1 I20 §7.5.6.1–3, PDF129–133 | Time/frequency transport, reduced domain and pilot reference | Reuse existing SymbolInterleaver and PilotReference; production TX/RX preserve source/channel identities, max depths and M-1 pair delay |
| PHY3.1 I20 §7.5.14.4, PDF163–164 | Highest eligible pre-interleaving NCP positions and constellation selection | Existing verified packer/field mapper feed actual cells; RX discovers length from decoded messages rather than TX reservations |
| PHY3.1 I20 §8.3.4.1–3, PDF180–185, Figure92/Table53 | Continuous codewords, start pointers, Z/NULL, profile selectors and whole-field CRC | Existing NCP codec plus new bounded collector/source decoder; valid-FEC wrong-CRC, orphan/early starts/filler and unsupported-selector tests |
| PHY3.1 I20 §7.5.4–5 and referenced ETSI short8/9 mode | BCH then LDPC; corresponding soft inverse bit processing | Reuse ConcatenatedCodec/BitInterleaver; decoded flags/iterations and independent actual payload comparison |
| CM-SP-PHYv4.0-I07-250627 §7.3.5.5–6 | FDD delegates mapping/interleaving to PHY3.1 | Same verified baseline subset; no FDX-specific or general high-order coding claim |

Source paragraphs and cross-references were reviewed in the pinned local text.
Existing primitive arithmetic retains its earlier independent vectors and local
PDF checks; this integration adds no newly transcribed waveform or coding formula.
See [NCP](../user/ncp.md), [cell interleaving](../user/cell-interleaving.md),
[FEC scope](coding-mode.md) and [waveform evidence](waveform-evidence.md).

The fixed payload session accepts uniform256-QAM ProfileA, uniform positive NCP
and the even selector pattern emitted by the existing packer. Dynamic odd/even
activation, U signaling, tagging and zero/mixed DPD loading are unsupported;
rejection is an explicit subset policy, not a claim that DOCSIS prohibits them.
OCD/DPD counters stay independent and are not equated to activation selectors.

Post-acquisition session synchronization at a128-symbol origin, explicit BPSK
startup/drain replacement, known matched-pair warm-up suppression, live readiness
checks and terminal payload-session rejection are laboratory/reference policies.
No TX replacement-rank list, queue ID, original bits or field length enters RX.
Successful actual payload waveform tests establish this fixed noiseless subset,
not blind acquisition, nonideal resynchronization, full dynamic modem behavior,
external interoperability or the future one-second job runner.

[Subsystem design](../user/payload-transport/README.md) describes state, boundaries,
results and the bounded sample-domain example.
