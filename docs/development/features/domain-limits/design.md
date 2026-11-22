# Domain limit and default ownership

The user requests explicit domain limits and explanations for opaque literals,
with generic reusable guidance in the engineering foundation. This focused
refactor preserves the existing scientific behavior under
[REQ-DS-INPUT-01](../../../requirements.md#req-ds-input-01) and
[REQ-DS-API-01](../../../requirements.md#req-ds-api-01).
[STYLE-DOMAIN-01](../../checklist.md#task-style-domain-01) owns delivery; API-05
work is preserved separately and is not accepted by this task.

## Architecture and reuse

Shared fixed numerical/physical definitions live in
`crates/dsp/src/common/constants.rs`, consumed by independent PHYs. Downstream
shared defaults/edit policies live in `crates/ofdm/src/common/constants.rs`,
which re-exports applicable shared leaf definitions. Both common modules are
stateless; neither PHY depends on the facade or the other PHY.

`crates/ofdm/src/signal_processing/rxmer_designer.rs` remains the numerical owner.
Name the existing accepted synthetic RxMER value domain in dB and reuse it for
flat baseline admission, explicit replacements and final designed results. Keep
independent tilt, ripple, notch and random-variation limits separately named even
when values coincide. Name echo-distance bounds and the existing light-speed
constant with their units. These values are existing application policies or
physical constants, not new DOCSIS/PNM requirements. Imported observations remain
unchanged; this does not apply a synthetic-editor range to all captured data.

The common DSP constants own the 15-bit register domain; the downstream common
constants own the continuous-source all-ones default. Existing public constants
in `crates/ofdm/src/simulation/source/prbs.rs` remain compatible re-exports/aliases. Name the maximum valid seed separately from
the sequence period and derive the default from its intentional all-ones policy.
Reuse that source default in controller, waveform, document and preview owners.
File-mode fallback seed is dormant and documented; file-fill/noise seeds remain
independent. Preserve independent literal test vectors.

| Owner | Decision |
| --- | --- |
| `crates/dsp/src/common/{mod,constants}.rs`, `lib.rs`, `prbs15.rs` | Add the shared register/mask/period and physical/unit definitions; preserve recurrence exactly. |
| `crates/ofdm/src/common/{mod,constants}.rs`, `lib.rs` | Add grouped downstream default/domain policies and re-export shared leaf definitions. |
| `crates/ofdma/src/source/{prbs15,verifier}.rs` | Reuse the shared maximum valid seed, preserving independent algorithms/errors. |
| `crates/ofdm/src/signal_processing/{waveform/timing,receiver/diagnostics}.rs` | Reuse the same physical/unit constants without changing operation order. |
| `crates/ofdm/src/signal_processing/rxmer_designer.rs` | Edit named local policies/physical units; expose the shared synthetic-value bounds without changing calculations. |
| `crates/pnm-input/src/workbench/designer.rs` | Reuse the real synthetic-value domain; no anonymous signed range. |
| `crates/ofdm/src/simulation/source/prbs.rs` | Edit named seed maximum/default, preserving public period and recurrence. |
| `crates/ofdm/src/api/{controller,units,transmitter,full_loop/request}.rs`, `simulation/channel/{config,document}.rs` | Reuse authoritative source domain/default; preserve every value and constructor signature. |
| `skills/coding-style/SKILL.md` | Add explicit range, opaque literal/comment and authority guidance. |
| `crates/ofdm/tests/{rxmer_designer,prbs_source}.rs`, existing Designer admission test | Reuse independent numerical/bounds/default tests; no new tests that merely mirror renamed constants. |

## Requirements and acceptance

Preserve exact values, finite checks, inclusive endpoints, errors, source state,
serialization, public imports and default behavior. Limits require a domain owner,
units, origin/rationale and a stated sharing relationship. A one-off opaque value
may use an adjacent precise explanation; repeated shared meanings use one named
owner. Numerical equality alone never establishes shared purpose.

Run focused existing Designer/PRBS and native saved-Designer admission cases,
formatting and warnings-denied checks for the touched owners. Validate skill/docs
links and traceability. Record actual results below before checking the task.
Final integration acceptance under [UIU-QA-01](../../checklist.md#task-uiu-qa-01)
requires a line-by-line audit of changed project-owned production functions,
defaults, examples and tests: remove unexplained magic values, centralize true
common definitions and document independent equal values and numerical anchors.
That final audit remains pending; this focused task does not accept it.

Qt behavior/source, full historical regressions, releases and hosted jobs are
outside this naming refactor. Foundation edits remain a separate working tree.

## Validation

[Exact implementation/log evidence](evidence.json) records twenty passing focused
existing Rust cases: two shared PRBS, five downstream PRBS, six Designer, two
waveform timing, two receiver diagnostics, two upstream PRBS and one saved
Designer admission. Warnings-denied Clippy passes for the four touched core/input
owners and all targets. Formatting and the analysis-only no-default build pass.
Current-mode feature documentation passes. Independent numerical vectors and all
178 Qt source hashes remain intact. No tests were added merely to repeat constants.

The generic foundation link/skill/portability checks and all 31 existing tooling
tests pass. All 908 unrelated foundation files and its exact index are preserved;
foundation edits remain uncommitted. No external checkout or product-specific
numeric policy is required by its new portable procedure.

The complete integration line-by-line numeric audit remains pending UIU-QA-01;
this focused acceptance does not claim that every repository literal was reviewed.
