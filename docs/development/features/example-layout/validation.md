# Example layout validation

All eleven moved JSON inputs and the sample message retain their original SHA-256
hashes. Measurement captures/provenance are untouched. No old references to the
moved `examples/FILENAME.json` or `examples/payloads/` paths remain in tracked
source, commands or documentation. Dynamic CLI fixture roots were updated too.

- All 20 packaging tests pass: desktop examples exclude `cli/`, while the CLI
  package retains `examples/cli/awgn.json` and complete workbench resources.
- All 23 standalone CLI tests and 6 focused Rust document/modulation/public-path
  tests pass against the moved examples.
- Seven SDK/development-tool tests pass. The CLI's native smoke exercises the
  moved AWGN request with output/no-clobber checks.
- The latest Linux Qt executable is rebuilt; the focused native input-file test
  checks the moved sample payload. Previous MSI validation includes 75 adapter
  Rust tests, 68 QML checks and Qt Clippy/formatting.
- Example-index, CLI README and shared-measurement README links resolve. The CLI
  README lists each request/settings file, its use case, command and sample output
  behavior. Rust formatting, workflow checks and release skill validation pass.
- Current MSI authoring against real public data/notices excludes the CLI folder
  and passes local structural WiX validation. Full Windows MSI compilation and
  installed lifecycle remain pending native CI, as recorded in the MSI feature.

See [design](design.md), [CLI README](../../../../examples/cli/README.md) and
[MSI validation](../windows-desktop/validation.md).
