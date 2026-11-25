# PyPNM QAM reference resources

Unmodified tables and scaling factors copied from PyPNM; see provenance.json
for the source revision and SHA-256 hashes. Retain LICENSE and NOTICE.

Rust build.rs validates the supported tables and compiles normalized static
I/Q lookup arrays into the library. Neither text parsing nor filesystem/database
access happens while opening a capture. Reference points use 1/sqrt(Es);
received s2.13 soft decisions are already normalized and are never rescaled.

8QAM_table.txt is preserved for provenance but intentionally not compiled:
its mean energy is 88, while the supplied 3-bit scaling factor is 10. It is not
one of the DsOfdmModulationType constellations. No invented correction is used.
2-QAM/BPSK is available as a reference but is not mapped to a data capture code.
The non-square 128/512/2048/8192-QAM geometries use the original tables.


The build also compiles nearest-symbol decision boundaries. Square-QAM cells use
halfway thresholds; cross-QAM includes diagonal corner boundaries. A build-only
Delaunay helper finds neighboring symbols; neither it nor a geometry database
runs when a capture opens. See the [conversion design](../../../../docs/development/qam-resources.md)
for formulas, generated output and validation.
