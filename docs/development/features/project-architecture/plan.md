> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/project-architecture/`
> Logs, captures and specification images are not published.

# Official architecture and implementation checklist

Documentation-only task before the next DSP implementation. Preserve pending
cell-interleaving changes and existing runtime behavior. Canonical tracked design:
docs/architecture.md; progress/TODO: docs/development/checklist.md. Development
notes/private evidence remain here, not a competing official architecture.

Source review: codecs expose BCH/LDPC and concatenated results; NCP has separate
CRC and control LDPC; data randomizer, mapper/LLR functions and time/frequency
interleavers have direct APIs. Bit processing currently composes parity/twist/
demux into one permutation and lacks named intermediate outputs. New channel
integration currently lives in tests, not a reusable one-second channel runner.
No common test-point observer/export bus, hard slicer, PRBS, pilot BPSK/boost,
PLC waveform or FFT waveform path exists. Current CLI remains16-QAM experiment.

Document module ownership, stage test-point matrix, typed metadata and bounded
observation design. Mark planned modules explicitly; do not create empty Rust
files. Record next-step PHY evidence/acceptance TODO before implementing pilots/
PLC. Update README and coding-style/public-api routing as appropriate. Validate
Markdown links and modified canonical/discovery skill paths. No Rust changes or
full simulation rerun needed for this documentation-only delta.

## Completed

Consolidated docs/architecture.md into the canonical narrative: current scope,
module ownership, target pipeline, existing-vs-planned test points, observer
metadata/lifetimes/limits, proposed next modules and stage-guide references.
Added docs/development/checklist.md with stable IDs,25 completed and58 remaining
items. The next15 TODOs cover stage contracts and separate pilot/PLC components,
including PLC block interleaving. Hard slicer, PRBS, fine-grained bit boundaries,
production channel orchestration and common observation remain explicitly TODO.
The three-stage count is scoped to the first noiseless waveform milestone.

Updated README/public-API navigation, coding-style stage/test-point guidance and
AGENTS routing to the canonical design/checklist. No DSP implementation was
changed for this documentation task; pending cell-interleaving work is retained.

Validation: relative links/anchors, Markdown fence balance, unique sequential
checklist IDs and git diff --check passed. Coding-style skill validation passed
at canonical, .agents and .claude discovery paths. No Rust tests rerun for this
documentation-only delta; existing previous-stage validation remains separate.
