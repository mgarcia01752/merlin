> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/pilot-plc-signals/`
> Logs, captures and specification images are not published.

# Pilot stages and observation foundation

Start with reusable stage-result types, a bounded optional observation dispatcher,
and separate pilot sequence/power/continuous/scattered functions. Pilot algorithms
remain independent of simulation observers. A thin simulation adapter observes
already generated immutable stage outputs. PLC content/coding/placement and
physical frame assembly remain separate subsequent work; no empty PLC modules.

Authority: local CM-SP-PHYv3.1-I20-230419 §7.5.15.3–4, Figure84, PDF171.
Pilot LFSR13 bits all ones at physical FFT bin0, polynomial
x^13+x^12+x^11+x^8+1, clock after EVERY FFT subcarrier, including protected bins.
w=0 maps +1+j0, w=1 maps -1+j0; explicit independent examples k1000=0,
k2999=1. Pilot amplitude twice other-subcarrier RMS (power ratio4), not2dB.
Verify exact Figure84 bit/tap orientation before coding. Reset each OFDM symbol,
not each pilot or data-only position. Do not share data/NCP/PLC randomizer state.

Observation: shared borrowed-event model, stage IDs registered for current pilot
stages only, explicit direction/domain/reference/run phase and source/channel
indices. Selection bounds/windows, finite callback and element budgets; no-op
path does not invoke callbacks. Budget exhaustion reported without modifying
science. Callback errors consume dispatch budget and return explicit errors; no
implicit retry or engine mutation. Observer does not receive mutable stage state.
No filesystem, CLI dump command, full-channel runner, Python or Qt added.

Acceptance: exact published pilot bits, independent recurrence/basis/period,
FFT extent and skipped-bin indexing, separate CP/SP roles, boost energy and input
rejection. Observation enabled/disabled/effect/error/budget tests, JSON example,
analysis-only availability for DSP and existing simulation feature for adapter.
Update official architecture and checklist without checking PLC TODOs early.

## Completed implementation and checks

Separate sequence, continuous, scattered and power DSP modules now produce typed
signals. Thin simulation composition and bounded immutable pilot observations are
implemented. Independent diagram recurrence and both published pilot bits pass;
11 new tests cover sequence, power, physical roles and observer behavior. Full
pre-push checks passed on local Linux/Rust1.85.1; analysis-only Clippy passed.
After final metadata-bound and event-shape refinements, the six observer tests
and workspace warnings-denied Clippy passed again. The JSON example dispatched
four events/24 selected elements with reference RMS1 and pilot power4.
Markdown local targets/fences and git diff whitespace checks pass.

Follow-up remains PLC evidence, typed messages and coding, then the remaining PLC
stages and production physical assembly. Additional continuous-pilot position
generation and complete waveform interoperability are not established here.
