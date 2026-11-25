# PyPNM capture test references

Acknowledgment: PyPNM - Proactive Network Maintenance Toolkit.

Originally copied from PyPNM commit
`82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9` on 2026-11-03 with the user's
permission. The binary fixtures are now modified derivatives: every CM and CMTS
MAC is the user-selected example `aa:bb:cc:dd:ee:ff` (`aabb.ccdd.eeff`). All
measurement bytes and file lengths are unchanged. See provenance.json for
original SHA-256 hashes, current hashes and modified identity fields.
Upstream LICENSE and NOTICE are retained alongside these files.

- `tests/files/rxmer.bin`: positive RxMER capture fixture.
- `tests/files/fec_summary.bin`: FEC profile/interval capture.
- `tests/files/channel_estimation.bin`: channel-estimate coefficients.
- `tests/files/const_display.bin`: constellation samples.
- `tests/files/histogram.bin`: dwell/hit counters.
- `tests/files/us_pre_equalizer_coef.bin`: current upstream coefficients.
- `tests/files/us_pre_equalizer_coef_last.bin`: last upstream coefficient update.
- `tests/files/spectrum_analyzer.bin`: file-based spectrum amplitudes.
- `tests/files/modulation_profile.bin`: compact modulation assignments.

Latency has no upstream fixture and uses labeled synthetic Rust tests. Symbol
capture remains a stub. The internal SNMP spectrum fixture is not imported.
- `tests/test_pnm_rxmer_parse.py`: upstream Python test reference, with its
  original relative fixture paths preserved. Not a Rust test or runtime dependency.

The upstream tests check model shape, units, statistics, cached values, frequency
spacing, serialization and rejection of a different file type. Their quarter-dB
and clamping expectations are implementation observations, not normative OSSI
verification. They do not provide independent exact expected values for every
field or establish DOCSIS 3.1/4.0 format equivalence.

Use these bytes for Rust parser tests after cross-referencing field definitions.
Keep sanitized fixtures unchanged during normal tests; construct truncated/
corrupted test cases in memory. Use the explicit
[MAC sanitization tool](../../../tools/security/README.md) when importing or
intentionally changing fixture identities. Rust integration tests must explicitly load these shared fixtures; a
root fixture directory does not create a Cargo test target. User authorization on
2026-11-05 permits the nine sanitized binary captures, upstream license/NOTICE
and provenance in desktop installers. `scripts/release/samples.py` owns that
allowlist; Python test references remain excluded. The Python tests have not been run
in this repository and require PyPNM/pytest if used separately.
