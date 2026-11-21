# OFDMA transmitter

The typed transmitter accepts immutable [configuration](configuration.md), a
validated grant identity, a continuous source and a transmitter correction bank.
It returns indexed complex-baseband frame samples. Source positions advance once
for meaningful accepted payload; asking for the next frame consumes no extra bits.

```mermaid
flowchart LR
  G[Grant and source] --> F[LDPC and shortening]
  F --> R[PHY randomizer]
  R --> Q[QAM and resource visits]
  Q --> C[Transmitter correction C]
  C --> I[Shared unitary IFFT]
  I --> W[Prefix, window and overlap]
  W --> O[Indexed samples]
```

The grant ledger distinguishes source payload, information-fill ones, shortened
zeros, parity and zero grant pad. The 23-bit PHY randomizer resets per grant;
it does not reset PRBS15. Physical pilot/excluded-carrier coordinates survive
joint interleaving. Upstream 8-QAM and 32-QAM have their own source-backed labels;
other retained tables reuse the existing pinned resource bytes. Recognizing a
loading does not establish end-to-end receiver acceptance.

Pre-equalization is the transmitter's corrective response C. Its normalized
coefficient bank and explicitly based updates are separate from physical channel
H and received-pilot estimates. Unity correction preserves signed zero. The
IFFT wrapper retains named real/imaginary fields and converts to the bounded
shared numerical representation without unsafe casts.

Prefix/window/overlap state spans adjacent symbols, emits the terminal tail once
and retains absolute sample origins. Optional synthetic observation guards do
not change useful-symbol timing. Transmitter outputs include bounded scientific
inspection data; normal receiver input must exclude source/TX/C/H truth.

This implementation has local stage acceptance evidence. Complete typed configuration/session acceptance now passes; remaining adapter
and final regression gates are tracked in the
[official checklist](../../development/checklist.md#int-us-tx-01).
See [stage design and validation](../../development/features/ofdma/design.md#int-us-tx-01-transmitter-implementation-unit).
