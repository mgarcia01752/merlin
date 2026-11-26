# Upstream workbenches

Open `workbench.merlin.json` from one of these folders. The Upstream tab restores
an idle configuration; Start runs the native MERLIN transmitter → channel →
receiver loop. PRBS-RX shows the actual recovered-source comparison and FEC result.

For the complete input scenario, choose **synthetic-rxmer-25khz** or
**synthetic-rxmer-50khz**. Both embed their UCD, synthetic 40 dB CMTS RxMER,
Pre-Eq and execution settings. All input blocks are accepted after Open; press
Start to produce the actual independently verified result.

Ideal examples are also complete runnable scenarios. They do not require an
RxMER measurement: **Not used** is a neutral indicator. Before any run,
PRBS-RX shows neutral **Awaiting results**. A green PRBS-RX indicator requires
successful actual source verification; cancelled, incomplete or failed recovery
remains explicit.

| Bundle | Channel spacing | Input |
| --- | --- | --- |
| [ideal-50khz](ideal-50khz/README.md) | 50 kHz | Ideal synthetic channel. |
| [ideal-25khz](ideal-25khz/README.md) | 25 kHz | Ideal synthetic channel. |
| [synthetic-rxmer-50khz](synthetic-rxmer-50khz/README.md) | 50 kHz | Synthetic 40 dB CMTS RxMER, UCD and admitted Pre-Eq. |
| [synthetic-rxmer-25khz](synthetic-rxmer-25khz/README.md) | 25 kHz | Synthetic 40 dB CMTS RxMER, UCD and admitted Pre-Eq. |

Each saved file embeds its exact source document and finite run/instrument
budgets. Companion source documents and binary captures retain their original
bytes and hashes. These are authored synthetic examples, not field CMTS captures
or external interoperability evidence. No new RxMER/Pre-Eq/channel combination
is implied. The backend preserves correction C, physical H and effective C×H.

[All workbenches](../README.md) · [Upstream guide](../../../docs/user/upstream-workbench.md).
