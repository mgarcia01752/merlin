# Coding-loop design

This module verifies one payload's noiseless path through data coding and256-QAM.
Its purpose is to isolate coding/modulation problems before introducing pilots,
physical placement or a waveform channel.

```mermaid
flowchart LR
    INPUT["Known payload bits"] --> BCH["BCH encoder"]
    BCH --> LDPC["LDPC encoder"] --> BITS["Parity/twist/bit demultiplexing"]
    BITS --> RAND["Data-cell randomization"] --> QAM["256-QAM mapping"]
    QAM --> SOFT["Noiseless reference soft demapping"]
    SOFT --> INV["Inverse mask and bit processing"]
    INV --> LDPCD["LDPC decoder: actual iterations"] --> BCHD["BCH decoder"]
    BCHD --> CHECK["Recovered payload comparison"]
    INPUT --> CHECK
```

The module composes existing codec and DSP stages. Those stages own their
mathematics; the loop owns their execution order and per-codeword comparison.
Original bits remain independent truth and are never supplied to the decoder.

Codec/permutation setup is reused, but each trial begins with fresh phase0 TX
and RX randomizers. Successive trials are independent codeword experiments,
not successive symbols of a continuous OFDM channel. No noise is injected.
A finite reference noise variance still supplies soft-bit scaling; that value
does not imply that AWGN was added.

Results distinguish errors before LDPC, errors after LDPC, final payload errors,
BCH corrections, decoder success and actual LDPC iteration count. A decoder flag
and a truth comparison answer different questions; both are retained.

This module does not create NCP messages, pilots, PLC signaling, time/frequency
cell interleaving, IFFT samples or channel timing. Those components are developed
and tested separately before full-channel composition.

See the [256-QAM stage guide](../qam256-coding-loop.md) for equations and a runnable
example, and the [FEC guide](../fec-codec.md) for the separate coding boundary.
