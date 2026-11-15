# PNG asset audit

UI-DOC-02 reviews the 99 PNGs tracked at the start of this task. There are
82 workbench images, nine scientific figures and eight branding assets.
The result is 21 refreshed native screenshots, two unused images removed,
four historical proposals retained and 72 other assets retained.

All replacement screenshots are unedited native Ubuntu Qt window captures from
MERLIN 0.2.9. They show actual inputs, previews or simulation results. The
single workbench supplies completed QAM and spectrum results; multi-rxmer
supplies the five-input average; multi-rxmer-2 supplies the 132-input replay
and Designer examples. Designer changes were confined to a disposable draft.
The calibrated power capture uses an ignored copy of the single workbench
with a 0 dBmV/6 MHz input reference, 6 dB gain and 6 dB attenuation.
No shipped workbench or capture input was overwritten.

No light-theme UI PNGs exist in this inventory. The three light branding
variants were reviewed and retained. Scientific figures document numerical
analyses; the four labeled proposals remain historical design evidence.
Some screenshot paths intentionally share the same native view where their
documentation uses the same instrument state.

## Per-file outcomes

| PNG | Outcome | Review |
| --- | --- | --- |
| [channel-estimation-rx-pilots.png](../../../images/channel-estimation-rx-pilots.png) | Retained scientific | Numerical analysis figure, not a desktop UI capture. |
| [channel-estimation-rx-plc.png](../../../images/channel-estimation-rx-plc.png) | Retained scientific | Numerical analysis figure, not a desktop UI capture. |
| [constellation-ce-only-after.png](../../../images/constellation-ce-only-after.png) | Retained scientific | Numerical analysis figure, not a desktop UI capture. |
| [constellation-ce-only-before.png](../../../images/constellation-ce-only-before.png) | Retained scientific | Numerical analysis figure, not a desktop UI capture. |
| [constellation-ce-only-paired.png](../../../images/constellation-ce-only-paired.png) | Retained scientific | Numerical analysis figure, not a desktop UI capture. |
| [constellation-pre-ce.png](../../../images/constellation-pre-ce.png) | Retained scientific | Numerical analysis figure, not a desktop UI capture. |
| [merlin-emblem-dark.png](../../../images/logo/merlin-emblem-dark.png) | Retained branding | Current approved artwork. |
| [merlin-emblem-light.png](../../../images/logo/merlin-emblem-light.png) | Retained branding | Current approved artwork; light variant reviewed. |
| [merlin-icon-dark.png](../../../images/logo/merlin-icon-dark.png) | Retained branding | Current approved artwork. |
| [merlin-icon-light.png](../../../images/logo/merlin-icon-light.png) | Retained branding | Current approved artwork; light variant reviewed. |
| [merlin-logo-dark-alternate.png](../../../images/logo/merlin-logo-dark-alternate.png) | Retained branding | Current approved artwork. |
| [merlin-logo-dark-transparent.png](../../../images/logo/merlin-logo-dark-transparent.png) | Retained branding | Current approved artwork. |
| [merlin-logo-dark.png](../../../images/logo/merlin-logo-dark.png) | Retained branding | Current approved artwork. |
| [merlin-logo-light.png](../../../images/logo/merlin-logo-light.png) | Retained branding | Current approved artwork; light variant reviewed. |
| [paired-constellation.png](../../../images/paired-constellation.png) | Retained scientific | Numerical analysis figure, not a desktop UI capture. |
| [pilots-ce-only-paired.png](../../../images/pilots-ce-only-paired.png) | Retained scientific | Numerical analysis figure, not a desktop UI capture. |
| [plc-ce-only-paired.png](../../../images/plc-ce-only-paired.png) | Retained scientific | Numerical analysis figure, not a desktop UI capture. |
| [about-dark.png](../../../images/workbench/about-dark.png) | Refreshed | Current product version and About actions. |
| [about-menu-dark.png](../../../images/workbench/about-menu-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-amplifier-dark.png](../../../images/workbench/channel-amplifier-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-attenuator-dark.png](../../../images/workbench/channel-attenuator-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-combined-response-dark.png](../../../images/workbench/channel-combined-response-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-inputs-ready-dark.png](../../../images/workbench/channel-inputs-ready-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-instrument-icons-dark.png](../../../images/workbench/channel-instrument-icons-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-loss-dark.png](../../../images/workbench/channel-loss-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-model-dark.png](../../../images/workbench/channel-model-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-output-qam-dark.png](../../../images/workbench/channel-output-qam-dark.png) | Refreshed | Small constellation points and TX-reference/nearest MER. |
| [channel-power-sa-dark.png](../../../images/workbench/channel-power-sa-dark.png) | Refreshed | Calibrated amplifier tap with PLC overlay. |
| [channel-power-staging-proposal-dark.png](../../../images/workbench/channel-power-staging-proposal-dark.png) | Retained historical | Labeled design proposal, not a current application screenshot. |
| [channel-qam-mer-dark.png](../../../images/workbench/channel-qam-mer-dark.png) | Refreshed | Shared channel QAM capture with current MER labels. |
| [channel-response-capture-dark.png](../../../images/workbench/channel-response-capture-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-response-flat-dark.png](../../../images/workbench/channel-response-flat-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [channel-workbench-proposal-dark.png](../../../images/workbench/channel-workbench-proposal-dark.png) | Retained historical | Labeled design proposal, not a current application screenshot. |
| [dpd-design-dark.png](../../../images/workbench/dpd-design-dark.png) | Retained historical | Labeled design proposal, not a current application screenshot. |
| [dpd-input-dark.png](../../../images/workbench/dpd-input-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [dpd-profile-bandwidth-dark.png](../../../images/workbench/dpd-profile-bandwidth-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [dpd-profile-overlays-dark.png](../../../images/workbench/dpd-profile-overlays-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [dpd-profile-plot-dark.png](../../../images/workbench/dpd-profile-plot-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [mce-receiver-configuration-dark.png](../../../images/workbench/mce-receiver-configuration-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [mce-response-comparison-dark.png](../../../images/workbench/mce-response-comparison-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [mce-waveform-channel-dark.png](../../../images/workbench/mce-waveform-channel-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [mce-workbench-dark.png](../../../images/workbench/mce-workbench-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [measurement-mouse-zoom-dark.png](../../../images/workbench/measurement-mouse-zoom-dark.png) | Refreshed | RxMER zoom focused on PLC; caption reconciled. |
| [ofdm-settings-dark.png](../../../images/workbench/ofdm-settings-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [open-examples-native-dark.png](../../../images/workbench/open-examples-native-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [playback-faq-dark.png](../../../images/workbench/playback-faq-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [prbs-tx-dark.png](../../../images/workbench/prbs-tx-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [prbs-tx-file-dark.png](../../../images/workbench/prbs-tx-file-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [readme-workbench-dark.png](../../../images/workbench/readme-workbench-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [receiver-configuration-dark.png](../../../images/workbench/receiver-configuration-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [receiver-fec-codewords-dark.png](../../../images/workbench/receiver-fec-codewords-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [receiver-fec-control-dark.png](../../../images/workbench/receiver-fec-control-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [receiver-fec-dark.png](../../../images/workbench/receiver-fec-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [receiver-profile-performance-dark.png](../../../images/workbench/receiver-profile-performance-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [receiver-qam-earlier-profile-dark.png](../../../images/workbench/receiver-qam-earlier-profile-dark.png) | Refreshed | Earlier profile selection and paired TX-reference MER. |
| [receiver-qam-mer-dark.png](../../../images/workbench/receiver-qam-mer-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [receiver-qam-ncp-dark.png](../../../images/workbench/receiver-qam-ncp-dark.png) | Refreshed | Paired NCP raw/equalized views and TX-reference MER. |
| [receiver-response-dark.png](../../../images/workbench/receiver-response-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [receiver-slicer-dark.png](../../../images/workbench/receiver-slicer-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [receiver-spectrum-dark.png](../../../images/workbench/receiver-spectrum-dark.png) | Refreshed | Receiver-input spectrum with PLC overlay. |
| [receiver-workbench-dark.png](../../../images/workbench/receiver-workbench-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [rxmer-animation-dark.png](../../../images/workbench/rxmer-animation-dark.png) | Refreshed | Playback active with frame position and PLC band. |
| [rxmer-capture-configuration-dark.png](../../../images/workbench/rxmer-capture-configuration-dark.png) | Refreshed | Five-input capture controls and PLC band. |
| [rxmer-designer-dark.png](../../../images/workbench/rxmer-designer-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [rxmer-designer-main-dark.png](../../../images/workbench/rxmer-designer-main-dark.png) | Refreshed | 50 generated captures, three selected within CWB capacity. |
| [rxmer-designer-plc-zoom-dark.png](../../../images/workbench/rxmer-designer-plc-zoom-dark.png) | Refreshed | Imported capture zoomed to continuous PLC region. |
| [rxmer-interpolation-dark.png](../../../images/workbench/rxmer-interpolation-dark.png) | Refreshed | Preview symbol 5 between captures spaced 10 symbols apart. |
| [rxmer-selection-dark.png](../../../images/workbench/rxmer-selection-dark.png) | Refreshed | 23 of 132 selected within the CWB replay capacity. |
| [rxmer-single-dark.png](../../../images/workbench/rxmer-single-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [sa-mouse-zoom-dark.png](../../../images/workbench/sa-mouse-zoom-dark.png) | Refreshed | Mapped spectrum zoom with PLC band and full timing. |
| [sa-timing-proposal-dark.png](../../../images/workbench/sa-timing-proposal-dark.png) | Retained historical | Labeled design proposal, not a current application screenshot. |
| [sa-tx-baseband-dark.png](../../../images/workbench/sa-tx-baseband-dark.png) | Refreshed | Baseband spectrum with PLC overlay and current timing. |
| [sa-tx-mapped-dark.png](../../../images/workbench/sa-tx-mapped-dark.png) | Refreshed | Capture frequency mapping, PLC band and full two-symbol timing. |
| [settings-global-dark.png](../../../images/workbench/settings-global-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [symbol-timing-expanded-dark.png](../../../images/workbench/symbol-timing-expanded-dark.png) | Refreshed | Full duration and enlarged overlap with usable guard. |
| [transmitter-dpd-tree-dark.png](../../../images/workbench/transmitter-dpd-tree-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [transmitter-mapped-dark.png](../../../images/workbench/transmitter-mapped-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [transmitter-message-bytes-dark.png](../../../images/workbench/transmitter-message-bytes-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [transmitter-messages-dark.png](../../../images/workbench/transmitter-messages-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [transmitter-ordered-cells-dark.png](../../../images/workbench/transmitter-ordered-cells-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [transmitter-output-sa-dark.png](../../../images/workbench/transmitter-output-sa-dark.png) | Refreshed | Shared actual TX spectrum, PLC overlay and full timing. |
| [transmitter-physical-bins-dark.png](../../../images/workbench/transmitter-physical-bins-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| `transmitter-qam-interleaved-dark.png` | Removed | Unused obsolete transmitter image; no maintained references. |
| [transmitter-source-dark.png](../../../images/workbench/transmitter-source-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [transmitter-stage-dark.png](../../../images/workbench/transmitter-stage-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [transmitter-workbench-dark.png](../../../images/workbench/transmitter-workbench-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| `transmitter-workbench-proposal.png` | Removed | Unused obsolete transmitter image; no maintained references. |
| [updates-dark.png](../../../images/workbench/updates-dark.png) | Refreshed | Actual up-to-date result; no fabricated available release. |
| [verification-results-dark.png](../../../images/workbench/verification-results-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [workbench-completed-elapsed-dark.png](../../../images/workbench/workbench-completed-elapsed-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [workbench-configuration-profiles-dark.png](../../../images/workbench/workbench-configuration-profiles-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [workbench-configuration-run-dark.png](../../../images/workbench/workbench-configuration-run-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [workbench-controls-dark.png](../../../images/workbench/workbench-controls-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [workbench-entry-dark.png](../../../images/workbench/workbench-entry-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [workbench-header-dark.png](../../../images/workbench/workbench-header-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [workbench-loading-dark.png](../../../images/workbench/workbench-loading-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [workbench-loading-debug-dark.png](../../../images/workbench/workbench-loading-debug-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |
| [workbench-multi-rxmer-dark.png](../../../images/workbench/workbench-multi-rxmer-dark.png) | Refreshed | Five-input bundled multi-rxmer example, average mode. |
| [workbench-running-elapsed-dark.png](../../../images/workbench/workbench-running-elapsed-dark.png) | Retained current | Current controls and layout; no stale feature state identified. |

## Validation

- Built the current native Qt executable successfully (2 min 23 s).
- Captured actual completed simulations and native preview/playback states.
- Decoded every remaining PNG and checked its PNG chunk checksums.
- Checked replacement bytes against their original native captures.
- Checked maintained documentation links and removed-image references.
- Ran `git diff --check`. No Rust or QML source changed; no full test suite run.
- Linux capture evidence does not establish Windows installer or updater validation.

## UI-DOC-03: receiver-mode follow-up

This bounded follow-up supersedes the earlier receiver-configuration retention
decision. Actual dark native Ubuntu captures from the current release executable
were visually reviewed; no shipped workbench or input was saved/modified.

| PNG | Outcome | Review |
| --- | --- | --- |
| receiver-configuration-dark.png | Refreshed | Replace obsolete two-mode dialog with the current supplied CM mode and shared Apply/Cancel actions. |
| receiver-mode-options-dark.png | Added | Actual open dropdown showing all three receiver choices. |
| mce-receiver-configuration-dark.png | Retained | Already shows the current three-mode selector with conditional MCE averaging. |
| rxmer-designer-dark.png | Retained | Current stacked arrows and Save action; corrected the guide to describe two imported captures. |
| rxmer-designer-plc-zoom-dark.png | Retained | Current stacked arrows, Save action and true first-to-last PLC span. |
| rxmer-designer-main-dark.png | Retained | Separate generated 50-capture sequence, three selected within capacity. |
| rxmer-designer-save-dark.png | Retained | Current PNM/JSON save choices and export conversion summary. |

These captures verify Linux appearance only. No new UI source changes occurred
and no rebuild/full Rust or Qt test suites were required. Existing MCE and
Designer images were inspected rather than unnecessarily regenerated.

### API-CPA-01 — CPA editor terminology

Updated `channel-attenuator-dark.png` and `channel-inputs-ready-dark.png` from
the actual QML editor and channel diagram rendered on the
Ubuntu XCB desktop with Qt 6.8.3. This is a component-test capture with fixture
backend settings (editor: zero attenuation; diagram: 12 dB gain, 6 dB CPA;
independent noise disabled), not evidence
of an end-to-end simulated run. The test window uses the application's theme
palette for popups.

## RXMER-DESIGN-13 refresh (2026-11-15)

Rebuilt native Qt VM captures replace the Designer full-span, PLC-zoom and Save
images. They show a new flat sequence, 43 dB baseline, 1.5 dB independent random
variation, seed 41 and an 8 MHz notch centered at 375 MHz with 33 dB depth.
The main-chart image is retained because that UI is unchanged. These are actual
application captures, not mockups; the review closed without exporting inputs.
