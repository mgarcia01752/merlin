# RF-POWER-01: Absolute downstream power budget

Status: planned; no absolute RF calibration or receiver sensitivity implemented.

The additional-loss model changes relative complex amplitude. Its successful
1000 ft examples do not establish a physical receive-power budget. Normalized
signal and RxMER residuals are attenuated together, and matched correction can
restore both. Absolute launch power, receiver noise floor and front-end limits
are missing. This task follows WAVE-CH-TILT-01; it does not undo that model.

## Requirements and evidence

The user proposes an operating target of −12 to +12 dBmV at the CM and a
transmitter ceiling around +60 dBmV. Use explicit bandwidth references:

- CM-SP-PHYv3.1-I20-230419 §7.5.11, Table45, PDF pages146–147 specifies a
  level range −9 to +21 dBmV per24 MHz, equivalent to −15 to +15 dBmV per6 MHz,
  and separately total input power below40 dBmV over54–1794 MHz. The table
  explicitly distinguishes level range from BER/modulation performance.
- Treat ±12 dBmV per6 MHz as the user's engineering target inside that range,
  not a replacement normative requirement or total OFDM channel power limit.
- §7.5.9.1.1, Table42, PDF page139 gives CMTS power capability as
  60 − ceil[3.6 log2(N*)] dBmV per equivalent channel. N* depends on port
  capability/loading under §7.5.9.1. A universal +60 dBmV PSD ceiling does not
  follow from the leading constant. Pin the intended transmitter/port model
  and whether the proposed60 dBmV means total power or a6 MHz equivalent.

These edition-specific findings use the repository text extraction; no rendered
I20 PDF verification is claimed. The official
[CableLabs PNM primer](https://account.cablelabs.com/server/alfresco/4f3782eb-a1d7-4398-bc21-6d12bb70f915),
§5.9, pages97–100, independently explains6 MHz equivalent power, occupied-channel
count, pilot contribution and the difference from total integrated power.

For a flat illustrative spectrum, P_total = P_6MHz + 10 log10(B/6 MHz).
For192 MHz this difference is about15.05 dB. Actual calculations must integrate
linear power using active/excluded carriers, pilot boosts and response; do not
apply a nominal bandwidth correction blindly to tilted or partially occupied bands.
Absolute dBmV levels describe rms voltage in75 ohms; density needs its bandwidth
reference. Raw dBmV, dBmV/Hz and6 MHz equivalent values are not interchangeable.

## Proposed implementation

1. Add explicit TX launch power with a named6 MHz equivalent reference and a
   separately derived integrated channel level. Define complex-envelope/rms
   calibration once in the shared Rust waveform model, with no hidden √2 factor.
2. Compose additional cable attenuation with relative H. Imported channel-estimate
   magnitude is not proof of calibrated insertion loss: require an explicit
   reference/normalization policy before claiming an absolute plant budget.
3. Calculate signal level at RX input before equalization, showing the6 MHz
   levels across the occupied band, integrated power and margin to the chosen
   operating envelope. Keep single-channel power distinct from total RF-port
   composite input, which this standalone simulator does not otherwise contain.
4. Add a separately defined receiver noise floor/noise figure and front-end
   gain/overload policy so attenuation can reduce usable SNR. Preserve the current
   RxMER-only mode. Do not add a physical noise floor on top of measured total
   residual without an explicit reference plane and budget that avoids double counting.
5. Expose calibrated power at existing TX/channel/RX test points, with SA units,
   concise RX-level diagnostics and versioned WB persistence. Keep QAM coordinate
   scaling distinct from RF gain/AGC. Out-of-envelope levels are diagnostics,
   not a fabricated decoder failure; actual modeled samples/FEC determine outcomes.

All power, calibration, noise and limit calculations remain Rust. Qt presents
controls and derived results. Defaults/migration must preserve legacy relative
experiments explicitly; do not silently label existing unit-amplitude runs dBmV.

Before implementation settle transmitter capability/reference, imported-H scaling,
receiver noise parameters and readiness handling for deliberately weak/strong
experiments. Test independent voltage/power/bandwidth conversions, zero-loss and
known-loss budgets, active/pilot/excluded contributions, fixed-noise SNR change,
receiver correction invariance of SNR, malformed bounds and save/restore.

## Proposed amplifier, attenuator and TapPoint layout

The [compact dark proposal](../../../images/workbench/channel-power-staging-proposal-dark.png)
([SVG source](../../../images/workbench/channel-power-staging-proposal-dark.svg))
keeps the signal stages on one horizontal centerline. RxMER and DS Channel
Estimate have straight vertical branches into Channel Model. The amplifier is a
clickable right-pointing triangle; the attenuator is a clickable variable resistive
symbol crossed by a diagonal adjustment arrow. Their symbols open separate draft
settings dialogs directly, with no separate Configure button or block. Receiver
AGC remains a separate receiver function; TX's own launch level stays a TX setting.

Defaults are0 dB amplifier gain,0 dB attenuation and Tilt Off. These optional
controls require no initial acceptance or additional readiness gate at their unity
defaults. Zero dB gain/loss is not0 dBmV absolute level. Close/X cancels drafts;
Apply accepts valid settings. Gain and target output PSD are alternative amplifier
controls, with explicit bandwidth and output limits.

Replace the post-amplifier junction dot with an inline circle marked **TP**
(TapPoint), and add a second TP before the amplifier. Clicking either opens an
**SA / QAM** instrument chooser for that exact plane. Remove the dedicated SA
card and its downward branch. Label chooser/window titles Before Amplifier or
After Amplifier so the source remains clear. Output instruments stay after RX
attenuation. Preserve compact blocks and straight connectors.

Reuse the existing SA window. Current QAM UI has separate TX/input and receiver
wrappers; the planned TP routes must share one QAM window/presentation and obtain
available selectors/roles from the tap provider. Do not infer post-amplifier I/Q
from the plot, apply display-only RF gain, or substitute an earlier tap. Rust
must provide the actual selected boundary with consistent coordinates, units,
calibration and unavailable-state handling. These amplifier/attenuator boundaries
are not implemented yet.

## Calculated channel power in the SA

Every SA route must show calculated Channel Power at its selected tap, including
before and after the amplifier. Total channel power in dBmV is separate from6 MHz
equivalent level and per-Hz density. Integrate linear PSD over explicit OFDM
channel integration limits with consistent FFT/window scaling, not dB averaging
or a nominal bandwidth multiplier. Preserve actual pilot/exclusion and in-band
noise contributions; identify the integration bandwidth. Display zoom must not
change the configured channel-power integral. Rust owns calibration, integration
and both level representations. A gain value alone does not establish absolute
dBmV. This remains planned pending the calibrated model and noise/limit policy.

## Shared instrument organization proposal

Use `apps/qt/qml/instruments/` for shared instrument windows/charts and
`apps/qt/src/instruments/` for their Qt-facing Rust adapters. Group by instrument:
`spectrum`, `qam`, `response`, `slicer` and `fec`; keep each window in its own QML
file. Workbench dialogs own tap selection/navigation and pass a typed source
selection to the instrument, rather than copying windows or calculations for each
stage. Shared controls can remain common to multiple instruments.

Core numerical algorithms remain in their existing Rust signal-processing and
simulation modules, independent of Qt. Moving presentation files must preserve
licenses and update QML registration/imports, module wiring, tests and source links.
Refactor incrementally, preserving existing instruments and data reference planes.
UI-ORG-01 implements the folder organization separately; see the
[organization record](../qt-source-organization/design.md). Shared TP QAM
presentation and the calibrated power model remain pending.

## Layout symmetry rule

The revised mockup derives inline positions from one16 px edge-to-edge gap,
centers the complete row with equal outer margins and uses one signal centerline.
Input/model/output blocks share dimensions; TP circles share a diameter. RxMER
and channel-estimate blocks match size and sit50 px above/below Channel Model.
Captions align under their symbol bounds. This corrects the uneven gap after the
second TP without inventing extra taps. General UI rules are maintained in the
coding-style skill. Rendering and geometry were checked; this remains a proposal.
