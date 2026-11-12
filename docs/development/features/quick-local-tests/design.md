# DEV-TEST-QUICK-01: Explicit local test subset

Expose `--test-quick` on push, release, gate and local test launcher. Full is
still the default; reject combining quick and skip-test. Keep CI unchanged.
Own exact omitted test identities and reasons in tools/testing/quick-tests.json;
never skip tests merely because their name contains a broad word. Retain four
uniform-order waveform cases (64/16384 QAM at both spacings), two CLI width/profile
cases (24 MHz profile 0, 90 MHz profile 3), validation/codec/reference tests,
doctests, preparation example, analysis-only phase and all existing release
smokes. Omit 21 named expensive sweep cases, not their test binaries wholesale.

Rust assertions and scientific implementations stay unchanged. Python builds a
scoped nextest filter from the reviewed manifest; the launcher reports every
omission and rejects extra filters that could widen the selection. Shell and
Python release adapters forward the mode through retries, validation and atomic
push guards. Verify option conflicts, failing retained tests, exact live inventory,
representative inclusion and default behavior. Run the actual quick subset and
record observed counts/timing; never equate its result with full matrix coverage.

## Validation

Ubuntu VM, four workers, cached Rust 1.85.1 builds: actual
`scripts/test-local.sh all --test-quick` passed in **132.02 seconds**.
Workspace: 340 passed, 21 deliberately omitted; preparation example: 4 passed;
analysis-only: 127 passed; both doctest phases succeeded with zero doctests.
Total: 471 passed. This measures the Rust launcher, not the entire pre-push
gate or fresh compilation. No claim of equivalent omitted-case coverage or a
controlled speedup over an earlier run is made.

Live nextest inventories confirm exactly 361 full versus 340 quick workspace
cases, with the difference equal to all 21 reviewed manifest identities.
52 Git/release/gate tests and 10 launcher/catalog/bootstrap tests pass. A further
plain retry-hint regression passes at patch/minor/major modes. Bash syntax,
ShellCheck, skill validation and whitespace checks pass. Gate command doubles
verify retained static/tooling checks, release build and simulation-smoke
dispatch. Full hosted CI/default selection remain unchanged; no real push or
version bump was performed for this change. Local evidence is under
`docs/development/local/quick-local-tests/` (ignored).
