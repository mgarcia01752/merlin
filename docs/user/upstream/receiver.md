# OFDMA received-only receiver

The typed receiver accepts legitimate waveform/profile/grant configuration,
a run identity and indexed received complex samples. Its input contains no
source seed, original payload, transmitted grids or actual channel/correction
response. Received pilot observations estimate effective C times H.

```mermaid
flowchart LR
  Y[Received samples] --> T[Configured timing and useful windows]
  T --> F[Shared FFT]
  F --> E[Received-pilot estimate and equalization]
  E --> R[Inverse physical resources]
  R --> L[Soft QAM metrics]
  L --> M[Soft PHY derandomization]
  M --> D[LDPC decisions]
  D --> B[Meaningful bits and coverage]
```

FrameFlatLs uses a frame-wide least-squares response. FramePilotLinear uses
bracketed pilot interpolation with a bounded gap. Missing pilots or a deep
response null yield unavailable cells; they do not trigger supplied-truth repair.
Positive soft metrics favor zero. Shortened known zeros are separate from
missing observations, which become erasures at their original positions.

Input chunks must be contiguous and belong to the accepted run/configuration/grant.
[Optional bounded estimate retention](receiver-estimates.md) exposes actual
completed-frame coefficients without replay or configured-response substitution.
Rejected calls preserve receive state. Explicit gaps and truncated windows keep
logical bit positions; incomplete samples never become a fabricated FFT window.
Decoder convergence, parity, known-zero/fill checks, actual iterations and bit
errors are separate outcomes. Failed words retain their available decisions.

An independent simulator observer compares meaningful recovered bits with its
own PRBS stream. This observer cannot feed expected bits into receiver decisions.
Local internal loops pass both numerologies, all pilot patterns and supported
loading. Complete document/session acceptance now passes; final adapter/backend acceptance remains in the
[checklist](../../development/checklist.md#int-us-rx-01); this is not external
hardware interoperability evidence. See [validation](../../development/features/ofdma/design.md#int-us-rx-01-received-only-receiver-implementation-unit).
