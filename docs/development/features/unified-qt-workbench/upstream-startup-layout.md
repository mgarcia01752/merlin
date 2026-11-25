# Approved upstream OFDMA startup layout

The user supplied and approved this layout on 2026-11-22 for
[REQ-UIU-10](../../../requirements.md#req-uiu-10) and
[UIU-US-01](../../checklist.md#task-uiu-us-01). It satisfies the visual-input
prerequisite. Implementation continues in dependency order without another
layout approval. UIU-US-01 remains unchecked until its implementation and
[Qt acceptance](validation.md#upstream) pass. The user may fine-tune spacing,
styling and individual drill-ins after the first functional implementation.

The [canonical upstream UI architecture](../../../architecture.md#unified-qt-upstream-scientific-presentation)
owns scientific reference planes and the public API boundary. This record
preserves the approved presentation and connector semantics.

## Subsequent presentation refinement

The latest 2026-11-25 request reverses the canvas to **CMTS Receiver ← Channel
Simulator ← CM Transmitter** and connects CMTS Configuration to CMTS Receiver.
It supersedes the earlier transmitter-only wire refinement described below.
PRBS blocks follow their stages, correction C remains before physical H, and
received C×H stays at the channel/receiver boundary. Backend TX/RX configuration
and scientific processing order are unchanged. See
[UI-US-LAYOUT-02](../../checklist.md#task-ui-us-layout-02) and its
[design](design.md#ui-us-layout-02-reversed-canvas-and-receiver-side-control).
The original approval below is retained as historical input.

Earlier on 2026-11-25 the user requested the same canvas framing as Downstream and
omission of the explicit CMTS Configuration → CMTS Receiver connector. The
visible control route still reaches CM Transmitter; corresponding receiver
configuration remains implicit within the CMTS group. This visual refinement
supersedes the receiver-wire placement below without changing native session
configuration or scientific reference planes. See
[UI-US-LAYOUT-01](../../checklist.md#task-ui-us-layout-01) and its
[design](design.md#ui-us-layout-01-shared-canvas-framing-and-implicit-cmts-control).

## Canvas and blocks

Use the existing downstream Workbench as the visual reference: dark MERLIN
theme, dotted canvas, rounded WorkbenchBlock components, cyan/teal connections
and status indicators. Reuse Connection, WorkbenchDialog, hover/focus behavior,
status components and run controls. Each functional block opens its own drill-in
window. New upstream drill-ins belong under `apps/qt/qml/upstream/`.

Preserve File, Settings and About; persistent Downstream (OFDM) and Upstream
(OFDMA) tabs; and shared bottom Configure, Start, Stop, Pause and status controls.
Remove the large internal MERLIN branding/header to maximize diagram space.
Keep startup simple: QAM plots, spectrum charts, resource grids and detailed FEC
tables appear in drill-ins rather than the canvas.

Arrange the three main functional blocks horizontally, with balanced spacing:

```text
CM Transmitter  ─────▶  Channel Simulator  ─────▶  CMTS Receiver
      ▲                        ▲                        │
   PRBS-TX                   RxMER                      ▼
                                                     PRBS-RX
CM Pre-Eq ──▶ transmit correction at the TX/channel boundary
```

Place a visually separate **CMTS Configuration** group to the left. Route its
control connectors to the transmitter and corresponding receiver/session
configuration. The logical illustration does not prescribe crossing connectors;
use clean, noncrossing routing in the actual Qt scene. PRBS-TX remains a separate
visible block; PRBS-RX sits above or below CMTS Receiver consistently with the
downstream layout. RxMER and CM Pre-Eq are separate connected input blocks.

## Configuration and drill-ins

CMTS Configuration has three clickable sub-blocks:

- **UCD / OFDMA Configuration:** channel numerology, configuration and applicable
  PHY parameters.
- **IUC / Burst Profiles:** modulation, coding and burst-profile selection.
- **MAP / Grants:** resource allocation, timing and scheduled transmission grants.

The CMTS determines the transmission configuration. These blocks are control
inputs and do not originate upstream sample data.

**CM Transmitter** uses that exact heading. Its drill-in contains PRBS source,
FEC encoder, QAM mapper, pilot/resource allocation, frequency interleaving,
CM Pre-Eq, IFFT and waveform output. Separate PRBS-TX retains the downstream
source-block convention.

**Channel Simulator** sits between CM Transmitter and CMTS Receiver and exposes
only admitted upstream channel effects. **RxMER** represents supported
CMTS-measured upstream RxMER and connects to this admitted impairment model.
**CM Pre-Eq** represents measured cable-modem pre-equalization coefficients,
connected at the transmit/channel boundary: correction C is applied before
physical channel H. Drill-ins distinguish C, H and effective C×H. Do not label
CM Pre-Eq as a CMTS channel-estimation capture or invent unsupported combinations
of RxMER, Pre-Eq and channel models.

**CMTS Receiver** obtains observations from the modeled channel. Its drill-in
contains burst detection/framing, timing, FFT, pilot-based channel estimation,
equalization, resource deinterleaving, soft demapping, LDPC decoding and
received-data recovery. Normal receiver operations must not use transmitter
truth as their input.

**PRBS-RX / Verification** connects to recovered receiver output and independently
compares the recovered source. Its drill-in exposes BER, codeword results,
verification status and incomplete recovery.

## Connector semantics and implementation boundary

| Role | Connection meaning |
| --- | --- |
| Solid waveform/data arrows | CM Transmitter → Channel Simulator → CMTS Receiver. |
| Control connectors | CMTS configuration/grants → transmitter and receiver/session configuration. |
| Measurement connectors | RxMER → admitted channel impairment; CM Pre-Eq → transmit correction at the TX/channel boundary. |
| Verification connector | Recovered CMTS receiver output → PRBS-RX. |

Connections follow the accepted backend reference planes. Never suggest that
upstream payload originates at the CMTS. Colors may share the existing cyan/teal
theme, but control, measurement and verification roles must remain distinguishable.

Thin direction-specific CXX-Qt adapters use `merlin::upstream` and genuine shared
contracts from `merlin::common` where appropriate. Python is not used by Qt.
Science, validation, resource positions and receiver behavior stay in the Rust
high-level API. Missing observations require independently validated public API
extensions before Qt consumes them.

## Acceptance and authorization

Reuse existing presentation components; preserve the accepted one-active-run
policy, direction isolation and active-direction Save. After UIU-SHELL-01 passes,
implement the layout and configuration drill-ins, validate actual Qt/QML rendering
and interactions, and reconcile architecture, requirements and checklist evidence.
Test block navigation, Apply/Cancel admission, connector/reference-plane labels,
focus/keyboard behavior and resize constraints. A mock scene or this approval
does not establish runtime acceptance.

The existing continuous execution authorization includes a focused local commit
with `tools/git/git-save.sh --commit-msg` after acceptance. No remote push,
master merge, release or system-wide installation is authorized.
