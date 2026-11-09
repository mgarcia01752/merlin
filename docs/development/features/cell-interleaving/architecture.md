> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/cell-interleaving/`
> Logs, captures and specification images are not published.

# Cell time/frequency interleaving

Pinned authority: CM-SP-PHYv3.1-I20-230419 §7.5.6, PDF129–133;
Figures55–58 inspected in original PDF. AppendixI, PDF211–212 is informative,
used as a cross-check, not sole authority. Actual BPSK/boost and waveform deferred.

## Ownership and API

signal_processing/interleaving owns validated settings, immutable frequency
permutation and bounded generic time-delay buffers. These support I/Q, tags and
soft vectors without FEC dependencies. A combined symbol operation orders TX time
then frequency; RX frequency inverse then complementary time delays.
transmission/layout/pilot_reference owns physical/reduced-domain association and
128-symbol pilot reference, computed from CarrierLayout and inverse operations.

NI in1..spacing maximum, M in1..spacing maximum. J=ceil(NI/M), padded JM.
Branch b has bJ FIFO positions, clocked J times each symbol, so actual cell at
reduced rank p has b=p mod M and TX delay b whole symbols. RX uses M-1-b,
therefore matched pair delay D=M-1 symbols. Dummy cells only pad end; no dummy
appears at a valid rank after steady state. Generic Option<T> marks startup/drain,
never fake zeros. Consecutive indices and length checked before any mutation.

Frequency: K=ceil(NI/128), C=NI-128(K-1) in1..128.128-row bit reversal;
column rotation positive row displacement; GF(64) register17, next=(old>>1)
XOR ((old&1)<<5) XOR ((old&1)<<4). Last column never rotates. Column bit
width ceil(log2K), ordinary bit reversal for powers of2; out-of-range reversal
falls back to original column for other K. The text's C=0 aside contradicts
its formula for exact multiples; use C=128, also consistent with AppendixI.

Pilot reference: reduce static physical bins to Data/SP only. For postfrequency
pattern S[t], apply frequency inverse to obtain U[t]. Preinput pilot tag
P[t,p]=U[(t+(p mod M)) mod128,p]. This compensates TX delay and chooses correct
phase from first post-PLC-preamble physical symbol. Equivalent to inverse time
reference with matched-pair delay removed. Startup unknowns kept distinct; validate
physical pattern only once TX branch history filled. No actual pilot signs yet.

## Validation

Independent frequency index vectors, seed/tap/direction boundaries, full/partial
and non-power-of-two column counts; bijections and inverse. Exact delayed tags
for all depths, dummy padding, startup/drain and invalid-call atomicity. Physical
pilot map matches after warm-up over128symbols, protected bins never enter domain.
Real NCP and payload recovery with interleavers and non-oracle receiver boundaries.
Document reference model vs external interoperability; one-second runner pending.

## Completed validation

Implemented settings/frequency/time/composition modules plus physical pilot
reference. Generic cell parameter is descriptively named CellValue. Added a
bounded tagged-cell JSON example and public guide with equations/Mermaid.

Five independent/boundary tests pass, plus one integrated160-source-symbol test
at both spacings and maximum depths16/32. Each mode recovers four actual
14232-bit payloads with zero errors using real NCP/CRC boundary discovery,
physical pilot association, randomizer reset phase, startup and complete drain.
No actual BPSK pilot/PLC signals or IFFT waveform claimed at this stage.

Full scripts/check-before-push.sh passed on local Linux. No-default-feature
Clippy also passed with warnings denied. Two existing NCP chain-only helpers
were feature-gated to eliminate unused warnings in analysis-only builds. Current
source passed warnings-denied all-target Clippy and the six new tests after the
final generic-name readability edit. Changed Markdown links/example JSON,
git diff --check, and scientific-testing skill canonical/.agents/.claude
validation passed. Test output recorded in checks.log; example.json demonstrates
startup/drain. No new packages, commits, tags or pushes. Windows/macOS runtime
and external interoperability are unverified.
