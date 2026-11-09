> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/plc-block-interleaving/`
> Logs, captures and specification images are not published.

# PLC block interleaving

NEXT-09: PHY3.1 I20 §7.5.13.7/Figures65–67/PDF154–156. Original PDFs inspected.
Group384 encoded bits MSB-first into96 nibbles. 4K:8rows x12columns, output
v[t,f]=u[t+12f],12symbols x8carriers. 8K:16rows x6columns, v[t,f]=u[t+6f],
6symbols x16carriers. Output time-major, low-to-high local PLC carrier. No
preamble, data/NCP cell interleaver, randomizer or physical-bin insertion here.
PHY4.0 FDD delegates §7.3.5.13/PDF146 to3.1.

Separate modules: transmission/plc/nibbles for validated grouping; block_interleaving
for permutation, hard and finite soft inverse. Analysis-only available, no codec
dependency. Simulation/plc/interleaving composes typed stages and bounded optional
observations. New IDs tx.plc.coded_nibbles and tx.plc.interleaved use label payloads,
explicit codeword/PLC-cell domains and full grid geometry; selected shape is1D.
Do not mutate completed science based on logging or observation. Phase is validated
for a complete PLC codeword within120data symbols, using existing post-preamble
index0 convention. No CRC or truth acceptance hidden in the permutation.

Tests: fixed full96-coordinate vectors, bijection and all96 nibble basis cases,
MSB vectors and all384 bit basis cases, shape/value/finite rejection, hard and
soft inverse, both complete frames through real LDPC/CRC, observer limits and
invariance. Example plus guide/evidence/architecture/checklist update required.

## Completed verification

NEXT-09 implemented and checked; NEXT-10 remains next. Eight new tests pass:
2nibble grouping,3permutation/soft inverse,1full-frame codec integration and2
observer tests. All384 bit basis and96 label basis inputs match explicit expected
significance/positions; full96-coordinate vectors were independently transcribed
from the rendered diagrams. Both complete message-frame sizes recover correctly
through real PLC FEC with timestamp CRC validation.

Full pre-push gate passed. Analysis-only warnings-denied Clippy passed. After
adding extra raw-dispatcher invalid-label/domain assertions, both observer tests
passed again. Example JSON validates6x16/12x8 rows,96labels/block,32selected labels
per probe instance and distinct spacing configuration IDs. Changed Markdown
fences/local targets, skill validation at canonical/both discovery paths and git
whitespace checks pass. No new dependencies, system packages or external calls.

Updated architecture, checklist, public API/evidence, stage guides, README/build
examples and testing skill. Observer metadata now optionally contains plc_grid;
label events retain full grid metadata with1D selected shapes. Existing pilot
producers continue using the same borrowed/budget contracts. Only two PLC label
producers are wired; message/coded-bit/receiver observer producers remain TODO.

Randomization, preamble,16QAM mapping, physical insertion and actual acquisition
MAC packets remain outside this step. The soft inverse assumes already
unrandomized likelihoods. No waveform or full-channel interoperability claim.
