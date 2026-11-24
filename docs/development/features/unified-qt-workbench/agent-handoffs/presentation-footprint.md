# C5 logical presentation counting prerequisite

## Scope and design

This is a source prerequisite for the frozen
[C5 closure condition](../session-closure.md#c5-existing-presentation-counting-prerequisite),
[REQ-UIU-07](../../../../requirements.md#req-uiu-07) and
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01).
The [approved logical presentation contract](../retention-contract-decision.md#approved-presentation-and-channel-boundaries)
remains authoritative. The helper alone does not close C5 or accept SESSION.

Branch `feature/qt-presentation-footprint` starts from accepted primary
`d09c8cf5f6b736690a535395bf6c051336672bbd` in the existing isolated worktree.
Frozen upstream branches remain at `6b9519d` and `7a0cd0c`.

Only three files are owned: application-private pure
`apps/qt/qml/shared/presentation/LogicalFootprint.js`, its focused
`apps/qt/tests/tst_presentation_footprint.qml` and this handoff. No scientific
API, resource reservation, application ceiling, consumer, bridge, global
registration, canonical record or build change is included.

The explicit operation is `measure(value, limits)`. Limits supplies
`max_utf16_bytes`, `max_entries`, `max_depth` as nonnegative safe integers.
Success returns `ok`, `utf16_bytes`, `array_entries`, `object_entries`, `entries`;
failure returns only `ok:false` and a static error code. The encoding factor is
two bytes per actual UTF-16 code unit, including object names and string values.
Array indices/length are structural, not additional string names. Array slots
and object properties share the supplied entry bound. Root depth is zero and
each child value advances depth once.

The lead explicitly chose conservative logical occurrences: repeated object
references included in one value are counted each time; ancestor cycles reject.
This is not physical allocation deduplication. Shared plot bindings use the
parent witness; the caller measures independently owned derived complements.
Iterative traversal avoids recursion limits. Checked safe-integer accounting
and entry/depth checks bound work without choosing new default limits.

Plain dense arrays and plain/null-prototype data objects are supported, with
finite numbers, strings, booleans and null. Own nonenumerable data properties
are counted as actual retained fields. Accessors, symbol fields/values, sparse
or custom-property arrays, custom prototypes, functions, undefined and nonfinite
numbers reject. Accessor values are never invoked. Inputs are neither changed
nor retained after the call; no cache or reservation is created.

## Focused validation

Executed with the existing read-only Qt 6.8.3 Ubuntu x64 SDK at
`/home/dev01/Projects/raven/local/qt/6.8.3/gcc_64`. The only runtime input was
`apps/qt/tests/tst_presentation_footprint.qml`; no whole QML suite was selected.
The QtTest result is **45 passed, 0 failed, 0 skipped**, comprising 43 functional
row cases plus initialization/cleanup. This verifies:

- Actual UTF-16 ASCII/BMP/surrogate pairs and lone surrogates, object keys,
  exact and one-beyond byte/entry/depth policies, zero limits and safe-integer
  policies without 32-bit truncation.
- Invalid policies, unsupported scalars/prototypes/Qt objects/array shapes,
  nonenumerable/null-prototype data, frozen inputs and uninvoked accessors.
- Ancestor cycles, repeated noncyclic references, iterative 2048-level input,
  oversized array rejection before child traversal and independent calls.
- Actual `TlvTreeGrid.visibleRows` wrappers and `FecCodewordTable.filtered`
  arrays, retaining original record references and conservative explicit bundles.
- Actual `ReceiverCellControls` run/profile/order labels and a concrete Qt
  sequence boundary described below. Fixtures exercise presentation shapes only;
  no scientific execution or aggregate-admission acceptance is claimed.

The runner used `-platform offscreen -style Fusion`,
`QT_QUICK_BACKEND=software`, `QSG_RHI_BACKEND=software`,
`QML_DISABLE_DISK_CACHE=1`, and isolated `XDG_CACHE_HOME`/`XDG_RUNTIME_DIR`
under ignored `docs/development/local/qt-presentation-footprint/` in this
worktree. `qt-final.txt` contains the executed result. The runner artifact is:

```text
bin/qmltestrunner SHA-256
636d7a9a050891bb36d918d2159fd621a87e188761390ce88160b5789e223d0b
```

Targeted `qmllint --max-warnings 0` on the test QML exited zero with an empty
`qmllint-test.txt`; it resolved the helper import. `qmlformat` output exactly
matched the final test candidate. Direct SDK `qmllint` invocation on the `.js`
file itself exited 139 without diagnostics; standalone JS lint is **not passed**.
The successful QML import/runtime and QML lint are the executed checks instead.

An initial selector test assumed `ComboBox.model` remained a plain JS array;
its assertion failed. Actual Qt 6.8.3 readback is a Qt sequence with
`Array.isArray(model) === false`. The final test expects static unsupported
rejection for that object, then counts the actual displayed labels in bounded
plain-array test snapshots. Production must measure its original JS label
candidate before ComboBox conversion and retain that witness; the helper does
not introspect Qt sequence getters or claim their physical capacity.

## Source receipt and integration boundary

Final source SHA-256 values before save:

```text
apps/qt/qml/shared/presentation/LogicalFootprint.js
dca0cffff72c71338441aecd911ab4c1c100e308c8363622f9e00dc8b8decf89
apps/qt/tests/tst_presentation_footprint.qml
7ad3e30326424489ac008f60def10e08fc490d9e2832eee909172c1f5fc8923e
```

Review covered all three owned files, arithmetic guards, traversal/alias behavior,
SPDX scope, static error responses and unchanged input references. Architecture,
requirements and checklist were read for correspondence; their existing C5 and
unchecked SESSION state require no change in this scoped helper unit. Python
integration guide `--check` reported current before save. The helper refresh must
leave it unchanged. Frozen branch tips remain `6b9519d` and `7a0cd0c`.

Disk before the focused rendering tests was 80% used, 24 GiB free, 13% inodes;
after tests it was 79% used, 26 GiB free, 12% inodes. The agent's ignored output
was 36 KiB at that measurement. No cache outside this disposable output was
created, edited or deleted by this unit. No Cargo, native build, wheel, full
regression, installation or publication ran.

Lead integration still owns global registration and the actual presentation
adapter: caller-supplied application ceilings, retained/open-view populations,
candidate admission before persistent adoption, old/candidate overlap, parent
alias witnesses, independently derived complements and final clear/destruction.
Object-key introspection obtains own names before the entry rejection; this
logical helper is not an allocator/scratch-capacity or RSS instrument. Inputs
remain the existing bounded plain-data DTOs. Hidden/closed data must keep its
witness until actual clear/disposal. Synchronous response measurement alone does
not cover later persistence. **C5, SESSION and shell acceptance remain open.**
