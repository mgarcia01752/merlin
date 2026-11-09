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

The supplied H is independently copied into the existing known-reference receiver.
It is not estimated from transmitter payload truth. The channel and receiver own
separate transform/window/framing state, and channel input/output plus RX FFT and
equalized test points expose the resulting stages.

The implementation does not convolve arbitrary H across a continuous waveform.
It applies the diagonal response before per-symbol synthesis; it does not model
channel-memory leakage, ISI/ICI, unknown timing or multipath exceeding the cyclic
prefix. Blind estimation and nonideal equalization remain open in MODEL-05 and
WAVE-RX-05. Constant complex and actual frequency-varying captured responses are
tested without claiming those broader receiver capabilities.
