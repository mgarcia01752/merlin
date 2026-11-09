> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/receiver-configuration/`
> Logs, captures and specification images are not published.

# RX-CONFIG-01 validation

Implemented received-descriptor configuration as separate channel, profile,
collection, complete-configuration and readiness modules. A composed PLC
acquisition receiver connects the existing actual I/Q decoder and MC reassembler
to admission without passing transmitter settings/layout/profile objects.

## Evidence and scope

Pinned local MULPI3.1 I25 sections6.4.40-41, including independent OCD/per-profile
change counts and default/specific DPD precedence. Existing physical geometry
and interleaver validators retain PHY-derived ownership. See the tracked PLC
evidence reference for source mappings. Fixed-session rejection/reset is an
explicit laboratory policy, not dynamic modem activation behavior. Occupied
bandwidth is an inferred minimum validation envelope, not an OCD wire field.

## Validation performed

- Full scripts/check-before-push.sh: PASS, including formatting, all-target Clippy,
  workspace Rust tests, analysis-only tests, 32 helper tests, ShellCheck,
  release CLI build, version consistency and bounded JSON simulation smoke test.
- Analysis-only all-target Clippy with warnings denied: PASS.
- Eleven new Rust tests: five configuration, four profile interpretation,
  two actual PLC I/Q acquisition/integration tests. Both spacings exercised.
- PLC example: both spacings reach Ready, three received descriptors each;
  four completed frames have zero information-bit errors and matched preambles.
- Documentation: 130 local links, anchors, code fences and seven subsystem
  Mermaid blocks checked. Mermaid rendering was not performed.
- Modified scientific-testing skill: canonical and both discovery paths validate.
- git diff --check: PASS.

## Regression found during implementation

The longer acquisition fixture exposed idle bytes inserted into unfinished MC
packet continuations. Pointer-free blocks now use full payload capacity; a new
packet is deferred if there is no room for its pointer and first byte. The
multi-frame actual-I/Q test and existing scheduler/reassembly tests pass.

## Remaining boundaries

Known PLC alignment/spacing remains bootstrap input. Full waveform/payload/NCP
receive orchestration, blind acquisition, timeouts and dynamic DPD activation
remain deferred. WAVE-01 is the next official checklist item.
