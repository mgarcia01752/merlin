# RF-POWER-01: Absolute downstream power budget

Status: UI-RF-01 implemented and validated on Ubuntu22.04 x64 / Qt6.8.3. Physical front-end
limits and CMTS RF-port capability modeling remain separate follow-up work.

The original additional-loss model changes relative complex amplitude. Its successful
1000 ft examples do not establish a physical receive-power budget. Normalized
signal and RxMER residuals are attenuated together, and matched correction can
restore both. Absolute launch power, receiver noise floor and front-end limits
were missing before UI-RF-01. This task follows WAVE-CH-TILT-01; it does not undo that model.

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
calibration and unavailable-state handling. These boundaries are now implemented by UI-RF-01 below.

## Calculated channel power in the SA

Every SA route must show calculated Channel Power at its selected tap, including
before and after the amplifier. Total channel power in dBmV is separate from6 MHz
equivalent level and per-Hz density. Integrate linear PSD over explicit OFDM
channel integration limits with consistent FFT/window scaling, not dB averaging
or a nominal bandwidth multiplier. Preserve actual pilot/exclusion and in-band
noise contributions; identify the integration bandwidth. Display zoom must not
change the configured channel-power integral. Rust owns calibration, integration
and both level representations. A gain value alone does not establish absolute
dBmV. UI-RF-01 below implements explicit reference calibration and the independent-noise policy.

## Workbench JSON persistence requirement

The amplifier/attenuator/tilt implementation includes persistence, not just UI
controls. File → Save, Save As and Open must round-trip all accepted parameters
through Rust-owned configuration and restore the same frozen run request:

- Launch-amplifier gain in dB and receiver-side attenuation in dB.
- Tilt mode and signed value, or additional cable preset/version, length and unit.
- Any exposed launch/input level target, calibration and power-reference band/unit
  choices, once their physical meaning is resolved by this design.
- Existing RxMER and DS channel-estimate originals, hashes and playback settings.

New workbenches default to0 dB gain,0 dB attenuation and Tilt Off. Supported older
WB files missing new controls use those same defaults without extra acceptance.
Existing saved additional loss must survive migration. The current versioned
channel_model section only implements additional loss; do not silently reinterpret
it as a new power-model schema. Define/version the extension and explicit supported
migration when implementing it; do not claim the future keys already exist.

Rust validates finite/ranged values, units and schema/section versions before
atomic adoption. Failed or unsupported loads preserve the current workbench.
Cancelled drafts never change saved accepted values. Derived Channel Power,
results, viewports and transient TP/instrument selections are not persisted.

Acceptance requires scoped tests of nondefault Save/Open round trips, legacy
zero-gain/zero-attenuation/Off defaults, preservation of existing tilt/cable/input
settings, malformed/unsupported version rejection and equality of restored
parameters in the prepared run request. Update the user guide with actual file
behavior when implemented. This persistence extension is implemented by UI-RF-01 below.

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
presentation and explicit power conditioning are implemented by UI-RF-01 below.

## Layout symmetry rule

The revised mockup derives inline positions from one16 px edge-to-edge gap,
centers the complete row with equal outer margins and uses one signal centerline.
Input/model/output blocks share dimensions; TP circles share a diameter. RxMER
and channel-estimate blocks match size and sit50 px above/below Channel Model.
Captions align under their symbol bounds. This corrects the uneven gap after the
second TP without inventing extra taps. General UI rules are maintained in the
coding-style skill. Rendering and geometry were checked; this remains a proposal.


## UI-RF-01: Implemented scalar conditioning and channel instruments

The runtime follows the compact layout: straight signal path, equal16 px visible
edge clearances, inline TP circles before/after a clickable amplifier triangle,
a clickable variable attenuator, and matched RxMER/DS-estimate branches. Input
and output retain instrument access. Separate amplifier and attenuator QML windows
use X/Close to cancel and Apply to accept. Defaults remain0 dB/0 dB/Tilt Off.
All three channel taps use one TapQamDialog and the existing shared SA window.
Receiver output selectors retain actual received role/bin/symbol choices; input
taps display retained physical bins with original coordinates. Missing data is
unavailable. After-amplifier samples/bins are the exact scalar transform of the
retained TX boundary using the frozen core gain, not display-only point movement.

The core PowerConditioning model composes amplifier gain and receiver attenuation
with imported H and additional loss. Existing measurement playback and response
transitions preserve conditioning. RxMER retains its post-EQ reference. An optional
independent receiver noise density is added after attenuation; matched correction
cannot restore the SNR lost against this fixed floor. The residual and independent
floor contributions both enter calibrated LLR variance, including playback changes.
The independent floor is a frequency-diagonal Gaussian model synthesized through
OFDM symbols, not a continuous analog front-end/ADC noise implementation.
No independent floor is enabled implicitly; user must account for noise already
represented by measured RxMER. Noise is synthesized through the existing Rust
waveform path. No frontend clipping, ADC model, physical plant inference or automatic
AGC is fabricated.

Electrical calibration is explicit and optional. The input DATA reference is in
dBmV per6 MHz, with unit-mean-square normalized FFT data cells and the existing
unitary transform at204.8 MS/s. If L is the selected reference, the conversion
is mV² per normalized sample unit² = 10^(L/10) * 204.8 MHz /6 MHz. Complex envelope
power means rms voltage squared; no extra sqrt(2) factor is introduced. Pilots,
exclusions and tilt remain in actual PSD. Imported H is a relative multiplier,
not proof of absolute measured insertion loss. Without a selected reference,
legacy experiments remain explicitly relative and never claim calibrated dBmV.

An amplifier may use explicit gain or an alternative nominal DATA output target,
requiring calibration. The0..60 dB gain/attenuation bounds and−120..+60 dBmV/6 MHz
reference/target range are numerical model choices, not a universal CMTS port
capability. Independent noise accepts−200..−30 dBmV/Hz subject to normalized
variance bounds. Receiver level warnings use the user's±12 dBmV/6 MHz band-average
engineering target; they do not manufacture decoder failure or model composite
wideband RF input. Per-subband electrical limits, overload/clipping and detailed
CMTS port loading remain RF-POWER-01 follow-ups.

Channel SA integrates full-resolution linear PSD over the fixed lowest-through-
highest active bin band, including internal gaps, actual pilot energy, noise and
window leakage. It reports total Channel Power, the explicit integration bandwidth
and separately a band-average6 MHz equivalent level. These are independent of
zoom. Completed-run TX/RX time-domain SA routes share this formatter. FFT-bin
views are still separately labeled bin power; transmitter-only preview remains
relative. Channel output QAM uses actual raw RX FFT cells before equalization,
never ideal decisions or supplied H in place of received data.

Persistence uses a new `sections.channel_power` object with schema_version1 and
`conditioning` containing all PowerConditioning fields. The existing
`sections.channel_model` schema_version1 additional-loss section is unchanged.
Root WB schema remains1; supported legacy workbenches missing channel_power use
zero gain/attenuation, no electrical reference/independent noise, preserving saved
additional loss. Unsupported section versions, unknown fields and invalid numeric
settings reject before adoption. Restored typed values feed the frozen run request.
Source captures, hashes and measurement playback options remain unchanged.

Source ownership: core `signal_processing/channel/power.rs`, impairment planner,
waveform link and measurement transitions; Qt `inputs/channel_model.rs`,
`instruments/channel/` and existing measurement bridge. The separate dialog files
live under `qml/workbenches/channel/`; shared tap QAM under `qml/instruments/qam/`.


## UI-RF-01 completed validation

- 14 scoped core tests pass: electrical reference/integration/range tests,
  waveform conditioning/independent-noise tests and measurement playback. The
  fixed floor lowers measured MER by10 dB for10 dB attenuation; matching gain
  restores MER and recovered bits. Playback preserves conditioning and identical
  outcomes for identical interpolated residuals. Malformed PSD geometry rejects.
- 4 actual CLI channel tests pass, including power settings in the shared input
  contract, successful payload recovery and existing no-clobber/schema behavior.
- 23 Qt adapter tests pass, including legacy defaults, nondefault WB power/loss
  round trips, unsupported section rejection and restored prepared-run values.
  Tap checks verify an exact6 dB SA change, exact scalar I/Q gain and a power
  integral unchanged by display zoom. The existing full-loop test remains green.
- 29 QML checks pass without warnings, including independent Apply/Close/X drafts,
  TP selection, source routing and equal spacing/centerline geometry. All50 QML
  files are registered exactly once; all QObject files share the required directory.
- Root/Qt formatting, warnings-denied core/CLI and Qt Clippy, and the native build
  pass. A no-default-feature core check passes with the existing unrelated
  unused SymbolPacker.coded_bits warning; that packer source was not changed.
  No unrelated full workspace suite was run. Cargo targets and profiles were kept.
- A separate native VM instance loaded the shipped single WB, accepted6 dB gain,
  6 dB attenuation, a0 dBmV/6 MHz nominal DATA reference and−150 dBmV/Hz independent
  noise. Native Save As/Open restored the same parameters. Its completed run
  compared1,223,952 bits with0 errors. The post-amplifier SA reported22.53 dBmV
  total,7.59 dBmV/6 MHz average over187 MHz; the actual raw output QAM and received
  selectors were inspected. Native logs contain no Qt/QML warnings.
- Actual dark staging, amplifier, attenuator, SA and output QAM captures update
  the user guides. A new CLI example uses the same typed conditioning model.
  Source PNM captures and tracked demo workbenches were not modified. Temporary
  GUI processes were closed and the existing user's instance was preserved.
- Skill validation/discovery links, maintained documentation paths, example JSON
  and diff whitespace pass. Logs and temporary native saved WBs stay ignored under
  docs/development/local/qt-source-organization/. The latest executable is
  apps/qt/target/debug/merlin-qt (26229920 bytes). Other platforms and installed
  lifecycle behavior were not revalidated for this change.
