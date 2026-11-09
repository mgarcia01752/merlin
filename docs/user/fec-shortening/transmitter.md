# Shortened-codeword transmitter

The shortening stage surrounds the existing encoders with four explicit
coordinate operations. The authoritative order is PHY I20 §7.5.4.2.3,
Figures46–47, PDF114–115. The local original PDF page115 was visually checked.

```mermaid
flowchart LR
    DATA["K original bits d"] --> PREPEND["Prepend S zeros"]
    PREPEND --> BCH["BCH mother encoder"]
    BCH --> STRIP["Remove S leading zeros"]
    STRIP --> APPEND["Append S zeros"]
    APPEND --> LDPC["LDPC mother encoder"]
    LDPC --> OMIT["Omit appended systematic zeros"]
    OMIT --> WIRE["K data +168 BCH parity +1800 LDPC parity"]
```

Writing concatenation as $\Vert$, the boundaries are:

$$
b=\mathrm{BCHenc}(0^S\Vert d)=0^S\Vert d\Vert p_B,
$$

$$
u=d\Vert p_B\Vert0^S,\qquad
c=\mathrm{LDPCenc}(u)=d\Vert p_B\Vert0^S\Vert p_L,
$$

$$
w=d\Vert p_B\Vert p_L.
$$

BCH leading zeros do not alter the polynomial remainder, but moving zeros from
the front to the tail before LDPC changes the LDPC systematic coordinates and
its parity. Encoding the original full BCH mother word directly into LDPC and
then deleting its prefix would be the wrong process.

Each boundary is independently callable through the validated geometry and
returns an owned bounded vector without changing its input. Removal functions
reject nonzero omitted positions. They check shape/zero structure, not encoder
syndrome; the mother codec and independent tests own parity validation.

The composed encoder returns BCH mother output, padded LDPC input, LDPC mother
output and the transmitted word as direct test points. Nothing is punctured from
BCH or LDPC parity. Bit interleaving and QAM grouping consume the transmitted
coordinates in a later integration stage; these functions do not invent those
rules or reuse a full-word interleaver on a shorter vector.
