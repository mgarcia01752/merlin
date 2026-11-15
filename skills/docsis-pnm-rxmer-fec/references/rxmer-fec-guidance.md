# Reviewed interpretation guidance

These are paraphrased findings with MERLIN application notes, not new CableLabs
requirements. All references below are V05 unless V02 is explicitly named.

## Candidate-profile subcarrier counts

Section 5.12, pp.120–121, describes candidate-profile threshold comparisons:
`RxMER < (RxMER vs Bit-loading Target - RxMER Margin)`.
The target is chosen for the subcarrier's assigned modulation. It is not simply
`10 log10(2^bits - 1)`. The reprinted PHY example associates target values with
CER 1e-5 under ideal AWGN. It describes a configurable deficit with default 1 dB.
The displayed inequality is strict, while the explanatory prose says "at least";
resolve the boundary using the applicable normative PHY/MULPI definition before
claiming exact protocol equivalence. Do not silently round equality across it.

The referenced example SNR-margin algorithm averages in dB, then subtracts the
required average from the measured average. The guide explicitly says that the
example algorithm is non-normative and alternative vendor algorithms are allowed.
This dB averaging is a named reporting calculation, not permission to replace
MERLIN's residual-power averaging/interpolation of captures.

MERLIN design implications (proposed, not implemented by this skill): report
below-target count and percentage, evaluated denominator, missing/unsupported
measurements, target source and deficit margin. Use exact aligned grids and the
selected profile. Distinguish input field RxMER from simulated receiver MER.
Define data-carrier eligibility separately from all-active-carrier statistics;
exclude non-data roles from a data-profile statistic and never count nulls as good.
A percentile of profile-relative margin for mixed modulation is a MERLIN extension,
not the guide's raw-RxMER percentile. Zero eligible samples require unavailable
status, not a fabricated zero-percent healthy result.

## Percentile reliability screening

Section 5.3.5, pp.82–84: the raw-RxMER percentile is over active carriers, sorted
ascending. Its example floors 3677 × 0.02 to rank 73 (one-based). It is not an
interpolated percentile. Consult normative definitions for rank-zero/small-sample
edge cases; this example alone does not resolve them.

Table 6 (p.84; also V02 p.74) gives these empirical reliability bands in dB:

| QAM | Pass | Marginal pass | Fail |
| --- | --- | --- | --- |
| 256 | >26 | 24–26 | <24 |
| 512 | >29 | 26–29 | <26 |
| 1024 | >32 | 29–32 | <29 |
| 2048 | >35 | 32–35 | <32 |
| 4096 | >38 | 35–38 | <35 |

These apply to the 2nd-percentile metric, not every individual subcarrier and not
a universal "2% erased carriers" decoder limit. The accompanying text uses a
1024-QAM example with errors below 29 dB. Table 5 on p.82 is different: operating
recommendations for average RxMER. Do not conflate the two tables, the PHY receiver
requirements, OSSI configurable targets, or Shannon capacity references.

## Controlled FEC experiments

Appendix II pp.203–215 (V02 Appendix I pp.151–166) reports AWGN, simulated LTE
and combined-impairment tests. The plan specifies at least eight CMs from four
manufacturers, fixed 4096-QAM for the initial tests and several RF input levels.
Figure 190, p.208, separately plots CM and analyzer MER against corrected and
uncorrectable codeword proportions. The plotted flat-noise test has a broad range
of nearly 100% corrected words and negligible uncorrectables. This demonstrates
possible behavior; it does not validate a MERLIN run with a different capture.

Pages 208–210 describe 8 MHz ingress in 96/192 MHz channels (94/190 MHz modulated
spectrum), varying ingress level. Page 209 distinguishes notch MER from whole-
channel MER and reports a large CM/analyzer discrepancy for this experiment.
Page 207 explains pilot-based CM versus data-carrier analyzer measurement.
Do not transplant plotted thresholds between those reference planes.

Section II.4, p.215, retains the observation that LTE ingress occupying more than
approximately 5% of OFDM bandwidth makes performance impact more likely. This is
not 5% below-threshold data carriers and not an unconditional UFEC trigger. Preserve
notch width, level, modulation, channel width, noise floor and measurement method
when comparing experiments. No universal unusable-carrier count is established by
these reviewed sections. An RxMER-only noise model does not reproduce every LTE,
impulse, correlated-noise or synchronization impairment in the laboratory setup.

## Decoder outcomes and capacity references

Sections 5.4 and 9.3 distinguish downstream LDPC/BCH outcomes. Page 184 describes
correctables as pre-decoding LDPC syndrome failure followed by BCH success, and
uncorrectables as BCH failures. Audit MERLIN counter definitions against this
before claiming field-counter equivalence: hard-decision changes or LDPC rejection
alone need not be the same classification. Keep independent source comparison too.

Page 184's cited code example has overall rate 0.8785; spectral efficiency includes
code rate times bits per QAM symbol. The discussion separates ideal Shannon capacity
from actual FEC performance. Do not turn a modulation-only curve into a calibrated
UFEC threshold or treat an analytical margin as decoded counters.

For simulation validation, trace profile mapping, RxMER-to-noise application,
receiver correction/reference plane, demapper reliability, actual LDPC/BCH status
and source comparison. A controlled impairment sweep should retain all these
settings and report actual outcomes, uncertainty and finite observation scope.
