# Example layout

## EXAMPLES-01: distinguish CLI requests from desktop workbenches

Move the eleven top-level CLI request/settings JSON files into `examples/cli/`.
Move the associated sample payload into `examples/cli/payloads/`. Add a README
there with file purpose, use cases and runnable commands; add `examples/README.md`
as the discoverable index. Preserve all moved input bytes exactly.

Keep `examples/workbench/` for UI-ready saved workbenches. Measurement JSON and
binary captures under `examples/measurements/` are shared input/test data; they
are not all CLI requests and remain in their current location with provenance.
Update tracked code, tooling, tests and documentation references using the exact
moved paths. Do not rewrite external/private inputs, captures or provenance hashes.

The shared desktop payload copies only the workbench and measurement example
trees plus the explanatory index, excluding `examples/cli/`. CLI archives retain
the entire public examples tree. Verify both packaging inventories, moved-byte
identity, links, old-path absence and affected Rust/CLI import tests. Rebuild the
latest Qt executable if its fixture references change. Continue UI-MSI-01 with
its existing native Windows evidence boundary after this organization task.

Implementation and checks: [validation](validation.md).
