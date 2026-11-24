# Accepted upstream resource-grid fixture

`ResourceGridFixture.js` embeds the unchanged JSON payload from the existing
separate Rust-process `upstream_resource_grid` example. The small JavaScript
wrapper only lets QtTest import local data without file/network I/O. No cell,
region, count, identity, capacity, gain, bit offset or sample timing was edited,
filtered or recalculated. These are configured-resource observations, not emitted
waveform, received I/Q or external interoperability evidence.

The reviewed payload is 196867 bytes, two snapshots and 600 total cells:

| Snapshot | Reported spacing | Original carrier window | Original frame-symbol window | Cells | Frame/grant coded bits |
| --- | --- | --- | --- | --- | --- |
| 0 | 50000 Hz | first 99, count 34 | first 0, count 6 | 204 | 328 / 656 |
| 1 | 25000 Hz | first 199, count 66 | first 0, count 6 | 396 | 712 / 1424 |

Both retain relative frame index 1, absolute frame identity 12, original grant
symbol origin 6 and nonzero sample origins. All six reported roles are present:
data, complementary, known pilot, zero loaded, excluded and unallocated.
`transmission_planned` is false for snapshot 0 and true for snapshot 1; the viewer
must retain that distinction rather than inferring it from visible allocations.

Payload SHA-256:
`653f27fc675a9c97e7673c6bbf422e9bd94e28aada82e9fc3b90bed8ac3b7ab6`.
The payload starts immediately after `var snapshots = ` in the wrapper and
includes the original trailing newline. The tests import the public DTO shape;
they do not reconstruct resources from the numerology or pilot pattern.

The payload was copied read-only from
`/home/dev01/Projects/merlin/docs/development/local/unified-qt-workbench/api-05-project-native-full/resource_grid.stdout`.
It is byte-identical to the accepted API-03 final source at
`/home/dev01/Projects/merlin/docs/development/local/unified-qt-workbench/api-03-resource-grid-native-final/resource_grid.stdout`.
The latter folder's `results.json` has status `passed` and records this producer
command, working directory `/home/dev01/Projects/merlin`, exit 0:

```bash
bash /home/dev01/Projects/merlin/scripts/cargo-local.sh run \
  -p docsis-ofdma --example upstream_resource_grid --locked
```

That command was **not rerun** for this presentation unit. The API-05 copy's
producer also exited 0, but that folder's overall report is `failed`; its other
gates are not cited as passing evidence. Existing API-03 acceptance remains
[historical backend evidence](../../../../../docs/development/features/unified-qt-workbench/validation.md#uiu-api-03-idle-upstream-resource-grid-accepted),
separate from these new Qt checks.

The [producer](../../../../../crates/ofdma/examples/upstream_resource_grid.rs),
[public snapshot](../../../../../crates/ofdma/src/api/resources.rs) and
[region/cell owner](../../../../../crates/ofdma/src/resources/view.rs) were
reviewed at presentation base `9d9783bddfd7393caa6c4351bee6aed28dcfdeeb`.
The underlying Rust owners remain Apache-2.0; the new MERLIN QtTest wrapper and
presentation code are GPL-3.0-only. This synthetic public example contains no
private capture bytes or device metadata.
