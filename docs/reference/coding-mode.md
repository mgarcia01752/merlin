# Coding experiment and scientific conventions

## Evidence and scope

The first experiment uses DOCSIS3.1 downstream short LDPC N=16200, K=14400,
rate8/9, with BCH K=14232, N=14400, t=12, and uniform16-QAM.
No DOCSIS LDPC shortening or puncturing is enabled. The BCH code itself is a
shortened binary BCH code over GF(2^14); this differs from DOCSIS LDPC shortening.

Normative parameter sources:

- CM-SP-PHYv3.1-I20-230419 §7.5.4.2, Table23, PDF114: code dimensions and
  delegation to ETSI EN302769 V1.2.1 (2011-04).
- ETSI EN302769 V1.2.1 Tables3(b)/4(b), PDF29/30: BCH parameters and generator
  factors. AnnexB TableB.6, PDF96: LDPC accumulator addresses for short8/9.
- PHY I20 Tables24/25, PDF116: column-twist configuration. ETSI §6.1.3 and
  Table10(a) define parity interleaving and rate-specific bit demultiplexing.
- PHY I20 AnnexA Figure102, PDF198: 16-QAM labeling. The original PDF diagram
  was visually inspected and tested against all16 labels.
- PHY4.0 I07 §7.3.5 references the baseline PHY3.1 coding section. The toolkit
  retains a separate FDD protocol enum, but measured detailed simulation is
  restricted to the first DOCSIS3.1 experiment. FDX is unsupported.

The first chain starts at **post-randomizer BCH input**. MAC framing, NCP/PLC,
frequency/time-interleaver codeword placement, pilots in a physical waveform,
FFT/IFFT processing and a complete RF receiver are outside its boundary.
Data symbols cycle sequentially through selected data-carrier noise models.
The name and exported validation status explicitly call this a coding experiment.
An independently transmitted DOCSIS interoperability vector is still needed;
passing round trips and independently transcribed parameter equations do not
establish complete DOCSIS compliance.

Private PDF/evidence copies live under ignored
`development/specs/ds-profile-fec-lab/`. A fresh clone does not include them.
The exact ETSI edition is available from its official archive:
https://www.etsi.org/deliver/etsi_en/302700_302799/302769/01.02.01_60/en_302769v010201p.pdf

## Noise, energy and soft bits

All16-QAM reference coordinates are divided by sqrt(10), giving E[|s|²]=1.
For residual equalized MER M in dB:

```text
N0 = E[|n|²] = 10^(-M/10)
Var(nI) = Var(nQ) = N0/2
received = symbol + nI + j*nQ
LLR(bit) ≈ [min distance²(bit=1) - min distance²(bit=0)] / N0
```

LLRs use natural-log likelihood ratios; positive favors0 and magnitude is clipped
to100. The LDPC decoder uses horizontal layered sum-product updates and a user
iteration cap. Noise is independent Gaussian by assumption, not inferred impulse
noise or temporal correlation. No channel-estimate gain is applied again to
capture-derived residual MER.

To combine same-reference MER observations as a noise model, average error
power before converting back:

```text
M_equivalent = -10 log10(mean(10^(-Mi/10)))
```

An arithmetic mean in dB is not an average noise-power model. Temporal profile
screening instead retains individual frames and lower-tail carrier observations.

## Results and limits

Pre-LDPC BER counts hard-decision errors over16200 coded bits. Post-LDPC BER
counts errors in14400 BCH-input bits. Post-BCH payload BER counts errors over
14232 source bits. Decoder failure flags and undetected truth errors are separate.
Post-BCH payload CWER uses the actual generated payload truth per completed word.

CWER has a95% Wilson interval. For zero payload failures in n completed words,
the exact one-sided95% upper bound is `1 - 0.05^(1/n)` for **codeword** errors,
not BER. No completed trials means no estimated probability.

Trials are limited to512 words, at most100LDPC iterations and300000ms configured
runtime; interactive defaults are10words/30iterations/10000ms. Cancellation and
time budget are checked between words, preserving completed outcomes. A bounded
word may finish after cancellation/deadline. Codec setup occurs before timing.

The seeded random sequence is reproducible on the tested build; floating-point
results across untested native platforms are not promised bit-identical.

## Analytical profile screening

Targets are a named policy, not receiver guarantees. OSSI defaults come from
CM-OSSI3.1 I27 D.2.11 Table72, pp182–183, in quarter-dB steps. Shannon is an ideal
uncoded bound; the WebUI heuristic is a separately named implementation policy.

Conservative generation uses the minimum present reading for each carrier.
Percentile and weak-interval objectives use the lower empirical order statistic
`floor(q*(present_count-1))`. Missing observations remain coverage gaps; they do
not become noise-free carriers. A per-carrier allowance does not guarantee that
the same fraction of whole frames passes.

Gross useful-symbol capacity is `sum(data bits/symbol) * spacing_hz`. It excludes
cyclic prefix, FEC, pilot/PLC/excluded carriers, MAC and other PHY overhead. It is
not measured or net service throughput. DOCSIS3.1 candidates are capped at13bits
(8192-QAM); role preservation and modulation caps are checked independently.
