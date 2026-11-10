> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/plc-completion/`
> Logs, captures and specification images are not published.

# PLC completion validation

Completed NEXT01, NEXT07, NEXT11–15 for the fixed-setting reference scope.
- Rendered/inspected local PHY PDF151–152; independent preamble column vectors
  match both tables. Shared AnnexA mapper/soft inverse reused and tested.
- Fourteen new tests pass: five descriptor/transport, one preamble, four frame/
  physical assembly, two observation and two acquisition integration tests.
- Both spacings recover multiple full frames and segmented acquisition packets
  through actual LDPC, QAM and soft inverse with zero information-bit errors.
- Real cell-interleaver startup markers remain missing in assembled data bins.
- Repetition interval rounds200ms toward earlier whole frames. CRC check strings,
  packet corruption, semantic TLV rejection and callback/receiver error paths pass.
- Full scripts/check-before-push.sh passed, including formatting, warnings-denied
  workspace Clippy/tests, analysis-only tests, helper tests and release CLI smoke.
- Separate analysis-only all-target Clippy passed. Logs retained in this directory.
- plc_channel example ran successfully: both FFTs, two frames each, all three
  received descriptors, zero errors. JSON structure verified with development
  tooling; numerical calculations and comparisons are implemented in Rust.
- Changed Markdown relative links/fences pass. Both updated canonical skills and
  their Codex/Claude discovery links validate.
- No commits/pushes/system installations performed. No external interoperability,
  blind acquisition, live CMTS or time-domain waveform claim.
- Next: WAVE01, separate IFFT/FFT with explicit normalization and bin conventions.
