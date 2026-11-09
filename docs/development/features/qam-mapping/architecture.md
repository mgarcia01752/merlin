> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/qam-mapping/`
> Logs, captures and specification images are not published.

# Shared QAM references through16K

## Accepted scope

Prepare all pinned downstream data constellations through16384 while initial
channel integration targets256. Reuse PNM Viewer reference resources and Rust
build generation, preserve notices/provenance. No external checkout modifications.

## Implementation

Copied unmodified2/4 control references plus10downstream data tables/scaling and
license/notice. Copied resource hashes match upstream PyPNM provenance. Build
Rust helper adapted from viewer without Delaunay/boundary dependencies; validates
cardinality, integer-coordinate uniqueness, mean energy then emits static arrays.
No runtime table parsing/database. No new Cargo dependencies or system packages.
QamReference now uses static slice rather than fixed16-array. map_symbol takes
ConstellationOrder and SymbolLabel u16; qam_llrs supports all references with
MSB-first bits, positivezero, linearcomplexN0 and clipping100. Legacy16interfaces
share the same resources/calculation. Max-log finite-distance overflow rejected.
Generic demapper is O(order*bits) correctness reference; optimize before sustained
high-order1second throughput workloads. No full-mode claim from mapper alone.

## Evidence and validation

Local PHY I20 AnnexA PDF199/201–206 rendered and inspected. Table54 energies;
Figure108 all64positive-quadrant labels independently transcribed plus reflection
rules from pages199–200 produce256label expected vectors. Other orders have
label0 diagram anchors, complete uniqueness/energy, selected noiseless softbits.
Not exhaustive external high-order interoperability vectors. No full coding
mode enabled outside existing16QAM. Registry mapping state separate from coding.

## Pending

256bit interleaver/demux/randomizer, padding/symbol packing/NCP, time/frequency
interleavers and waveform/inverse receiver. Additional pilot generator stillpending.
