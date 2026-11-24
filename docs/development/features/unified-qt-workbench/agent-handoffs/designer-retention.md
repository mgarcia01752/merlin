# Designer retention metadata preparation

## UIU-SESSION-01: independent Designer retained metadata preparation

This isolated source-only preparation starts at
`71505d97c0c2bf430942d0a8b3a5e20b2709fe61` on
`feature/qt-retention-designer`. It belongs to
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03) and
[REQ-UIU-07](../../../../requirements.md#req-uiu-07), under the existing
[session lifecycle architecture](../../../../architecture.md#unified-qt-session-and-file-lifecycle).
The lead's pending primary design section is
`design.md#independent-input-and-idle-preview-owner-closure`; it was reviewed
read-only rather than copied into this branch. This preparation does not accept
the task, native exposure, session admission or the aggregate runtime gate.
At preparation time the output-construction-peak decision was pending and
independent. The user subsequently approved retained-owner aggregate accounting
with separate finite temporary-construction/numerical limits.

### Contract and ownership audit

The sole added public operation is
`RxMerDesign::retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint>`.
It follows the existing constructor group. It borrows the current immutable root,
uses the common checked ownership walkers, and maps overflow to
`ErrorCode::ResourceLimit`. It neither clones the retained data nor serializes,
reconstructs, evaluates or changes scientific state. Requested capacities and
cached ownership are counted. The metadata excludes allocator, private tree-node
and reference-count headers; it is not total heap/RSS, output reservation,
construction-peak accounting or independently retained consumer storage.

| Owner | Exhaustive retained children |
| --- | --- |
| `RxMerDesign` | Original `NamedCapture`, `Arc<RecipeInput>`, `Arc<RxMer>`. |
| `RecipeInput` | Version, selected baseline, recipe, original capture index/count. |
| `Baseline::Flat` | Exact `ScenarioKey`, anchor epoch, full requested `Vec<bool>` mask capacity and level. Existing key/grid owner visitors retain all nested source identity. |
| `Baseline::Imported` | The complete saved `Stored` input. |
| `Stored` | Name, original byte length, SHA-256 and base64; every owned string's requested capacity. |
| `RxmerRecipe` | Existing DS owner visitor for random variation, tilt, ripple, notch and edit-buffer requested capacity, including all component fields/variants. |
| Cached `RxMer` | Existing complete DS frame/provenance visitors, including sample-buffer capacity, source metadata strings and original/decoded byte Arcs. |

Every struct is destructured without `..`; the baseline match names all variants
and fields. Fixed fields are visited without counting their inline bytes twice.
The common Arc walker counts complete pointee payload and one owner edge for
each occurrence, with no allocation or deduplication. A normal Designer root has
five shared edges: original bytes, typed record, cached RxMER, and cached
original/decoded provenance. There are no ordered-container entries in this
owner graph. This is conservative current-root reporting, not a deduplicated
session total.

### Changed files and adoption prerequisites

- `crates/pnm-input/src/workbench/designer/admission.rs`: declares a private
  path-local retention child and the one public passive operation. Making the
  visitor a child preserves access to private fields without extra getters or
  visibility changes.
- `crates/pnm-input/src/workbench/designer/retention.rs`: complete root, record
  and baseline visitors plus seven private focused tests.
- `crates/pnm-input/src/workbench.rs`: `Stored` visitor only.
- `crates/pnm-input/tests/designer_retained_footprint.rs`: four public API tests.
- This handoff: source-only evidence and deferred adoption instructions.

Adopt together with DS input/recipe preparation
`91d1e9bedbb88626d8faf6104f3b65366b105f9c` and the lead-owned direct
`pnm-input -> merlin-api` dependency and locks. The DS preparation supplies the
`NamedCapture` and complete `RxmerRecipe`/component visitors. The already accepted
base supplies cached `RxMer`, frame, key and provenance visitors. No private
storage is reconstructed through public getters. The lead owns native Python
method/stub, paired Rust/native reference, canonical documentation and combined
validation. No manifests, locks, shared contract, Qt/native source, scientific
statement, default or serialization statement are changed by this preparation.

### Prepared cases: all UNEXECUTED

Seven owner-private cases under
`workbench::designer::admission::retention::tests`:

1. `flat_valid_mask_requested_spare_capacity_is_retained`
2. `recipe_edit_spare_slots_beyond_initialized_values_are_retained`
3. `original_named_capture_name_capacity_is_counted_without_getter_reconstruction`
4. `imported_stored_strings_count_all_requested_capacities`
5. `cached_materialized_rxmer_counts_sample_spare_capacity`
6. `cached_source_provenance_string_capacities_are_retained`
7. `original_record_cache_and_nested_provenance_edges_are_counted_on_aliases`

Four public cases in `designer_retained_footprint`:

1. `flat_metadata_includes_typed_record_original_and_cached_rxmer_without_changes`
2. `imported_metadata_preserves_embedded_baseline_and_original_source_provenance`
3. `exact_original_whitespace_is_charged_on_each_retained_original_arc_edge`
4. `cloned_design_metadata_survives_original_release_and_keeps_cached_result_independent`

The private tests compare actual requested spare-capacity deltas while preserving
initialized values, original bytes and cached scientific observations. The
imported case starts from an admitted real repository RxMER fixture. The alias
case verifies conservative repeated payload/edge accounting across two roots and
survival after releasing one root. The public cases cover flat/imported inputs,
original recipe/provenance preservation, exact trailing original whitespace on
both byte-Arc occurrences, immutable repeated reads, and retained aliases after
original release. They use existing tracked `designer-flat.json` and
`examples/measurements/single-channel/{modulation_profile,rxmer}.json`; no fixtures
were copied, changed or fabricated. Test capacity increments are explicit test
storage quantities, not scientific limits. Overflow propagation is source-audited
through the existing checked common walkers; no unsafe or impossible allocation
is constructed for an owner overflow test.

After combined adoption, run the existing Cargo wrapper with these filters:

```text
scripts/cargo-local.sh test -p pnm-input --lib workbench::designer::admission::retention::tests --locked
scripts/cargo-local.sh test -p pnm-input --test designer_retained_footprint --locked
```

These commands are prepared only. Compilation, Cargo tests, Clippy, full
regressions, Qt tests and native wheel builds were not executed in this isolated
preparation. Primary adoption must validate the combined owner graph before
reporting any Rust/native or runtime acceptance.

### Executed source checks and limits

The four changed/new Rust files were parsed/formatted, then checked for formatting
idempotence using the existing complete pinned Rust 1.85.1 toolchain's rustfmt
1.8.0-stable (`4eb161250e`, 2025-03-15), with stdin and
`skip_children=true` so no other modules were traversed or written. The executable
was read-only at
`/home/dev01/Projects/merlin/local/toolchains/rust-1.85.1-ylt882zx/bin/rustfmt`.
The incomplete global Rust toolchain was not repaired. Source review inspected
all modified/untracked contents, exact field/variant coverage, include-file
existence, original-source preservation, the explicit allowlist and staged scope.
`git diff --check` passed, and the integration-guide generator's read-only
`--check` reported the guide current. The ignored local source-only receipt is
`docs/development/local/upstream-ui-retention/source-review.json`; the final SHA
and clean state are reported to the lead after save. No Cargo, shared targets,
caches, wheels or scientific execution were created or modified.
