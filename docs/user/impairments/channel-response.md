# Supplied channel response and equalization

The imported DS channel-estimation coefficients represent response **H**. Local
CM-SP-CM-OSSIv3.1-I27-250219 AnnexD.2.6, PDF166, states that reciprocals of these
coefficients are typically used as downstream equalizers. They are scaled to
approximately unit average power and do not establish calibrated absolute path loss.
PHY3.1 I20 section9.3.4 also describes this interpretation.

```mermaid
flowchart LR
    SOURCE["Supplied per-carrier H"] --> PLAN["Exact-frequency response plan"]
    TX["Ideal bins x + post-equalization residual e"] --> APPLY["Multiply by H"]
    PLAN --> APPLY --> SAMPLES["IFFT, CP/window and channel sample output"]
    SAMPLES --> FFT["RX framing and FFT"] --> EQ["Known-reference 1/H"] --> RESULT["Recovered x + e"]
    PLAN --> EQ
```

The model is explicitly symbol-diagonal and synchronized:

\[
 Y_k=H_k(X_k+E_k),\qquad R_k=Y_k/H_k=X_k+E_k.
\]

This puts an imported **post-equalization** RxMER budget at its stated reference
plane. Adding equal-strength receiver noise independently and then dividing by H
would amplify that noise again and double-count measured degradation. H-only runs
can select noiseless residuals; measured CLI requests may omit response selection
to use unity H. Zero, unavailable or nonrepresentable active gains reject.

In supplied-channel reference mode, H is independently copied into the receiver
to calculate correction 1/H. In existing pilot-estimated mode, the channel still
applies H, but the receiver derives its own H-hat from received pilots and uses
1/H-hat instead. Neither estimator uses transmitter payload truth.
The channel and receiver own
separate transform/window/framing state, and channel input/output plus RX FFT and
equalized test points expose the resulting stages.

**[MCE](../receiver-estimation/mce.md)** is Off by default in Receiver Configuration.
Off uses the default supplied-H reference workflow; On uses received-pilot channel
estimation and reliability. Supplied H impairs both modes identically. Existing
pilot-estimated mode keeps its separate calibrated-noise contract.

The default diagonal model applies H before per-symbol synthesis and does not
model continuous channel-memory leakage, ISI/ICI or long echoes. The explicit
[Waveform Channel filter](waveform-filter.md) adds static causal sample filtering,
with synthetic paths or a bounded CM-response reconstruction. Reconstruction gaps,
causal shift, truncation and fit errors are explicit; a finite band-limited capture
is not a unique physical impulse response. MCE does not guarantee correction of
echoes beyond the usable guard interval.

## Paired constellation observation

The [paired-constellation collector](paired-constellation.md) retains the same
payload cells before and after this equalizer, with a default target of8192
pairs. Existing raw FFT and equalized boundaries feed an independent diagnostic
inverse interleaver; received NCP ranks select payload samples for bounded
Rust/API/CLI export. This uses supplied H and its reciprocal, without estimating
H from pilots or changing the channel/decoder computations.

If coefficients explicitly represented correction C instead of response H,
channel reconstruction would use1/C and RX would use C. For C=I+jQ:

\[
 C^{-1}=\frac{I-jQ}{I^2+Q^2}.
\]

The current standard PNM input represents H. Its meaning is not inferred from
a filename or changed to match a diagram. The generated constellation is
measurement-conditioned: measured H shape plus synthetic residuals consistent
with RxMER, not the original field-noise realization. Preserve matching grids,
source epochs and explicit tolerance for nonsimultaneous captures.
