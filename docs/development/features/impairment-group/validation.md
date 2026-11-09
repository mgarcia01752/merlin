> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/impairment-group/`
> Logs, captures and specification images are not published.

# Five-step impairment group validation

Linux host, pinned local Rust1.85.1 wrapper. Full scripts/check-before-push.sh
passed after the final numerical-owner move:32 helper tests, rustfmt, all-target
warnings-denied Clippy, workspace/analysis-only Rust tests, fixture example tests,
release build, original coding/channel probes plus new import/measured/AWGN CLI
smokes. Added10 Rust tests across importer, CLI, LTE protected ranges and waveform
impairments.480 public relative document links and Markdown fences checked.

Independent sample/FFT noise-power anchors, seed/chunk invariance, zero passthrough,
complex response compensation, per-carrier measured operation, callback budget
invariance, missing active readings, mismatched grids/epochs, unsupported captured
orders, no-clobber output and failed-control partial-result semantics passed.

Actual captured-profile0 layout has PLC neighborhood648–654MHz. LTE center737.5MHz
is outside it. All six3ms seed41 sweep cases completed254 codewords/3614928 PRBS
bits with zero bit/codeword errors. LDPC histograms:
- baseline:254 at0iterations
-1RB:91 at0,163 at1
-6RB:12 at0,242 at1
-12RB:2 at0,252 at1
-25RB:254 at1
-50RB:249 at1,5 at2
BCH-corrected-bit counts were0 in every case. The smallest envelope affects47/7480
source grid positions including shoulders. No15percent usability cutoff exists.

Final CLI run uses explicit captured channel/MAC identity and synthetic-source
label. Baseline mean LDPC iterations0;1RB mean0.6417322834645669; both have zero
LDPC/BCH failures and zero payload errors. The complete1RB summary stayed exactly
identical after moving Gaussian/H arithmetic into the public signal-processing
boundary; elapsed host time was excluded from that comparison.

Normative evidence: local PHY3.1 I20 sections9.3.4/9.3.6 and CM-OSSI3.1 I27
AnnexD.2.6 text. No rendered-PDF or external modem interoperability claim. LTE
loading/shoulders are an engineering Gaussian residual approximation, not actual
LTE time samples. H is a synchronized per-symbol diagonal response; blind/nonideal
estimation, general FIR multipath and temporal/mixed-profile transitions remain
open. No Windows/macOS or hardware validation performed. New crate dependency is
locked Rust SHA-256 for decoded-JSON provenance, not system software.
