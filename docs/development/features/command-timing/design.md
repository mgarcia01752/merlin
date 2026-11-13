# Local command timing

## TOOL-TIME-01: test and push elapsed summaries

Print total wall elapsed hours/minutes/seconds at the end of the public local
unit-test runner and Git push helper, with the selected phase/mode and original
exit status. Include setup, compilation, all selected tests/doctests and, for
push, validation/versioning/tagging/network push. Hosted CI time is separate.

Use Bash's existing SECONDS counter and EXIT traps; no new dependency, log file
or Git mutation is needed. Keep timers local to each shell process. Help/list
requests do not claim a test run. Preserve all arguments, checks and failure
statuses. Replace the push wrapper's release-mode exec with a checked child call
and explicit successful exit so the wrapper can emit its final timing line.

The unit runner reports its own selected-phase time even inside a push. The
push wrapper reports the outer command time, including full/quick/skip selections
without changing those selections. Existing per-suite nextest timings remain.

Verify formatting, elapsed time greater than zero, success/failure status,
branch/release delegation, no-op/dry-run, and failed validation preventing pushes
using command doubles and disposable local Git origins. No production commit,
tag or push is performed during validation. Qt and scientific source are unchanged.

## Local validation

- Test-tool helpers: 11 tests passed, including a real delayed child command,
  failure exit preservation and help/list exclusions.
- Git/release helpers: 52 tests passed against disposable local origins,
  including branch failure, patch success/failure and release dry-run timing.
- Bash syntax, ShellCheck, version consistency and diff whitespace checks passed.

These checks validate command timing and Git guards. Scientific Rust and Qt
sources are unchanged; their full suites were not rerun for this tooling change.
Totals apply to normal exits after argument parsing; forced process termination
cannot guarantee a final summary. No historical push duration can be recovered
from these new summaries.
