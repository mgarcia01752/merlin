# Upstream OFDMA workbench

Select **Upstream (OFDMA)** to use the CM transmitter → Channel Simulator →
CMTS receiver diagram. The separate CMTS Configuration group supplies control
information; it is not the source of upstream payload samples. Click a block,
or focus it and press Enter or Space, to open its window. Escape, Cancel and X
close an editor without applying its changes.

## Load a configuration

Click **Configure** or **UCD / OFDMA Configuration**, then **Load Configuration**.
Choose a MERLIN upstream configuration JSON document, such as the
[50 kHz example](../../examples/cli/upstream/us-50khz.json). The native file picker
can be cancelled without changing the current editor. Loading validates the
selected document before replacing the accepted upstream configuration. Invalid
input leaves the previous configuration available. A successful load reopens
the same section with its newly accepted values.

Before a document is loaded, the editors show an explicit missing-configuration
message. They do not invent a complete channel. This action loads configuration
JSON; measured RxMER and CM Pre-Eq capture import remains a separate integration
task.

## Edit accepted settings

The following blocks open editors containing the values admitted from the loaded
configuration:

| Block | Editable settings |
| --- | --- |
| UCD / OFDMA Configuration | Existing preset selection and supported channel numerology fields. |
| IUC / Burst Profiles | Existing data IUC selection and each source burst group's modulation and pilot settings. |
| MAP / Grants | Synthetic grant identity, timing, frame count and existing physical minislot selection. |
| PRBS-TX | PRBS15 source seed. The CM Transmitter window also links to this editor. |
| Channel Simulator | Supported native channel delay, noise, phase, frequency offset and seed fields. |
| CMTS Receiver | Received-pilot estimator policy, reference variance and LDPC iteration limit. |
| CM Pre-Eq | Initial bank identity when an explicit synthetic correction plan exists. |

Settings are limited to the backend's supported modes. The complete loaded source
structure is preserved, including fields outside these editors. Complex channel
and Pre-Eq coefficient banks, events and probe plans remain in that source;
the scalar editors do not replace them. Without an explicit Pre-Eq plan, the
window identifies native unity correction rather than manufacturing a bank.

**Apply** submits changed values for Rust validation. The window stays open while
the candidate is prepared and closes only when it has been accepted. On rejection,
the original diagnostic remains visible and the accepted settings stay intact.
Reopening displays accepted values. **Cancel**, Escape or X discards local edits;
cancelling a pending edit cannot stop another simulation or import.

Configuration inspection does not advance the source or produce a waveform.
A green configuration block indicates accepted settings; it does not establish
received data or a successful simulation. Missing observations remain unavailable.
CM Pre-Eq is transmit correction **C**, the physical channel is **H**, and their
combined response is **C×H**. A CM Pre-Eq capture is not a CMTS channel estimate.

![Accepted receiver configuration in the native dark workbench](../images/workbench/upstream-configuration-dark.png)

## Direction isolation

Switching tabs preserves each direction's accepted settings. One simulation may
run across the application, including a paused simulation. Return to its direction
to control it. File project actions currently remain available on Downstream only;
loading upstream configuration cannot replace the downstream project or its Save
destination. Settings and About are shared.

The upstream execution, measured-input and scientific instrument acceptance gates
remain separate from configuration editing. See the
[workbench overview](workbench.md) for shared controls and presentation limits.
