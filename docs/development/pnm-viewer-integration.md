# PNM Viewer integration

The toolkit is a Rust library dependency, not a subprocess or HTTP service.
The CLI is an independent driver for testing the same scientific API.

For local development, the viewer's desktop service can depend on:

```toml
docsis-ofdm-fec = { path = "../../../DOCSIS-OFDM-FEC-Toolkit/crates/ofdm" }
```

This relative path applies from `PNM-Viewer-Toolkit/apps/viewer/Cargo.toml` when
the two repositories are siblings. Production integration should pin an approved
Git revision or published crate version after this repository has one.
No commit/release exists yet; no production dependency has been added.

Keep these responsibilities in the viewer:

1. Dispatch PNM bytes through existing parsers; retain a single owned raw-byte
   buffer per source, original filename, SHA-256, timestamp and format version.
2. Adapt decoded values to `profile::model` types without normalizing grids,
   filling missing MER, changing exclusions or inferring roles from filenames.
3. Group by device/channel/exact grid. Source format versions stay specific to
   each file type; different source types must not share an invented version rule.
4. Use `association::match_time` with explicit user tolerance. Show unmatched,
   ambiguous and approximate associations, including timestamp differences.
5. Run generation/evaluation/simulation on cancellable workers. Keep full arrays
   and candidate/session state in Rust; send bounded viewport data to QML.
6. Preserve original scientific results in exports, alongside policy and source
   metadata. Recalculate loaded result claims and verify captured roles.
7. Keep measured FEC-summary counters optional and separate. Those files have
   no frequency grid and no header capture timestamp; do not manufacture either,
   equate counters with BER or imply simultaneous measurements.

The toolkit exposes `generate_candidate`, `evaluate`, `CandidateProfile::edit_range`,
`verify_captured_roles`, `simulate`, progress callbacks and atomic cancellation.
Read [coding-mode limits](../reference/coding-mode.md) before labeling UI results.
Channel estimates currently provide retained context, not an additional gain
applied to measured RxMER. That avoids double-counting equalized impairments.

The earlier viewer UI/session prototype is preserved privately under
`development/migration/pnm-viewer-prototype/`. It was extracted before restoring
PNM Viewer tracked files. It needs adaptation to the library boundary and native
Qt verification before integration. Its first Qt check exposed CXX-Qt0.7.3's
single-directory requirement for bridge Rust files; place the registration file
in apps/qt/src and delegate implementation into its feature folder.

No Qt prototype is claimed to be working or shipped by this toolkit.
