# FEC-03 validation

Status: full pre-push gate passes; FEC-03 complete at the standalone codec boundary.
See the [design](design.md) for the pinned source and acceptance scope.

## Normative verification

- Local original PHY3.1 I20 PDF page115 was rendered and Figures46–47 visually
  checked. Exact leading BCH and trailing LDPC zero coordinates are implemented.
- PHY3.1 I20 §7.5.4/Table23, PDF112–114 establishes the short16200 rate8/9 mother
  code; normal64800 frames are not used. No other mother rates are introduced.
- PHY4.0 I07 §7.2.5.4, PDF77 and §7.3.5.4, PDF134 were read in local versioned
  text; both delegate downstream FEC to PHY3.1 §7.5.4. No4.0 PDF-render claim.
- Existing independently transcribed ETSI polynomial/accumulator test arithmetic
  was moved into a shared test-only oracle and reused. Its published table
  constants/operations are preserved; the original normative tests still pass.
  New oracles are independent computations, not externally produced vectors.

## New tests and outcomes

| Test file | Coverage |
| --- | --- |
| `shortening_geometry.rs` | All1..14232 geometries, PHY burst example dimensions/rates, exact stage coordinates, parity retention, known-zero LLR signs/saturation and malformed/contradictory inputs |
| `shortening_codec.rs` | Independent polynomial/LDPC-accumulator expected words, varied K/patterns, mother syndromes, zero-error recovery, full-mode compatibility and invalid-input recovery |
| `shortening_correction.rs` |0..12 BCH corrections across shortened information/parity, demonstrated40-error BCH failure, five weak wrong soft bits corrected with actual positive LDPC iterations, and bounded non-convergence retaining decisions |

Nine new tests pass. Twenty-one noiseless cases cover seven payload lengths
(K=1,7,8,168,6136,14231,14232) and three patterns. All recover zero payload errors,
with actual LDPC iteration count0. The disturbed-word test requires iterations
above0 and no greater than its50-iteration cap; its recovered payload is correct.
The overwhelmed test reports LDPC failure at the requested one-iteration cap and
retains original-length hard decisions with measured payload errors.

The existing codec logging test now checks shortened-codec filtering/ownership
and enabled/disabled result invariance. Full codec tests and all-target
warnings-denied Clippy pass. Inputs remain borrowed/unchanged, existing root
imports and full-block constructor behavior are preserved, and no dependency or
license change is introduced.

## Runnable demonstration and full validation

`cargo run -p fec-codec --example shortened_codec_loop -- 6136` reports6136
original bits,8104 transmitted bits,8096 omitted zeros and zero recovery errors.
The pre-push gate now runs this in release mode alongside existing simulation
smokes, full workspace/analysis-only tests, helper tests and formatting/Clippy.
The full gate passed:32 helper tests, Bash/ShellCheck, formatting, warnings-denied
all-target Clippy, workspace and analysis-only tests, fixture checks, release
builds and all simulation smokes. The shortened release example reports zero
errors with the dimensions above. Project-local Cargo/Rust1.85.1 was used.

Local-only generated logs and the rendered reference page belong under
`docs/development/local/fec-shortening/`; private PDF/image evidence is not shipped.
Validation is on the local Linux host/toolchain; no new Windows/macOS or external
DOCSIS interoperability claim is made. QAM-03/QA-05 remain separate.

Current integration note: QAM-03 has since connected explicit shortened words
through bit processing, received NCP length discovery and waveform samples. See
the [QAM-02/03 design](../qam-completion/design.md); the codec results above remain
the FEC-03 milestone's validation record.
