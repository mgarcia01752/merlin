# Upstream report retained-bound preparation

## UIU-SESSION-01: source-only pre-production report owner

Prepared from `713ec8782ec5fe1d695b6559bc1212adf70be944` on
`feature/qt-retention-report`. This implements the two selected upstream owner
operations in the lead's pending primary design section
`design.md#pre-production-upstream-report-bounds`, reviewed read-only. References:
[UIU-SESSION-01](../../../checklist.md#task-uiu-session-01),
[REQ-UIU-03](../../../../requirements.md#req-uiu-03),
[REQ-UIU-07](../../../../requirements.md#req-uiu-07) and
[session architecture](../../../../architecture.md#unified-qt-session-and-file-lifecycle).
This isolated preparation accepts no runtime, native, Qt or checklist gate.

### Exact public operations and sequence

- `Simulation::batch_report_retained_bound(&self) -> ApiResult<merlin_api::RetainedFootprint>`
- `Simulation::run_with_control_bounded(&mut self, control: &RunControl, limit: RetainedFootprint) -> ApiResult<BatchReport>`

The bound query freezes finite final snapshots and source exports without
claiming a control, consuming source/RNG or starting simulation. It may allocate
bounded temporary snapshot/export storage. The bounded run repeats that frozen
preflight, compares all three caller dimensions, and only then allocates a run ID
and calls the unchanged existing scientific loop. Rejection preserves the
accepted session and control. Canonical source export, exact original bytes/hash
and measured provenance are prepared before the control can be claimed or any
source generated. Source/canonical/provenance text keeps the existing 16 MiB
transport ceiling; no new scientific mode or truncation is introduced.

The legacy `Simulation::run` and `run_with_control` bodies are unchanged.
`simulation/run.rs` has exactly three value-preserving literal-to-constant
substitutions (report format/schema/noise-generator identity), plus their imports.
The report schema, scientific algorithms, defaults and serialized values are
unchanged; the exact substitution reversal reproduces the base file byte-for-byte.
Private snapshot helpers never evaluate Pre-EQ/RxMER again. They only
move/compact owned copies, preserving all accepted values, cached powers/banks,
missing values, field identity and original ordering.

### Retained-product proof

The prebuilt template's existing exhaustive `BatchReport` visitor supplies its
exact fixed-root, configuration/options/plan, map-entry and source-export
quantities. Every snapshot buffer is normalized before measurement. The checked
additional quantities depend on the admitted plan and frozen observation policy:

| Destination | Bound and exact final destination |
| --- | --- |
| Grants/unstarted identities | At most `G = options.grants` structs and `G` IDs, conservatively allowing both buffers. Both final vectors have capacity equal to actual length. Existing grant ceiling remains 256. |
| Completed words | `G * plan.codewords.len()` word structs. Each word retains at most `N + 2I` bytes: full mother decisions, retained information decisions, and original payload prefix. Checked `G * sum(N + 2I)` preserves the existing 16 MiB decision admission and at-most-4096 words/grant; all three final nested vectors and word-owner vectors are exact. |
| Selected observations | The empty prebuilt BTreeMap accounts for every key/trace and ordered entry. `floor(points/taps) * taps` slots conservatively allow samples, QAM points and private role gains for each selected tap. Existing limits remain nine taps/65536 aggregate points. All three actual buffers are exact. |
| Receiver estimates | `F = min(policy frames, planned grant frames, scalar slots/(2*FFT))`; charge `F` Arc slots, full estimate structs, complete optional-bin vectors and `F` shared edges. Existing limits remain eight frames/262144 scalar slots. |
| Soft cells | Independent cell policy, at most 1024 Arc records, each full record plus both at-most-12-bit LLR stages and one shared edge. |
| Soft words | `min(policy words, planned words)` Arc records, at most 32, plus `min(policy LLRs, planned transmitted bits)` scalars, at most 262144, and one shared edge per record. No partial vector is fabricated or silently truncated. |
| Failure text | Separately named `MAX_BATCH_FAILURE_DIAGNOSTIC_BYTES`, 16 MiB, reserves the complete retained diagnostic; final String capacity equals its full length. This is a diagnostic policy, distinct from the numerically equal source/transport ceiling. |

All products/additions use checked common capacity/aggregate walkers or checked
usize arithmetic. Fixed inline children are not charged twice. The final receiver
and soft stores consume Arc records with `try_unwrap`; a surviving alias is cloned
before normalization. Original shared owners are never mutated through aliases.
The final Vec capacities use the Box-slice-to-Vec exact-layout contract, and
String uses Box-str-to-String; there is no allocator/growth-factor assumption.

Diagnostics have a finite owner proof: `PhyError` contains only closed variants
with borrowed static field paths, or the closed `LdpcError` enum. Codec error
payloads contain at most two usize values. The source audit found 301 ASCII field
literal occurrences across the upstream source, maximum 40 bytes; the private
field-parameter helpers receive static owner literals, not caller text. Full
Debug formatting and two maximum-width decimal usize values remain far below
16 MiB on both target pointer widths. No arbitrary input bytes, owned error text
or callback-supplied Display enters this report diagnostic. Retain this audit
when extending either error owner. No diagnostic is truncated or rejected late.

After the unchanged engine returns, all actual complete/cancelled/time-limited/
failed science is moved into exact final capacities and the preflight snapshots
and exports are adopted. This path cannot return a late ResourceLimit. A final
`debug_assert` checks the retained-footprint invariant as a programming audit;
construction/preflight, rather than that assertion, guarantees admitted output.
Temporary engine builders, normalization copies and numerical scratch retain
separate finite execution limits. Metadata excludes allocator/node/refcount
headers and is not heap/RSS or a construction-peak promise. Hosts must still
charge simultaneous accepted source, old/new results and independently retained
copies, reserve all three dimensions, and freeze the matching session/generation.

### Exact owner/file map

| Files | Scope |
| --- | --- |
| `crates/ofdma/src/api/session.rs`, new `api/session/report_retention.rs` | Private registration, exactly two public methods, source preflight, control-before-science ordering and five private cases. |
| `crates/ofdma/src/simulation/mod.rs`, new `simulation/report_retention.rs` | Private module, checked retained-product proof, frozen snapshot/final report construction, nested decoder decisions, debug invariant and three private cases. |
| `configuration/{mod.rs,profile.rs}` | Exhaustive owned geometry/profile/grant buffer normalization only. |
| `measurements/{mod.rs,pre_eq.rs,rxmer.rs}`, `signal_processing/pre_eq.rs` | Exhaustive original/cache/bank/target/policy buffer normalization only, with no replay or recalculation. |
| `simulation/request.rs` | Exhaustive current ExperimentConfig snapshot ownership, including channel taps; no validation/execution statement changed. |
| `instruments/{observer.rs,receiver.rs,soft/store.rs}` | Private future-destination quantities, exact traces/gains/estimate/soft Arc records and three capacity/alias cases. |
| `common/constants.rs`, `simulation/run.rs` | Three private domain constants shared by the existing run and new preflight template, preserving exact prior values. |
| `simulation/control.rs` | Narrow private `cfg(test)` completed-grant cancellation threshold, default unset; no production State field, method, callback or algorithm change. |
| New `crates/ofdma/tests/report_retention.rs` | Thirteen public engine/contract cases. |
| This handoff | Source-only audit, limitations and adoption filters. |

All lower owner normalizers use local standard-library capacity conversions;
configuration/measurement owners do not call an upper simulation helper. No new
trait/framework or additional public operation is introduced. No manifests,
locks, common contract, facade, native, Qt or canonical documents are modified.
The prior Designer/QML branches remain untouched. Lead owns native pairing,
facade/Qt admission, canonical documentation, combined compilation/validation
and actual task acceptance.

### Prepared cases: all UNEXECUTED

Eleven private cases:

- `simulation::report_retention::tests::plan_decision_and_capacity_products_reject_checked_overflow`
- `simulation::report_retention::tests::exact_slice_conversion_removes_spare_slots_preserving_every_value`
- `simulation::report_retention::tests::closed_codec_error_variants_and_full_usize_text_fit_diagnostic_policy`
- `api::session::report_retention::tests::source_text_ceiling_accepts_exact_and_rejects_one_extra_byte`
- `api::session::report_retention::tests::source_export_rejection_leaves_accepted_state_and_control_untouched`
- `api::session::report_retention::tests::synthetic_failed_terminal_fixture_normalizes_without_losing_completed_science`
- `api::session::report_retention::tests::synthetic_time_limit_terminal_fixture_normalizes_without_losing_completed_science`
- `api::session::report_retention::tests::pilot_linear_observed_cancel_preserves_completed_estimate_soft_and_decoder_records`
- `instruments::observer::retained_footprint_tests::report_normalization_preserves_json_and_removes_private_gain_spare_capacity`
- `instruments::receiver::retained_footprint_tests::report_estimate_normalization_copies_surviving_alias_without_altering_observation`
- `instruments::soft::store::retained_footprint_tests::report_soft_normalization_preserves_actual_metrics_and_surviving_shared_records`

Thirteen integration cases under `--test report_retention` cover missing settings,
typed unobserved/exact-capacity output, one-byte/one-entry/one-edge policy
exhaustion, original document/hash plus actual observations, nested synthetic
playback, the second numerology, admitted measured context/provenance,
pre-cancellation, actual runtime partial cancellation, no-transmission grants and
actual observed `frame_pilot_linear` complete-run parity.
Rejected dimensional budgets are exactly one smaller than the reported bound;
each case reuses its untouched control successfully with the exact bound.
Scientific parity compares every serialized scientific/configuration/source/
provenance/observation field against legacy execution, excluding only actual run
identity and measured elapsed time. Hidden cache cases use actual legacy engine
observations, preserve JSON, and retain aliases to prove no original owner changes.
The two synthetic Failed/TimeLimit fixtures seed every source/word/estimate/soft
value from an actual completed observed engine report, then change only terminal
metadata and introduce requested spare capacity. They call the actual private
finalizer and compare the same held science before/after normalization, full
failure text, exact decision capacities and the pre-admitted footprint. These
are terminal-owner representation evidence, not execution of engine failure or
timeout algorithms. No elapsed-time equality between separate runs is required.

The actual observed linear-estimator cancellation case arms a private `cfg(test)`
completed-grant threshold, default unset for every existing test/control. It
cancels synchronously at publication of the first completed grant in a two-grant
job, after estimate/soft stores have returned to the report, with no scheduler or
wall-clock race. The progress guard is released before `cancel` takes the pause
lock. The test verifies Cancelled status, retained estimate/cell/word records,
completed decisions, remaining grants and the bound. Its actual first grant,
receiver estimate and soft metrics match a separately completed legacy run,
excluding only run identity. This seam is absent from the production type/API.

Private overflow/internal-state fault fixtures are explicitly not publicly
admitted scientific inputs. Existing tracked CLI fixtures are read unchanged;
no source captures or generated scientific fixtures were added.

Prepared adoption commands (not executed here):

```text
scripts/cargo-local.sh test -p docsis-ofdma --test report_retention --locked
scripts/cargo-local.sh test -p docsis-ofdma --lib simulation::report_retention::tests --locked
scripts/cargo-local.sh test -p docsis-ofdma --lib api::session::report_retention::tests --locked
scripts/cargo-local.sh test -p docsis-ofdma --lib retained_footprint_tests::report_ --locked
```

Compilation, Cargo, Clippy, all Rust/runtime tests, native wheels, Qt and full
regressions remain UNEXECUTED until primary adoption. No direct/native or runtime
acceptance is claimed. The lead must run all affected owner/native cases and
broader required gates in the serialized primary validation flow.

### Executed source review

The cumulative 18 Rust files are parsed/formatted and checked for idempotence with
the existing pinned Rust 1.85.1 rustfmt 1.8.0-stable (`4eb161250e`, 2025-03-15), using stdin and
`skip_children=true` to avoid other modules. Source review covers every diff and
untracked file, include paths, exhaustive owner/variant fields, preserved legacy
source (with exactly the three approved constant substitutions in `run.rs`),
checked arithmetic, exact constant provenance, scope and staged contents.
Save-time whitespace/guide/clean-SHA checks and source hashes are retained in the
ignored local receipts `docs/development/local/upstream-report-retention/source-review.json`
(initial preparation) and `source-review-followup.json` in that same directory
(this follow-up and cumulative scope).
No Cargo target, cache, wheel or scientific execution was created or modified.

### Exact shared report identity values and cumulative scope

The existing upstream common constants owner now defines three `pub(crate)`
values, with separate domain names and documentation, used by both the legacy
run and preflight template:

| Constant | Preserved value |
| --- | --- |
| `UPSTREAM_BATCH_REPORT_FORMAT` | `merlin.upstream.experiment` |
| `UPSTREAM_BATCH_REPORT_SCHEMA_VERSION` | `1` as `u32` |
| `UPSTREAM_BATCH_REPORT_NOISE_GENERATOR` | `chacha8-standardnormal-v1; rand=0.9.5; rand_chacha=0.9.0; rand_distr=0.5.1` |

No new production public API is added by those private constants or the test
seam. The cumulative `713ec87..HEAD` patch contains exactly 19 paths: the 18 Rust
files in the owner map and this unique handoff. The follow-up save changes only
`api/session/report_retention.rs`, `common/constants.rs`, `simulation/control.rs`,
`simulation/run.rs`, `tests/report_retention.rs` and this handoff. Cumulative
prepared coverage is 11 private + 13 integration cases, all UNEXECUTED. Both
source-only receipts remain local under
`docs/development/local/upstream-report-retention/`; the follow-up receipt records
the exact cumulative paths/hashes and the literal-substitution reversal proof.
