# DOCSIS OFDM FEC Toolkit

DOFT is short for DOCSIS OFDM FEC Toolkit. It is a standalone DOCSIS scientific
library and CLI. Work exclusively in this repository unless scope is changed.

Focus on downstream OFDM first. OFDMA is deferred. Two scientific modes are the
ideal/reference experiment and measurement-driven analysis using RxMER, downstream
channel estimates and an applied modulation profile. Support is developed against
pinned DOCSIS3.1/4.0 evidence up to16384-QAM; do not equate a recognized order with
an implemented full coding chain or assume an optional modem capability.

A future optional Qt component is presentation only. Build scientific functionality
first; there is no UI implementation or Python binding in the current scope.
Dependencies point from CLI/future adapters to docsis-ofdm-fec to fec-codec.

Apply skills/coding-style, skills/scientific-testing, and the applicable versioned
PHY skill. Apply skills/public-api to public models and operations and
skills/docsis-signal-processing to scientific changes. Keep canonical skills under skills/ with relative discovery links
under .agents/skills/ and .claude/skills/.

- All signal processing, channel models, modulation/demapping, FEC and scientific
  orchestration are 100% Rust, including dependencies providing those algorithms.
  Do not add C/C++ DSP/codec implementations, native FFI or Python calculations.
  Keep scientific crates free of unsafe code. Development tooling may use Python.
- Reusable code and tools are Apache-2.0. Preserve copied copyrights and third
  party licenses, including the Rust-adapted MIT BCH implementation and ldpc-toolbox.
- Follow meaningful domain aliases, descriptive variables/lifetimes and multiline
  public API documentation. New implementations belong in focused modules.
- Preserve exact source grids, nulls, excluded carriers, hashes and timestamps.
  Applications retain original capture bytes; decoded input contracts preserve provenance.
- Keep measured observations, analytical screening and simulated FEC outcomes
  distinct. Do not claim a DOCSIS receiver from a generic coding experiment.
- Record pinned normative evidence. Never infer missing shortening, interleaving,
  scrambling or waveform behavior from generic DVB support.
- Bound files, trials, iterations, runtime and memory. Preserve completed FEC
  trials on cancellation. Never overwrite user inputs or export destinations.
- Keep private PDFs and source captures under ignored development/. Do not ship
  migration prototypes, private evidence, Cargo caches or build outputs.
- Plan implementation under development/<feature>/ and maintain public docs in
  docs/. Update the architecture when ownership or input contracts change.
- No SNMP/TFTP, device capture/control, network analysis or Python sidecars.
- No commits, pushes, publication or system-wide installation without explicit
  authorization. Supply commands when asked for commit/push commands.

Validation:

```bash
scripts/cargo-local.sh fmt --all -- --check
scripts/cargo-local.sh test --workspace --locked
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
scripts/cargo-local.sh test -p docsis-ofdm-fec --no-default-features --locked
```

The initial toolkit version is 0.1.0. Source packages and VERSION must agree.
No native installer or cross-platform runtime is validated yet.

## Git and releases

Apply skills/git-workflow and skills/toolkit-release for these operations.
Use tools/git/git-save.sh --commit-msg and tools/git/git-push.sh. Inspect all
tracked/staged/untracked changes before writing a message. Format Feature:,
BugFix: or Documentation:, followed by - detail lines and - Tests:, at most60
characters per message line. Supply commands unless asked to execute.
Begin Git command blocks with cd /home/dev01/Projects/DOCSIS-OFDM-FEC-Toolkit &&.

VERSION is canonical. tools/bump-version.sh synchronizes every local package and
Cargo.lock. Combined --tag-patch/--tag-minor/--tag-major modes require clean master,
validate the bumped candidate, commit version files, annotate and atomically push.
No validation bypass or Git force-push. --force permits a fresh release only
from an already published clean HEAD. --tag retries without another bump.
No automatic GitHub publication or installer workflow is configured yet.

## Source module ownership

Keep fec-codec split into parameters, BCH, LDPC and concatenated codec modules.
The concatenated path must be usable without modulation, noise or capture models.
In ofdm, scenario owns observations/grids/association; profile owns hypotheses
and evaluation; signal_processing owns numerical algorithms; simulation owns
source generation, job orchestration and results. Keep CLI main focused on
startup, with arguments, commands and bounded JSON I/O in separate modules.
Preserve existing public imports with re-exports rather than duplicate logic.
See docs/architecture.md. PRBS is a later source implementation, not a label for
the existing seeded random generator.
