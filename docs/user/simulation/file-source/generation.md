# File generation and finite PRBS fill

The source owns one meaningful-bit cursor. Immutable original bytes are shared
with the verifier/configuration using bounded reference-counted storage. The
source never changes those bytes and does not share mutable state with noise.

```mermaid
flowchart LR
    CURSOR["Next meaningful bit index"] --> FILE["Original bytes: MSB-first"]
    FILE --> FILL["After file: optional seeded PRBS15"]
    FILL --> WORD["Up to14232 meaningful bits"]
    WORD --> PAD["Final information word: zero-pad tail"]
    PAD --> FEC["Existing BCH / LDPC"]
    CURSOR --> EOF["At finite target: EOF; no new word"]
```

For original byte count B, the file has F=8B bits. Its bit at index n is:

\[
x_n=(b_{\lfloor n/8\rfloor}\gg(7-(n\bmod8)))\mathbin{\&}1,
\qquad0\le n<F.
\]

Without fill, meaningful length T=F. With explicit append-to-fill, T is the
validated target and original bits always come first. The first appended bit
starts a fresh PRBS15 generator using the declared fill seed; its state then
continues across every information word. It does not advance during file bits
or padding, and is never reseeded at a word boundary. The [PRBS15 guide](../prbs.md)
defines the recurrence and seed emission order, which is distinct from file
byte MSB-first ordering.

With K=14232 information bits per supported BCH payload:

\[
N=\lceil T/K\rceil,\qquad P=NK-T.
\]

For T=0, N=P=0. Otherwise only the final payload has P zero padding bits; complete
payloads have no extra padding. This is application information padding before
the existing encoder, not an implementation of a new PHY shortening mode. Coded
last-cell padding for mixed profiles is a separate existing packing operation.

EOF returns no additional payload. The channel still transmits control/idle
symbols for the configured observation interval, completes already queued words
and drains the interleavers/waveform. The source cannot force a short observation
to consume the whole file. Queue-ahead generation may complete queued words after
observation; global source comparisons and whole-word window metrics remain
separate. Settings are frozen at session creation.

The source probe `source.payload` sees the complete information word,
including its coding-padding tail. Summary counters identify meaningful bits and
padding separately; observing this point does not draw noise or advance truth.
EOF, source limits, fill seed and target are validated in Rust. The CLI handles
only bounded regular-file loading; future Qt controls must use the same checks.
