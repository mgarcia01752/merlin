# Embedded QAM resources

Unmodified PyPNM tables copied through PNM Viewer Toolkit. See `provenance.json`
for source revisions and SHA-256 hashes. Retain LICENSE and NOTICE with these
resources. Acknowledge: **PyPNM - Proactive Network Maintenance Toolkit**.

For data orders16..16384, row index is the MSB-first cell-word label; columns are
integer I/Q coordinates. Raw BPSK/QPSK rows are geometric references; the build
uses explicit index permutations `[1,0]` and `[1,3,0,2]` to match PHY I20
Figures99–100. Do not sort rows or infer control labels from raw row indices.
Scaling factors specify mean
integer-coordinate symbol energy; divide both coordinates by its square root.

The Rust build helper validates cardinality, uniqueness and mean energy before
emitting static normalized references into Cargo's generated output directory.
Mapping and soft demapping perform no runtime text parsing or filesystem access.
No database or native code is needed.

Resources cover BPSK/QPSK control references and downstream data orders16,64,128,
256,512,1024,2048,4096,8192,16384. Non-square orders use their original coordinates.
The preserved scaling-factor file includes other orders;8-QAM/32-QAM mapping is
intentionally unavailable for downstream use. No unsupported mapping is guessed.

See [mapping conventions and validation](../../../../docs/user/qam-mapping.md).
