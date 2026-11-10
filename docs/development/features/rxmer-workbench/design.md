Current follow-up: [Qt closed-loop and persistence](../qt-closed-loop/design.md)
connects supported receiver results, pause/run dispatch and project files. Older
pending statements below describe the original implementation stage. NCP matched
I/Q and generalized observation selection remain pending.

# Shared channel measurement configuration

Status: shared Channel Estimate/RxMER editors, title/File labels and About are
implemented. Static/power-average/hold/interpolated-power Rust preview operations
are implemented. Full-loop UI dispatch, receiver QAM and channel/RX SA wiring
remain pending. See [user guide](../../../user/channel-measurements.md) and
[validation](validation.md).

## UI-SHELL-04: Main workbench identity and File labels

Use “MERLIN — DOCSIS OFDM Simulation Toolkit” at the top of the main workbench.
Use conventional File → Open…, Save, Save As… labels with keyboard shortcuts.
These commands refer to the complete versioned workbench JSON, containing all
accepted block settings and PNM inputs. Capture pickers stay in block dialogs.
Open/Save implementation remains UI-WB-JSON-01; label changes do not implement it.

## UI-RXMER-01: Input selection and exact association

Click RxMER to open its own configuration dialog, before starting the run. Use
an OS-native picker supporting one or multiple files. Stage original bytes in
Rust, detect content and decode using the pinned reusable decoder/shared JSON
importer; binary RxMER coverage must be verified before claiming support. Show
filename, identity, capture time, frequency bounds/spacing and validity per file.

Validate against the accepted DPD using existing exact ScenarioKey/grid policy:
same device/channel, frequency origin, spacing and carrier correspondence with
required active coverage. Merely overlapping frequency ranges do not suffice.
Preserve nulls/exclusions and provenance; never silently interpolate frequency,
replace missing samples or mix device identities. Report incompatible files
individually. Rejecting a draft preserves the last accepted configuration. Applied
DPD changes invalidate accepted incompatible RxMER configuration and old results.

## UI-CHANNEL-01: Response configuration and channel readiness

Both RxMER and channel-response inputs must be validly configured before Channel
Simulator becomes green. The response input permits an explicit default Flat /
unity mode instead of requiring a capture. Derive one unity coefficient H=1+0i
per physical carrier on the accepted DPD/OCD grid. Display equivalent flat gain
0 dB and phase 0°. A literal linear H=0+0i erases the signal and is not the flat
no-impairment default; do not conflate complex H with gain/phase presentation.
This matches the existing core's unity response initialization. Excluded carriers
retain their existing role rules. Never relabel this synthetic setting as a
measured channel estimate or as the receiver's estimated H-hat.

Once a compatible grid exists, the explicit default unity configuration counts
as a configured response; no PNM file is required for this mode. Imported response
mode must pass exact identity/grid/coverage and nonzero-active-gain validation.
RxMER requires accepted compatible capture(s) and a valid combination/schedule.
Channel Simulator readiness derives jointly from these inputs and its supported
channel settings. Changing DPD/OCD revalidates dependencies and invalidates old
results. Missing prerequisites are yellow; invalid accepted settings are red with
reasons. Rejected editor drafts preserve the last accepted configuration. This
is configuration readiness, not proof that full-loop dispatch is implemented.

## Reuse review: pinned PNM Viewer decoder and Qt presentation

MERLIN already depends on pnm 0.3.6 at revision
5fbd5561f3e06b73486546b5ffa520279080a5eb. Read-only inspection of the cached exact
revision confirms `CmDsOfdmRxMer::parse`, `values_db` and `frequencies_hz`, and
`analysis/rxmer/aggregate.rs` provides Series/GroupKey capture grouping. Reuse
these public Rust inputs instead of writing another binary decoder. Actual MERLIN
adapter/fixture tests are still required before claiming binary import support.

Suitable reference Qt components include RxMerCollectionView.qml,
RxMerCollectionTree.qml and RxMerPlaybackControls.qml. Adapt the compatible
presentation portions with original GPL notices/provenance, not viewer retrieval
or wall-clock playback behavior. No source vendoring, viewer process or checkout
runtime dependency is required; Cargo already fetches the exact library source.
Reusable parser libraries remain Apache-2.0, MERLIN desktop GPL-3.0-only.

The existing viewer aggregate explicitly uses arithmetic_db_valid_readings and
omits unavailable readings from its count. That is a display-statistics policy,
not the proposed MERLIN equal-weight linear-error-power/strict-validity model.
Do not pass it off as MERLIN simulation averaging. DPD association, accepted/draft
lifecycle, simulation combination/symbol scheduling and readiness remain MERLIN
responsibilities; reuse its existing core instead of duplicating DSP in QML.

## UI-RXMER-02: Proposed combination modes

- Single capture: static per-carrier residual model for the entire run.
- Average captures: one equal-capture-weight residual-power average per carrier,
  held for the run. For MER M in dB, use v=10^(-M/10), average v, and derive the
  equivalent MER as -10 log10(mean v). This is an application noise-model choice,
  not a sample-count-weighted aggregate of original error vectors. Default strict
  required-carrier validity across contributing captures; no silent null dropping.
- Sequence / hold: apply ordered captures at explicit physical-symbol offsets,
  holding each until the next boundary. Reuse the existing piecewise timeline.
- Sequence / interpolate: linearly interpolate normalized residual power between
  adjacent snapshots at symbol boundaries, never interpolate dB or frequency grids.
  The new shared Rust preview operation is independently tested; the production
  runner timeline remains piecewise constant until explicit dispatch integration. Synthesized points must retain their source identities and
  playback policy separately from original capture timestamps.

The profile/layout remains frozen. RxMER-only combination must not average or
replace a separately selected channel response H, or add a second noise floor.
Each mode states the existing conditionally independent Gaussian residual model;
RxMER does not establish burst/noise correlation or actual between-capture history.

## UI-RXMER-03: Proposed playback controls and lifecycle

For multiple captures, display an explicitly ordered list and a positive integer
“OFDM symbols between captures”. With interval K, boundaries are 0,K,2K,… and
elapsed time is K*(N+Ncp)/Fs. At the current 25 kHz / CP512 geometry, K=1000
corresponds to 42.5 ms. This is an illustration, not a specification default.
Show Rust-derived elapsed time and total transition span. Preserve recorded times;
user-set playback spacing is synthetic timing, not measured timing. Timestamp-based
playback can later expose the existing epoch scheduler as a separate option.

Use explicit order with timestamp sort/review and manual reordering; never infer
order from filenames. Initial end policy: hold the final capture, no extrapolation
or implicit looping. Reject unsupported/out-of-duration schedules instead of
silently truncating them. Keep allocation/transition bounds at least as strict as
the existing 4096-transition/4-million-retained-sample limits, documenting any lower
UI limits. Validate every selected capture and future schedule before execution.

Apply freezes configuration; Cancel preserves accepted state. Accepted compatible
configuration makes RxMER green; missing configuration stays yellow and actual
incompatibilities are red with reasons. Full-run Start requires all configured
blocks plus dispatch readiness. Do not open another mandatory prompt on every
Start after successful Apply. Current SA “Run simulation” remains TX-only and is
not affected by RxMER until channel/full-loop UI wiring is implemented.

## UI-QAM-01: Receiver constellation and shared SA reference planes

The SA is reusable across eligible time-domain sample taps. Its current TX-output
connection precedes RxMER impairments, so adding RxMER does not change that TX
spectrum. Future channel-output/RX-input connections include the impairment and
may show a changed noise floor, per-frequency power and spectral shoulders, even
when the occupied OFDM shape looks similar. Never promise that RxMER has no SA
effect or apply impairments just to a plotted trace.

The main QAM analyzer belongs at the receiver, after FFT/equalization and before
hard decisions. Use the existing bounded paired collector over `rx.fft.bins` and
`rx.equalized.iq` for matched before/after views. A slicer/post-FEC view would hide
noise by replacing observations with ideal decisions. Optional TX QAM is a clearly
labeled reference, not received signal evidence. Reuse existing Rust collectors;
no QML synthesis of random constellation clouds or duplicate equalizer logic.

Preserve same carrier/time pairs, receiver correction mode and normalized units.
Select one carrier or an explicitly compatible same-modulation group; keep mixed
QAM orders, pilots and PLC separate. Include failed-codeword observations already
eligible under the collector, not just successful decodes. Bounded symbol/time
windows let users inspect changes between RxMER epochs without smearing an entire
run into one unexplained scatter cloud. Future selection must expose the active
playback interval/policy and synthesized interpolation status honestly.

Averaged RxMER produces a static per-carrier noise model. Sequenced/interpolated
RxMER changes modeled residual variance over time; received QAM spread should
reflect those changes. It does not reconstruct captured I/Q or establish real
burst correlation. Channel/RX SA and QAM UI wiring remain pending; existing Rust
paired constellation collection is implemented and documented in
[the receiver guide](../../../user/impairments/paired-constellation.md).

Receiver navigation, NCP/PLC/profile selection and SA test-point routing are
expanded in the [Receiver Workbench design](../receiver-workbench/design.md).

## Implementation order and verification

1. Review this proposal, then implement the title/File-label corrections.
2. UI-RXMER-01: single-file decode, association, staged editor and static mode.
3. Add multi-file validation and equal-weight power-domain averaging.
4. UI-RXMER-03: expose existing hold scheduling and visible symbol/time controls.
5. Implement power-domain interpolation in shared Rust with independent boundary,
   missing/invalid-input, cancellation and observation-invariance tests.
6. Wire accepted RxMER into full channel runs with continuous RNG/source/history;
   verify static/identical-capture invariance and no double-counted impairments.
7. UI-QAM-01: connect bounded receiver pre/post-equalization constellation views
   and eligible channel/RX SA taps, with carrier/modulation and time selection.

Each step updates user guides and actual dark VM screenshots after implementation.
Use scoped tests/builds and retain caches; original planning did not compile.
Implementation validation is recorded separately.

Existing references: [measurement playback](../../../user/impairments/measurement-timeline.md),
[architecture](../../../architecture.md#temporal-measurements-and-applied-profiles),
[UI checklist](../../ui-checklist.md). UI owns presentation; all scientific input,
combination, scheduling, persistence and validation stay in Rust.

## Implementation scope (UI-RXMER-01/02/03, UI-CHANNEL-01)

Implement one reusable measurement viewer in separate RxMER/response dialogs,
backed by a focused Rust adapter with independent accepted/draft states. Use the
existing pinned parsers for binary decoding and add standalone shared JSON imports.
Original bytes/hashes remain in bounded Rust storage. A core playback model owns
static/linear-power-average/hold/interpolated-power queries with source indices,
never falsifies capture timestamps, and exposes bounded Rust-generated chart views.
The adapter retains accepted settings for future full channel dispatch, which is
explicitly not connected in this task. Channel block opens the same frozen-input
summary/editor; green represents valid configuration, not a running loop.
Native multi-select uses the existing isolated GTK helper. Limits: 64 input files,
64 MiB original bytes combined, 4 million decoded carrier values; each file ≤16 MiB.
Reference Qt tree/Canvas patterns come from the pinned viewer revision already
recorded above; adapted GPL components preserve PNM Viewer copyrights.

## UI-SHELL-05: About, title capitalization and rounded identity

About displays the active MERLIN icon, compiled package version and project link.
GPL, Apache and NOTICE links open a separate offline LicenseDialog with Rust-
whitelisted embedded text, never arbitrary file paths. AboutInfo is its focused
adapter. Title bars use initial capitals while preserving technical acronyms;
this rule and matched rounded light/dark icon silhouettes are recorded in the
canonical coding-style skill and existing discovery links. The icon PNGs are
rendered from source SVG wrappers with a shared border-following rounded clip,
preserving interior artwork and identical alpha masks. Desktop, side and About
all consume the same pair. Image-tool background-removal candidates were reviewed;
the final assets use the original artwork and shared deterministic SVG outline.
